// PartialEq and Eq is here due to checks in tests
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TokenizeError {
    #[error("Unexpected `)` encountered on position {position} near '{vicinity}'")]
    UnexpectedClosingParenthesis { position: usize, vicinity: String },
    #[error("Missing `)` on position {position} near '{vicinity}'")]
    MissingClosingParenthesis { position: usize, vicinity: String },
    #[error("Unexpected `}}` encountered on position {position} near '{vicinity}'")]
    UnexpectedClosingCurlyBrace { position: usize, vicinity: String },
    #[error("Missing `}}` on position {position} near '{vicinity}'")]
    MissingClosingCurlyBrace { position: usize, vicinity: String },
    #[error("No name literal `{{}}` encountered on position {position} near '{vicinity}'")]
    EmptyLiteralName { position: usize, vicinity: String },
    #[error("Unknown symbol {symbol} encountered on position {position}'")]
    UnknownSymbolError { position: usize, symbol: String },
    #[error("Unexpected whitespace encountered in the middle of operator")]
    UnexpectedWhitespace,
}

// TODO extend enum variants to carry position of error, code, impl position from errors
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ParseTokensError {
    #[error("Operator had nothing as its operand")]
    EmptySideOfOperator,
    #[error("Unexpected multiple consecutive literals, maybe you are missing an operator?")]
    UnexpectedLiteralsGroup,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseError {
    #[error(transparent)]
    TokenizingError(#[from] TokenizeError),
    #[error(transparent)]
    ParsingError(#[from] ParseTokensError),
}

pub const EOL_VICINITY: &str = "EOL";

#[cfg(feature = "python")]
mod bindings {
    use super::ParseError;
    use pyo3::exceptions::PyRuntimeError;
    use pyo3::PyErr;

    impl From<ParseError> for PyErr {
        fn from(err: ParseError) -> PyErr {
            PyRuntimeError::new_err(err.to_string())
        }
    }
}
