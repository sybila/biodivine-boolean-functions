use crate::table::iterators::RelationIterator;
use pyo3::prelude::*;

#[pyclass(name = "TableRelationIterator")]
pub struct PythonTableRelationIterator {
    iter: RelationIterator,
}

#[pymethods]
impl PythonTableRelationIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(Vec<bool>, bool)> {
        slf.iter.next()
    }
}

impl From<RelationIterator> for PythonTableRelationIterator {
    fn from(value: RelationIterator) -> Self {
        Self { iter: value }
    }
}
