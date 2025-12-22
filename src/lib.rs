use pyo3::prelude::*;
mod gf2_matrix;
use gf2_matrix::PyGF2Matrix;

#[pymodule]
fn gf2_lin_algebra(_py: Python<'_>, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<PyGF2Matrix>()?;
    Ok(())
}