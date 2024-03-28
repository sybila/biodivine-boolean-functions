use pyo3::exceptions::{PyKeyError, PyTypeError};
use pyo3::prelude::{PyAnyMethods, PyTypeMethods};
use pyo3::{Bound, PyAny, PyErr};

#[derive(Debug, thiserror::Error)]
pub enum PythonExpressionError<'py> {
    #[error("Expected a string to parse from, got {} instead", if let Ok(name) =.value.get_type().qualname() {name} else { "UnknownType".to_string() })]
    UnexpectedConstructorArgument { value: Bound<'py, PyAny> },
    #[error("Encountered unknown variable while evaluating: {name}")]
    UnknownVariableWhileEvaluating { name: String },
    #[error("")]
    UnexpectedTypeOfArgument,
}

// TODO: maybe full Eq with checking pointer equality for
impl<'a> PartialEq for PythonExpressionError<'a> {
    fn eq(&self, other: &Self) -> bool {
        use std::mem::discriminant;

        discriminant(self) == discriminant(other)
    }
}

impl<'a> From<PythonExpressionError<'a>> for PyErr {
    fn from(err: PythonExpressionError) -> PyErr {
        use PythonExpressionError::*;

        match err {
            UnexpectedConstructorArgument { .. } | UnexpectedTypeOfArgument { .. } => {
                PyTypeError::new_err(err.to_string())
            }
            UnknownVariableWhileEvaluating { .. } => PyKeyError::new_err(err.to_string()),
        }
    }
}
