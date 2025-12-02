use mluau::prelude::LuaError;
use tokio::{
    select,
    sync::{
        mpsc::{UnboundedReceiver, UnboundedSender},
        oneshot::{Receiver as OneshotReceiver, Sender as OneshotSender},
    },
};
use tokio_util::sync::CancellationToken;

use crate::service::lua::vm::Error;

use super::vm::{Vm, VmCreateOpts};

enum VmMessage {
    MemoryUsage {
        tx: OneshotSender<usize>,
    },
    SetMemoryLimit {
        limit: usize,
        tx: OneshotSender<Result<usize, Error>>,
    },
}

/// VM provides a dedicated Luau VM reference for a specific IBL apoptosis layer with each VM occupying
/// a dedicated thread
///
/// Ideally, every IBL apoptosis layer will have its own Vm instance
pub struct VmThread {
    thread_handle: std::thread::JoinHandle<()>,
    tx: UnboundedSender<VmMessage>,
    cancellation_token: CancellationToken,
    srv_name: String,
}

#[allow(dead_code)]
impl VmThread {
    /// Creates a new VmThread
    pub fn new(srv_name: String, opts: VmCreateOpts) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let cancellation_token = CancellationToken::new();
        let ct_clone = cancellation_token.clone();
        let srv_name_clone = srv_name.clone();

        let thread_handle = std::thread::Builder::new()
            .name(format!("VmThread-{}", srv_name.clone()))
            .spawn(move || {
                Self::vm_thread(srv_name_clone, ct_clone, rx);
            })
            .expect("Failed to spawn VM thread");

        Self {
            thread_handle,
            tx,
            cancellation_token,
            srv_name,
        }
    }

    /// VM thread function
    fn vm_thread(
        srv_name: String,
        cancellation_token: CancellationToken,
        mut rx: UnboundedReceiver<VmMessage>,
    ) {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async move {
            let vm = Vm::new(VmCreateOpts::default())
                .await
                .expect("Failed to create VM");

            loop {
                select! {
                    Some(msg) = rx.recv() => {
                        match msg {
                            VmMessage::MemoryUsage { tx } => {
                                let usage = vm.memory_usage();
                                let _ = tx.send(usage);
                            }
                            VmMessage::SetMemoryLimit { limit, tx } => {
                                let result = vm.set_memory_limit(limit);
                                let _ = tx.send(result.map_err(|e| e.to_string().into()));
                            }
                        }
                    }
                    _ = cancellation_token.cancelled() => {
                        match vm.close() {
                            Ok(_) => {},
                            Err(e) => log::error!("Error closing VM: {e:?}"),
                        }
                        return;
                    }
                }
            }
        });
    }
}
