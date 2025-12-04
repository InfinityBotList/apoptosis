use mlua_scheduler::LuaSchedulerAsyncUserData;
use mluau::prelude::*;
use serde::{Serialize, de::DeserializeOwned};
use std::rc::Rc;
use tokio::task::spawn_local;
use tokio::{
    runtime::LocalOptions,
    select,
    sync::{
        mpsc::{UnboundedReceiver, UnboundedSender},
        oneshot::{Receiver as OneshotReceiver, Sender as OneshotSender, channel},
    },
};
use tokio_util::sync::CancellationToken;

use crate::service::lua::{
    FilesystemWrapper, OnBrokenFunc, OnDropFunc, SpawnResult, Vm, VmCreateOpts,
};
use crate::service::optional_value::OptionalValue;

type DispatchLayerResult = Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>>;

pub struct LayerData<T: IntoLua + 'static> {
    value: LuaValue,
    marker: std::marker::PhantomData<T>,
}

impl<T: IntoLua + 'static> LayerData<T> {
    pub fn new(layer_data: T, lua: &Lua) -> LuaResult<Self> {
        let value = layer_data.into_lua(lua)?;
        Ok(Self {
            value,
            marker: std::marker::PhantomData,
        })
    }
}

/// A layer provides a specific service within Omniplex/IBL
#[allow(dead_code)]
pub trait Layer: Sized + 'static {
    type Message: Serialize + DeserializeOwned + Send + 'static;
    type LayerData: IntoLua + 'static;

    /// Returns the layer name
    fn name() -> &'static str;

    /// Creates a new layer
    async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>>;

    /// Dispatches a message to the layer
    async fn dispatch(&self, msg: Self::Message) -> DispatchLayerResult;

    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    // Pre-provided helpers

    /// Set up the VM for this layer
    ///
    /// Can be called within new() etc
    async fn setup_vm(
        opts: VmCreateOpts,
        fsw: FilesystemWrapper,
        on_broken: Option<OnBrokenFunc>,
        on_drop: Option<OnDropFunc>,
    ) -> Result<Vm, Box<dyn std::error::Error + Send + Sync>> {
        let mut vm = Vm::new(opts, fsw)
            .await
            .map_err(|e| format!("Failed to create VM for layer {}: {}", Self::name(), e))?;

        vm.sandbox()
            .map_err(|e| format!("Failed to sandbox VM for layer {}: {}", Self::name(), e))?;

        if let Some(on_broken) = on_broken {
            vm.set_on_broken(on_broken);
        } else {
            vm.set_on_broken(Box::new(|| {
                log::error!("VM for layer {} has broken", Self::name());
            }));
        }

        if let Some(on_drop) = on_drop {
            vm.set_on_close(on_drop);
        } else {
            vm.set_on_close(Box::new(|| {
                log::info!("VM for layer {} has been dropped", Self::name());
            }));
        }

        Ok(vm)
    }

    /// Dispatches a message to a VM at the given path
    async fn dispatch_to_vm(
        vm: &Vm,
        path: &str,
        layer_data: LayerData<Self::LayerData>,
        msg: Self::Message,
    ) -> LuaResult<SpawnResult> {
        let ctx: Context<Self> = Context::new(layer_data, msg);
        vm.run(path, ctx).await
    }

    /// Same as dispatch_to_vm but with path as init.luau and return as a DispatchLayerResult
    async fn dispatch_to_vm_simple(
        vm: &Vm,
        layer_data: LayerData<Self::LayerData>,
        msg: Self::Message,
    ) -> DispatchLayerResult {
        let res = Self::dispatch_to_vm(vm, "init.luau", layer_data, msg)
            .await
            .map_err(|e| format!("{e}"))?;

        let value = res
            .into_must_value(vm)
            .map_err(|e| format!("Failed to convert VM result into value: {e}"))?;

        Ok(value)
    }
}

