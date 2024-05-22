mod bdd;
mod error;
mod expression;
mod iterators;
mod table;

use crate::bindings::bdd::PythonBdd;
use crate::bindings::expression::PythonExpression;
use crate::bindings::table::PythonTruthTable;
use crate::table::display_formatted::{TableBooleanFormatting, TableStyle};
use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn biodivine_boolean_functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PythonExpression>()?;
    m.add_class::<PythonTruthTable>()?;
    m.add_class::<PythonBdd>()?;

    m.add_class::<TableStyle>()?;
    m.add_class::<TableBooleanFormatting>()?;

    m.add_function(wrap_pyfunction!(crate::bindings::expression::var, m)?)?;
    m.add_function(wrap_pyfunction!(crate::bindings::expression::vars, m)?)?;
    m.add_function(wrap_pyfunction!(crate::bindings::expression::bool, m)?)?;
    Ok(())
}
