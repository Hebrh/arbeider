//! Worker library.
pub mod func;
use crate::task::Task;

/// Worker struct.
pub struct Worker {
    /// The worker id.
    pub id: u8,
    /// The worker name.
    pub name: String,
    /// The worker address.
    pub address: String,
    /// The worker port.
    pub port: u16,
    /// The worker tasks.
    pub tasks: Vec<Task>,
}
