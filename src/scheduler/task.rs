//! Task list management.

use std::sync::Arc;

pub struct Task {
    /// id
    pub id: String,
    /// The python code of this task, pickled
    pub code: Arc<Vec<u8>>,
    /// Function name to call
    pub func_name: String,
}
