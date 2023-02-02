/// Builder for create worker instance.

use std::{io, thread};
use std::time::Duration;
use std::sync::Arc;

use tokio::runtime::{Builder as RuntimeBuilder};
use tokio::sync::oneshot;

use crate::worker::Worker;
use crate::dropper::Dropper;

pub struct Builder {
    /// The id of this worker
    id: String,

    /// worker builder
    builder: RuntimeBuilder,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            id: "default-worker".to_string(),
            builder: RuntimeBuilder::new_multi_thread(),
        }
    }
}

impl Builder {
    /// Sets the number of worker threads the Runtime will use.
    ///
    /// This can be any number above 0. The default value is the number of cores available to the system.
    pub fn worker_threads(&mut self, val: usize) -> &mut Self {
        self.builder.worker_threads(val);
        self
    }

    /// Specifies the limit for additional threads spawned by the Runtime.
    ///
    /// These threads are used for blocking operations like tasks spawned through spawn_blocking,
    /// they are not always active and will exit if left idle for too long, You can change this timeout duration
    /// with thread_keep_alive. The default value is 512.
    pub fn max_blocking_threads(&mut self, val: usize) -> &mut Self {
        self.builder.max_blocking_threads(val);
        self
    }

    /// Sets a custom timeout for a thread in the blocking pool.
    ///
    /// By default, the timeout for a thread is set to 10 seconds.
    pub fn thread_keep_alive(&mut self, duration: Duration) -> &mut Self {
        self.builder.thread_keep_alive(duration);
        self
    }

    /// Sets name of threads spawned by the Runtime thread pool
    pub fn thread_name(&mut self, val: impl Into<String>) -> &mut Self {
        self.id = val.into();
        self
    }

    pub fn build(&mut self) -> Result<Worker, io::Error> {
        let worker = self
            .builder
            .enable_all()
            .thread_name(self.id.clone())
            .build()?;

        let handle = worker.handle().clone();
        let (send_stop, recv_stop) = oneshot::channel();
        // Block the runtime to shutdown.
        let _ = thread::Builder::new()
            .name(format!("{}-blocker", self.id))
            .spawn(move || worker.block_on(recv_stop));

        Ok(Worker {
            handle,
            _dropper: Arc::new(Dropper {
                close: Some(send_stop),
            }),
        })
    }
}

