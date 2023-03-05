//! Task return back.
use crate::define::Task;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Task return back.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Back {
    /// The task id.
    pub id: Uuid,
    /// The task.
    pub task: Option<Task>,
    /// The task result. json object format
    pub result: Option<Value>,
}
