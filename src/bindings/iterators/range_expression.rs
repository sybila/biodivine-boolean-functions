use crate::iterators::ImageIterator;
use pyo3::prelude::*;

#[pyclass(name = "ExpressionRangeIterator")]
pub struct PythonExpressionRangeIterator {
    iter: ImageIterator<String>,
}

#[pymethods]
impl PythonExpressionRangeIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<bool> {
        slf.iter.next()
    }
}

impl From<ImageIterator<String>> for PythonExpressionRangeIterator {
    fn from(value: ImageIterator<String>) -> Self {
        Self { iter: value }
    }
}
