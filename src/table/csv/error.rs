use std::io;

use crate::table::display_formatted::ALL_BOOL_STRINGS;

#[cfg(feature = "python")]
use pyo3::PyErr;

// PartialEq and Eq is here due to checks in tests
#[derive(Debug, thiserror::Error)]
pub enum TruthTableFromCsvError {
    #[error("Encountered the same variable multiple times: {name}")]
    DuplicateVariableName { name: String },
    #[error("Reached EOF before reading first row.")]
    UnexpectedEof,
    #[error("Expected row with index {row_index} to contain {expected_row_len} cells, found {actual_row_len} cells")]
    RecordDifferentSizeThanHeader {
        row_index: usize,
        expected_row_len: usize,
        actual_row_len: usize,
    },
    #[error("Invalid cell value, found '{actual}', expected one of '{}'", ALL_BOOL_STRINGS.join(", "))]
    NonBooleanCellValue { actual: String },
    #[error("Couldn't get last column of boolean function outputs.")]
    NoOutputColumn,
    #[error("Expected table with {variable_count} variables to contain {} rows, found {actual_row_count} rows", 2_usize.pow(*variable_count as u32))]
    MismatchedRecordCountAndVariableCount {
        variable_count: usize,
        actual_row_count: usize,
    },
    #[error("Found no delimiter, expected one of the following characters: .,`|\\t")]
    NoDelimiterFound,
    #[error(transparent)]
    ParsingError(#[from] csv::Error),
    #[error(transparent)]
    IOError(#[from] io::Error),
}

#[cfg(feature = "python")]
impl From<TruthTableFromCsvError> for PyErr {
    fn from(err: TruthTableFromCsvError) -> PyErr {
        use pyo3::exceptions::{PyEOFError, PyIOError, PyRuntimeError, PyTypeError};
        use TruthTableFromCsvError::*;

        match err {
            e @ UnexpectedEof => PyEOFError::new_err(e.to_string()),
            e @ NonBooleanCellValue { .. } => PyTypeError::new_err(e.to_string()),
            e @ DuplicateVariableName { .. }
            | e @ RecordDifferentSizeThanHeader { .. }
            | e @ NoOutputColumn
            | e @ MismatchedRecordCountAndVariableCount { .. }
            | e @ NoDelimiterFound => PyRuntimeError::new_err(e.to_string()),
            ParsingError(e) => PyRuntimeError::new_err(e.to_string()),
            IOError(e) => PyIOError::new_err(e),
        }
    }
}
