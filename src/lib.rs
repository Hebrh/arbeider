//! Worker Library.
mod stage;

use stage::Stage;

// pyO3 module
use pyo3::prelude::*;

#[pymodule]
pub fn _internal(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // the pymodule class to make the rustPyFunctions available
    m.add_class::<Stage>()?;
    Ok(())
}
