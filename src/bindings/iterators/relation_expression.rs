use crate::iterators::RelationIterator;
use pyo3::prelude::*;

#[pyclass(name = "ExpressionRelationIterator")]
pub struct PythonExpressionRelationIterator {
    iter: RelationIterator<String>,
}

#[pymethods]
impl PythonExpressionRelationIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(Vec<bool>, bool)> {
        slf.iter.next()
    }
}

impl From<RelationIterator<String>> for PythonExpressionRelationIterator {
    fn from(value: RelationIterator<String>) -> Self {
        Self { iter: value }
    }
}
