use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use pyo3::PyResult;

use crate::bindings::error::PythonExpressionError::UnknownVariableWhileEvaluating;
use crate::bindings::expression::PythonExpression;
use crate::expressions::Expression as RustExpression;
use crate::table::display_formatted::{TableBooleanFormatting, TableStyle};
use crate::table::TruthTable;
use crate::traits::{Evaluate, GatherLiterals, SemanticEq};

#[pyo3::pyclass(frozen)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PythonTruthTable {
    root: Arc<TruthTable<String>>,
}

#[pyo3::pymethods]
impl PythonTruthTable {
    #[staticmethod]
    pub fn from_expression(expression: &PythonExpression) -> Self {
        let rust_expression: RustExpression<String> = expression.into();
        let rust_table: TruthTable<String> = rust_expression.into();

        Self::new(rust_table)
    }

    pub fn to_expression_trivial(&self) -> PythonExpression {
        self.root.to_expression_trivial().into()
    }

    pub fn to_string_formatted(
        &self,
        style: TableStyle,
        boolean_formatting: TableBooleanFormatting,
    ) -> String {
        self.root
            .to_string_formatted(style, boolean_formatting, boolean_formatting)
    }

    pub fn gather_literals(&self) -> BTreeSet<String> {
        self.root.gather_literals()
    }

    /// Throws a `KeyError` when a variable is encountered that isn't found among
    /// the given `literal_values`.
    pub fn evaluate_checked(&self, literal_values: BTreeMap<String, bool>) -> PyResult<bool> {
        Ok(self
            .root
            .evaluate_checked(&literal_values)
            .map_err(|name| UnknownVariableWhileEvaluating { name })?)
    }

    /// Variables not in the dictionary default to false.
    pub fn evaluate_safe(&self, literal_values: BTreeMap<String, bool>) -> PyResult<bool> {
        Ok(self.root.evaluate(&literal_values))
    }

    /// Variables not in the dictionary defaults to the passed `default_value` argument.
    pub fn evaluate_with_default(
        &self,
        literal_values: BTreeMap<String, bool>,
        default_value: bool,
    ) -> PyResult<bool> {
        Ok(self
            .root
            .evaluate_with_default(&literal_values, default_value))
    }

    pub fn semantic_eq(&self, other: &Self) -> bool {
        self.root.semantic_eq(&other.root)
    }

    pub fn row(&self, row_index: usize) -> Vec<bool> {
        self.root.row(row_index)
    }

    #[cfg(feature = "csv")]
    #[staticmethod]
    pub fn from_csv_file(path: &str) -> PyResult<Self> {
        Ok(Self::new(TruthTable::from_csv_file(path)?))
    }

    #[cfg(feature = "csv")]
    #[staticmethod]
    pub fn from_csv_string(path: &str) -> PyResult<Self> {
        Ok(Self::new(TruthTable::from_csv_string(path)?))
    }

    #[cfg(feature = "csv")]
    pub fn to_csv(&self) -> String {
        self.root.to_csv()
    }

    pub fn __str__(&self) -> String {
        self.root.to_string()
    }

    pub fn __repr__(&self) -> String {
        format!("PythonTruthTable({})", self.__str__())
    }
}

impl PythonTruthTable {
    fn new(table: TruthTable<String>) -> Self {
        PythonTruthTable {
            root: Arc::new(table),
        }
    }
}
