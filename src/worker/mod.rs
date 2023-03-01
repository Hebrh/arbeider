//! Worker library.
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyTuple};

pub mod builder;
mod dropper;
pub mod executor;
pub mod func;

#[pyfunction]
pub fn remote_func(func: &PyBytes, args: &PyTuple, kwargs: &PyDict) -> PyResult<String> {
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

    Ok("success".to_string())
}
