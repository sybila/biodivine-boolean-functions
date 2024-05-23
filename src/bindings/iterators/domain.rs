use crate::iterators::DomainIterator;
use pyo3::prelude::*;

#[pyclass(name = "DomainIterator")]
pub struct PythonDomainIterator {
    iter: DomainIterator,
}

#[pymethods]
impl PythonDomainIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Vec<bool>> {
        slf.iter.next()
    }
}

impl From<DomainIterator> for PythonDomainIterator {
    fn from(value: DomainIterator) -> Self {
        Self { iter: value }
    }
}
