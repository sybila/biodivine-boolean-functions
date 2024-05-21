use crate::iterators::SupportIterator;
use pyo3::prelude::*;

#[pyclass(name = "ExpressionSupportIterator")]
pub struct PythonExpressionSupportIterator {
    iter: SupportIterator<String>,
}

#[pymethods]
impl PythonExpressionSupportIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Vec<bool>> {
        slf.iter.next()
    }
}

impl From<SupportIterator<String>> for PythonExpressionSupportIterator {
    fn from(value: SupportIterator<String>) -> Self {
        Self { iter: value }
    }
}
