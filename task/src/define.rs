//! Task struct define.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Task {
    /// id
    pub id: Uuid,
    /// The python code of this task, pickled
    #[serde(with = "serde_bytes")]
    pub code: Vec<u8>,
    /// Function name to call
    pub func_name: String,
}

impl Task {
    /// New task.
    pub fn new(id: Uuid, code: Vec<u8>, func_name: String) -> Self {
        Self {
            id,
            code,
            func_name,
        }
    }
}
