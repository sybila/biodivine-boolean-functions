use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::{PyAny, PyErr};

#[derive(Debug, thiserror::Error)]
pub enum PythonExpressionError<'a> {
    #[error("Expected a string to parse from, got {} instead", if let Ok(name) =.value.get_type().qualname() {name} else { "UnknownType".to_string() })]
    UnexpectedConstructorArgument { value: &'a PyAny },
    #[error("Encountered unknown variable while evaluating: {name}")]
    UnknownVariableWhileEvaluating { name: String },
    #[error("")]
    UnexpectedTypeOfArgument,
}

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
            UnknownVariableWhileEvaluating { .. } => PyValueError::new_err(err.to_string()),
        }
    }
}
