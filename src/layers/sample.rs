use serde::{Deserialize, Serialize};
use crate::layer;

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

layer! {
    /// This is a sample layer implementation that can be used as a reference for implementing Omniplex layers
    ///
    /// Layers may be used for any services for the Omniplex bot list such as html sanitization, cache server/presence
    /// handling etc.
    SampleLayer = ( samplelayer, "samplelayer", SampleLayerEvent, SampleLayerConfig, "./samplelayer" )
}