//! Task define.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Task {
    /// id
    pub id: String,
    /// The python code of this task, pickled
    #[serde(with = "serde_bytes")]
    pub code: Vec<u8>,
    /// Function name to call
    pub func_name: String,
}

impl Task {
    /// New task.
    pub fn new(id: String, code: Vec<u8>, func_name: String) -> Self {
        Self {
            id,
            code,
            func_name,
        }
    }
}
