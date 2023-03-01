/// Worker shutdown controller.
use tokio::sync::oneshot;

/// Dropping the dropper will cause runtime to shutdown.
#[derive(Debug)]
pub struct Dropper {
    pub(crate) close: Option<oneshot::Sender<()>>,
}

impl Drop for Dropper {
    fn drop(&mut self) {
        // Send a signal to say i am dropping.
        self.close.take().map(|v| v.send(()));
    }
}