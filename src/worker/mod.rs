//! Worker library.
pub mod func;
use crate::task::Task;
use crate::transmit::Server;

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
    /// Worker server.
    pub server: Server,
}
