use crate::bdd::iterators::ImageIterator;
use pyo3::prelude::*;

#[pyclass(name = "BddRangeIterator")]
pub struct PythonBddRangeIterator {
    iter: ImageIterator,
}

#[pymethods]
impl PythonBddRangeIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<bool> {
        slf.iter.next()
    }
}

impl From<ImageIterator> for PythonBddRangeIterator {
    fn from(value: ImageIterator) -> Self {
        Self { iter: value }
    }
}
