//! Stage to start worker.

use pyo3::prelude::*;

use crate::builder::Builder;
use crate::worker::Worker;

#[pyclass]
pub struct Stage {
    pub name: String,
    pub worker: Worker,
}

#[pymethods]
impl Stage {
    #[new]
    pub fn new(name: &str) -> Self {
        Stage {
            name: name.to_string(),
            worker: init_worker()
        }
    }
}

impl Default for Stage {
    fn default() -> Self {
        Stage {
            name: "stage default".to_string(),
            worker: init_worker()
        }
    }
}

/// Initialize the stage
/// Init rust executor runtime and start the worker
fn init_worker() -> Worker {
    let worker = Builder::default()
        .worker_threads(3)
        .thread_name("1")
        .build().unwrap();
    worker
}
