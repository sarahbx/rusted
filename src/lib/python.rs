use pyo3::prelude::{pymodule, pyclass, pymethods, pyfunction, wrap_pyfunction, PyModuleMethods};
use pyo3::{Bound, PyResult, Python};
use pyo3::types::PyModule;


#[pymodule]
fn rusted(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Rusted>()?;
    m.add_function(wrap_pyfunction!(run_python_from_rust, m)?)?;
    Ok(())
}

#[pyclass]
struct Rusted();

#[pymethods]
impl Rusted {
    #[new]
    fn new() -> Self {
        Self()
    }

    #[staticmethod]
    fn hello_world_rust_test() -> () {
        println!("RUST: Hello world\n");
    }
}

#[pyfunction]
fn run_python_from_rust() -> PyResult<()> {
    Python::with_gil(|py| {
        Python::run_bound(py, "from rusted import hello_world; hello_world.hello_world_python_test()", None, None)
    })
}