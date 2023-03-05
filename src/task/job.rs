//! Job.
use crate::task::{Back, Task};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Job struct.Job is a task list.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Job {
    /// The job id.
    pub id: Uuid,
    /// The job tasks.
    pub tasks: Vec<Task>,
    /// The back.
    pub backs: Vec<Back>,
}

impl Job {
    /// New job.
    pub fn new(id: Uuid, tasks: Vec<Task>, backs: Vec<Back>) -> Self {
        Self { id, tasks, backs }
    }
}
