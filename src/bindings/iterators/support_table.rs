use crate::table::iterators::SupportIterator;
use pyo3::prelude::*;

#[pyclass(name = "TableSupportIterator")]
pub struct PythonTableSupportIterator {
    iter: SupportIterator,
}

#[pymethods]
impl PythonTableSupportIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Vec<bool>> {
        slf.iter.next()
    }
}

impl From<SupportIterator> for PythonTableSupportIterator {
    fn from(value: SupportIterator) -> Self {
        Self { iter: value }
    }
}