/// A LayerThread provides a dedicated thread for a specific IBL apoptosis layer
#[allow(dead_code)]
#[derive(Clone)]
pub struct LayerThread<L: Layer> {
    tx: UnboundedSender<(L::Message, OneshotSender<DispatchLayerResult>)>,
    cancellation_token: CancellationToken,
}

#[allow(dead_code)]
impl<L: Layer> LayerThread<L> {
    /// Creates a new VmThread
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let cancellation_token = CancellationToken::new();
        let ct_clone = cancellation_token.clone();

        std::thread::Builder::new()
            .name(format!("LayerThread-{}", std::any::type_name::<L>()))
            .spawn(move || {
                Self::thread(ct_clone, rx);
            })
            .expect("Failed to spawn VM thread");

        Self {
            tx,
            cancellation_token,
        }
    }

    /// thread function
    fn thread(
        cancellation_token: CancellationToken,
        mut rx: UnboundedReceiver<(L::Message, OneshotSender<DispatchLayerResult>)>,
    ) {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build_local(LocalOptions::default())
            .unwrap();

        rt.block_on(async move {
            let layer = Rc::new(L::new().await.expect("Failed to create layer"));

            loop {
                select! {
                    Some(msg) = rx.recv() => {
                        let layer_ref = layer.clone();
                        spawn_local(async move {
                            let (msg, tx) = msg;
                            let result = layer_ref.dispatch(msg).await;
                            let _ = tx.send(result);
                        });
                    }
                    _ = cancellation_token.cancelled() => {
                        match layer.cleanup().await {
                            Ok(_) => {},
                            Err(e) => log::error!("Error during layer cleanup: {e}"),
                        }
                        return;
                    }
                }
            }
        });
    }

    async fn dispatch(&self, msg: L::Message) -> DispatchLayerResult {
        let (tx, rx): (
            OneshotSender<DispatchLayerResult>,
            OneshotReceiver<DispatchLayerResult>,
        ) = channel();

        self.tx
            .send((msg, tx))
            .map_err(|e| format!("Failed to send message to layer thread: {e}"))?;

        match rx.await {
            Ok(result) => result,
            Err(e) => Err(format!("Failed to receive response from layer thread: {e}").into()),
        }
    }

    fn cancel(&self) {
        self.cancellation_token.cancel();
    }
}

impl<L: Layer> LuaUserData for LayerThread<L> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_scheduler_async_method("Dispatch", |lua, this, msg: LuaValue| async move {
            let msg: L::Message = lua
                .from_value(msg)
                .map_err(|e| LuaError::external(format!("Failed to deserialize message: {e}")))?;

            let result = this
                .dispatch(msg)
                .await
                .map_err(|e| LuaError::external(format!("Layer dispatch error: {e}")))?;

            let lua_result = lua
                .to_value(&result)
                .map_err(|e| LuaError::external(format!("Failed to serialize result: {e}")))?;

            Ok(lua_result)
        });
    }
}

/// A context for an event
pub struct Context<L: Layer> {
    layer_data: LayerData<L::LayerData>,
    event: OptionalValue<L::Message>,
    event_cache: OptionalValue<LuaValue>,
}

impl<L: Layer> Context<L> {
    /// Creates a new context
    pub fn new(layer_data: LayerData<L::LayerData>, event: L::Message) -> Self {
        Self {
            layer_data,
            event: OptionalValue::with(event),
            event_cache: OptionalValue::new(),
        }
    }
}

impl<L: Layer> LuaUserData for Context<L> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("layer", |_lua, this| Ok(this.layer_data.value.clone()));

        fields.add_field_method_get("event", |lua, this| {
            this.event_cache.get_failable(|| {
                let event = this
                    .event
                    .take()
                    .ok_or("Event should be set")
                    .map_err(LuaError::external)?;
                let value = lua.to_value(&event)?;
                Ok(value)
            })
        });
    }
}
