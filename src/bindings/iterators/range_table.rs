use crate::table::iterators::ImageIterator;
use pyo3::prelude::*;

#[pyclass(name = "TableRangeIterator")]
pub struct PythonTableRangeIterator {
    iter: ImageIterator,
}

#[pymethods]
impl PythonTableRangeIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<bool> {
        slf.iter.next()
    }
}

impl From<ImageIterator> for PythonTableRangeIterator {
    fn from(value: ImageIterator) -> Self {
        Self { iter: value }
    }
}
