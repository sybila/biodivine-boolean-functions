use pyo3::prelude::{pyclass, pymethods, PyAny, PyAnyMethods, PyResult};
use pyo3::types::PyDict;
use pyo3::Bound;
use std::collections::{HashMap, HashSet};

use crate::bindings::error::PythonExpressionError;
use crate::bindings::error::PythonExpressionError::UnknownVariableWhileEvaluating;
use crate::expressions::Expression as RustExpression;
use crate::traits::{Evaluate, GatherLiterals, Parse, SemanticEq};

#[pyclass(frozen)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PythonExpression {
    root: RustExpression<String>,
}

#[pymethods]
impl PythonExpression {
    #[new]
    fn py_new(value: Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(expression) = value.extract::<Self>() {
            return Ok(expression);
        }
        if let Ok(value) = value.extract::<String>() {
            return match RustExpression::from_string(value) {
                Ok(expression) => Ok(Self::new(expression)),
                Err(parse_error) => Err(parse_error.into()),
            };
        }

        Err(PythonExpressionError::UnexpectedConstructorArgument { value }.into())
    }

    pub fn to_nnf(&self) -> Self {
        Self::new(self.root.to_nnf())
    }

    pub fn to_cnf(&self) -> Self {
        Self::new(self.root.to_cnf())
    }

    pub fn is_cnf(&self) -> bool {
        self.root.is_cnf()
    }

    /// Throws a `KeyError` when a variable is encountered that isn't found among
    /// the given `literal_values`.
    pub fn evaluate_nonsafe(&self, literal_values: Bound<'_, PyDict>) -> PyResult<bool> {
        let hashmap: HashMap<String, bool> = literal_values.extract()?;

        Ok(self
            .root
            .evaluate_with_err(&hashmap)
            .map_err(|name| UnknownVariableWhileEvaluating { name })?)
    }

    /// Variables not in the dictionary default to false.
    pub fn evaluate_safe(&self, literal_values: Bound<'_, PyDict>) -> PyResult<bool> {
        let hashmap: HashMap<String, bool> = literal_values.extract()?;

        Ok(self.root.evaluate(&hashmap))
    }

    pub fn gather_literals(&self) -> HashSet<String> {
        self.root.gather_literals()
    }

    pub fn semantic_eq(&self, other: &PythonExpression) -> bool {
        self.root.semantic_eq(&other.root)
    }

    /// Returns True if the expression is a literal value or a negation of a literal value.
    pub fn is_literal(&self) -> bool {
        self.root.is_literal()
    }

    pub fn is_constant(&self) -> bool {
        self.root.is_constant()
    }

    pub fn is_not(&self) -> bool {
        self.root.is_not()
    }

    pub fn is_and(&self) -> bool {
        self.root.is_and()
    }

    pub fn is_or(&self) -> bool {
        self.root.is_or()
    }

    // TODO maybe allow numeric booleans?
    #[staticmethod]
    pub fn mk_constant(value: bool) -> PythonExpression {
        Self::new(RustExpression::Constant(value))
    }

    #[staticmethod]
    pub fn mk_literal(name: String) -> PythonExpression {
        Self::new(RustExpression::Literal(name))
    }

    #[staticmethod]
    pub fn mk_not(expression: &PythonExpression) -> PythonExpression {
        Self::new(RustExpression::negate(expression.root.clone()))
    }

    #[staticmethod]
    pub fn mk_and_binary(left: &PythonExpression, right: &PythonExpression) -> PythonExpression {
        Self::new(RustExpression::binary_and(
            left.root.clone(),
            right.root.clone(),
        ))
    }

    #[staticmethod]
    pub fn mk_and_n_ary(expressions: Vec<PythonExpression>) -> PythonExpression {
        Self::new(RustExpression::n_ary_and(
            expressions.into_iter().map(|e| e.root).collect(),
        ))
    }

    #[staticmethod]
    pub fn mk_or_binary(left: &PythonExpression, right: &PythonExpression) -> PythonExpression {
        Self::new(RustExpression::binary_or(
            left.root.clone(),
            right.root.clone(),
        ))
    }

    #[staticmethod]
    pub fn mk_or_n_ary(expressions: Vec<PythonExpression>) -> PythonExpression {
        Self::new(RustExpression::n_ary_or(
            expressions.into_iter().map(|e| e.root).collect(),
        ))
    }

    pub fn __str__(&self) -> String {
        self.root.to_string()
    }

    pub fn __repr__(&self) -> String {
        format!("PythonExpression({})", self.__str__())
    }
}

impl PythonExpression {
    fn new(expression: RustExpression<String>) -> Self {
        PythonExpression { root: expression }
    }
}
