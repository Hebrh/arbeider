/// Task is python code that is executed by the worker
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyTuple};

/// Func struct define
pub struct Func {
    /// The python code of this task, pickled
    pub code: PyBytes,
    /// Function name to call
    pub func_name: String,
    /// The python code of this task
    pub args: PyTuple,
    /// The python code of this task
    pub kwargs: PyDict,
    /// Requires module. First is the module name, second is the version
    pub requires: Vec<(String, String)>,
}

impl Func {
    /// New func
    pub fn new(
        code: PyBytes,
        func_name: String,
        args: PyTuple,
        kwargs: PyDict,
        requires: Vec<(String, String)>,
    ) -> Self {
        Self {
            code,
            func_name,
            args,
            kwargs,
            requires,
        }
    }

    /// Execute the func
    pub fn run(&self) -> PyResult<()> {
        // Python::with_gil(|py| {
        // let key1 = "key1";
        // let val1 = 1;
        // let key2 = "key2";
        // let val2 = 2;
        //
        //
        // let name = self.func_name.clone();
        //
        // let func_name = PyString::new(py, &name);
        //
        // let func: Py<PyAny> = PyModule::from_code(py, &self.code, "", "")
        //     .unwrap()
        //     .getattr(func_name)
        //     .unwrap()
        //     .into();
        //
        // let args_value = self.args.value.clone();
        //
        // let args = PyTuple::new(py, args_value);
        // let kwargs = self.kwargs.clone().value;
        //
        // let kwargs_dict = PyDict::new(py);
        // for item in kwargs.iter() {
        //     kwargs_dict
        //         .set_item(item.clone().0, item.clone().1)
        //         .expect("Failed to set_item on dict");
        // }
        //
        // func.call(py, args, Some(kwargs_dict)).unwrap();
        // });

        Ok(())
    }
}
