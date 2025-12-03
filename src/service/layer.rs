use std::rc::Rc;

use tokio::task::spawn_local;
use tokio::{
    runtime::LocalOptions,
    select,
    sync::{
        mpsc::{UnboundedReceiver, UnboundedSender},
        oneshot::{Receiver as OneshotReceiver, Sender as OneshotSender},
    },
};
use tokio_util::sync::CancellationToken;

type DispatchLayerResult = Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>>;

pub trait Layer: Sized + 'static {
    type Message: serde::Serialize + serde::de::DeserializeOwned + Send + 'static;

    async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>>;

    async fn dispatch(&self, msg: Self::Message) -> DispatchLayerResult;

    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// A LayerThread provides a dedicated thread for a specific IBL apoptosis layer
pub struct LayerThread<L: Layer> {
    thread_handle: std::thread::JoinHandle<()>,
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

        let thread_handle = std::thread::Builder::new()
            .name(format!("LayerThread-{}", std::any::type_name::<L>()))
            .spawn(move || {
                Self::thread(ct_clone, rx);
            })
            .expect("Failed to spawn VM thread");

        Self {
            thread_handle,
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
        ) = tokio::sync::oneshot::channel();

        self.tx
            .send((msg, tx))
            .map_err(|e| format!("Failed to send message to layer thread: {e}"))?;

        match rx.await {
            Ok(result) => result,
            Err(e) => Err(format!("Failed to receive response from layer thread: {e}").into()),
        }
    }
}
