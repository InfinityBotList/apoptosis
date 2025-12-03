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

type DispatchLayerResult = Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>>;

/// A layer provides a specific service within Omniplex/IBL
pub trait Layer: Sized + 'static {
    type Message: Serialize + DeserializeOwned + Send + 'static;

    async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>>;

    async fn dispatch(&self, msg: Self::Message) -> DispatchLayerResult;

    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
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
}

impl<L: Layer> Drop for LayerThread<L> {
    fn drop(&mut self) {
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
