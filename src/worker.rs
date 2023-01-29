//! Worker is a server that receives tasks from the scheduler and executes them.

use std::future::Future;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::task::{JoinError, JoinHandle};
use tokio::runtime::{Handle};

use crate::dropper::Dropper;
use crate::task::Task;

/// A worker to run tasks
#[derive(Clone, Debug)]
pub struct Worker {
    /// Tokio handle
    pub(crate) handle: Handle,

    // Used to receive a drop signal when dropper is dropped, inspired by databend
    pub(crate) _dropper: Arc<Dropper>,
}


impl Worker {
    /// Spawn a future and execute it in this thread pool
    ///
    /// Similar to tokio::runtime::Runtime::spawn()
    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
        where
            F: Future + Send + 'static,
            F::Output: Send + 'static,
    {
        self.handle.spawn(future)
    }

    /// Run the provided function on an executor dedicated to blocking
    /// operations.
    pub fn spawn_blocking<F, R>(&self, func: F) -> JoinHandle<R>
        where
            F: FnOnce() -> R + Send + 'static,
            R: Send + 'static,
    {
        self.handle.spawn_blocking(func)
    }

    /// Run a future to complete, this is the runtime entry point
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.handle.block_on(future)
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::Builder;

    #[test]
    fn test_spawn() {
        let worker = Builder::default()
            .worker_threads(3)
            .thread_name("1")
            .build();

        assert!(worker.is_ok())
    }
}