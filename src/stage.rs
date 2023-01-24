//! Stage to start worker.

use pyo3::prelude::*;

#[pyclass]
pub struct Stage {
    pub name: String,
}

#[pymethods]
impl Stage {
    #[new]
    pub fn new() -> Self {
        Stage { name: "stage default".to_string() }
    }

    pub fn start(&self) -> PyResult<()> {
        println!("Hello, worker is starting!");
        Ok(())
    }
}