mod error;
mod expression;

use crate::bindings::expression::PythonExpression;
use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn biodivine_boolean_functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PythonExpression>()?;
    Ok(())
}
