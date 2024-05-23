use crate::bdd::iterators::SupportIterator;
use pyo3::prelude::*;

#[pyclass(name = "BddSupportIterator")]
pub struct PythonBddSupportIterator {
    iter: SupportIterator,
}

#[pymethods]
impl PythonBddSupportIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Vec<bool>> {
        slf.iter.next()
    }
}

impl From<SupportIterator> for PythonBddSupportIterator {
    fn from(value: SupportIterator) -> Self {
        Self { iter: value }
    }
}
