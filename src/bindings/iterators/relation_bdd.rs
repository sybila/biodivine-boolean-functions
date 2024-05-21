use crate::bdd::iterators::ImageIterator;
use crate::iterators::DomainIterator;
use pyo3::prelude::*;
use std::iter::Zip;

#[pyclass(name = "BddRelationIterator")]
pub struct PythonBddRelationIterator {
    iter: Zip<DomainIterator, ImageIterator>,
}

#[pymethods]
impl PythonBddRelationIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(Vec<bool>, bool)> {
        slf.iter.next()
    }
}

impl From<Zip<DomainIterator, ImageIterator>> for PythonBddRelationIterator {
    fn from(value: Zip<DomainIterator, ImageIterator>) -> Self {
        Self { iter: value }
    }
}
