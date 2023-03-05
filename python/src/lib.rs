//! Worker Library.

mod client;

use crate::client::{remote, remote_sync};

// pyO3 module
use pyo3::prelude::*;

#[pymodule]
pub fn internal(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // the pymodule class to make the rustPyFunctions available
    m.add_function(wrap_pyfunction!(remote, m)?)?;
    m.add_function(wrap_pyfunction!(remote_sync, m)?)?;

    Ok(())
}
