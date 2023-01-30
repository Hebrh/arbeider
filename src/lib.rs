//! Worker Library.
pub mod stage;
pub mod builder;
mod dropper;
pub mod worker;
pub mod task;
pub mod indicator;
pub mod mock;

use stage::Stage;

// pyO3 module
use pyo3::prelude::*;

#[pymodule]
pub fn internal(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // the pymodule class to make the rustPyFunctions available
    m.add_class::<Stage>()?;
    Ok(())
}
