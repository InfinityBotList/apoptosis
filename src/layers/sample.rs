use std::rc::Rc;

use serde::{Deserialize, Serialize};
use crate::service::{layer::{DispatchLayerResult, Layer, LayerConfig, LayerData, NewLayerOpts}, lua::{Vm, RuntimeCreateOpts}, sharedlayer::SharedLayer, vfs::get_luau_vfs};
use mluau::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct SampleLayerConfig {
    pub foo: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum SampleLayerEvent {
    Startup {},
    TestEvent { data: String },
}

impl Default for SampleLayerEvent {
    fn default() -> Self {
        SampleLayerEvent::Startup {}
    }
}

#[derive(Clone)]
pub struct SharedLayerData {
    cfg: LayerConfig<SampleLayer>,
    shared: SharedLayer,
}

/// This is a sample layer implementation that can be used as a reference for implementing Omniplex layers
///
/// Layers may be used for any services for the Omniplex bot list such as html sanitization, cache server/presence
/// handling etc.
#[derive(Clone)]
pub struct SampleLayer {
    vm: Rc<Vm>,
    layer_data: LayerData<Self>,
}

impl Layer for SampleLayer {
    type Message = SampleLayerEvent;
    type LayerData = SharedLayerData;
    type Config = SampleLayerConfig;

    fn name() -> &'static str {
        "samplelayer"
    }

    async fn new(opts: NewLayerOpts<Self>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let shared = SharedLayer::new(opts.pool);
        let vm = Self::setup_vm(RuntimeCreateOpts::default(), get_luau_vfs(), None).await?;

        let layer_data = Self::create_layer_data(SharedLayerData {
            cfg: LayerConfig::new(opts.config.clone()),
            shared: shared.clone(),
        }, &vm)
        .map_err(|e| format!("Failed to create layer data: {e}"))?;

        Ok(Self {
            layer_data,
            vm: Rc::new(vm),
        })
    }

    async fn dispatch(&self, msg: Self::Message) -> DispatchLayerResult {
        Self::dispatch_to_vm_serde(&self.vm, self.layer_data.clone(), msg, "./samplelayer").await
    }
}

impl LuaUserData for SharedLayerData {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("sharedlayer", |lua, this| {
            this.shared.as_lua_userdata(lua)
        });

        fields.add_field_method_get("config", |lua, this| {
            this.cfg.to_lua_value(lua)
        });
    }
}