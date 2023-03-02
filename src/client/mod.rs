//! Python client support.
//! Worker library.
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyString, PyTuple};

/// Submit a task to the scheduler and return the task id.
#[pyfunction]
pub fn remote(func: &PyBytes, address: &PyString, args: &PyTuple, kwargs: &PyDict) -> PyResult<u8> {
    println!("remote func");

    Python::with_gil(|py| {
        let pickle_module = PyModule::import(py, "cloudpickle").unwrap();
        let pickle_code = pickle_module.getattr("loads").unwrap();
        let pure_code = pickle_code.call1((func,)).unwrap();
        println!("pure_code: {pure_code:?}");
        println!("args: {args:?}");
        println!("kwargs: {kwargs:?}");

        let result = if args.is_empty() {
            pure_code.call((), Some(kwargs)).unwrap()
        } else {
            pure_code.call(args, Some(kwargs)).unwrap()
        };

        println!("result: {result:?}");
    });

    Ok(0)
}

/// Submit a task to the scheduler and wait for the result.
#[pyfunction]
pub fn remote_sync(func: &PyBytes, args: &PyTuple, kwargs: &PyDict) -> PyResult<u8> {
    println!("remote func");
    println!("func: {func:?}");
    println!("args: {args:?}");
    println!("kwargs: {kwargs:?}");

    Ok(0)
}
