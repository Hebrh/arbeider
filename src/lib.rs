//! Worker Library.
pub mod cal;
pub mod indicator;
pub mod mock;
pub mod scheduler;
pub mod worker;

use worker::remote_func;

// pyO3 module
use pyo3::prelude::*;

#[pymodule]
pub fn internal(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // the pymodule class to make the rustPyFunctions available
    m.add_function(wrap_pyfunction!(remote_func, m)?)?;

    Ok(())
}
