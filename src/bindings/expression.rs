use std::collections::{HashMap, HashSet};

use pyo3::prelude::{pyclass, pymethods, PyAny, PyAnyMethods, PyResult};
use pyo3::Bound;

use crate::bindings::error::PythonExpressionError;
use crate::bindings::error::PythonExpressionError::UnknownVariableWhileEvaluating;
use crate::expressions::{Expression as RustExpression, ExpressionNode};
use crate::traits::{Evaluate, GatherLiterals, Parse, SemanticEq};

#[pyclass(frozen)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PythonExpression {
    root: RustExpression<String>,
}

impl From<RustExpression<String>> for PythonExpression {
    fn from(value: RustExpression<String>) -> Self {
        PythonExpression::new(value)
    }
}

impl From<PythonExpression> for RustExpression<String> {
    fn from(value: PythonExpression) -> Self {
        (&value).into()
    }
}

impl From<&PythonExpression> for RustExpression<String> {
    fn from(value: &PythonExpression) -> Self {
        // We can safely clone here because this only increases
        // the reference count of the root expression.
        value.root.clone()
    }
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
    pub fn evaluate_checked(&self, literal_values: HashMap<String, bool>) -> PyResult<bool> {
        Ok(self
            .root
            .evaluate_checked(&literal_values)
            .map_err(|name| UnknownVariableWhileEvaluating { name })?)
    }

    /// Variables not in the dictionary default to false.
    pub fn evaluate(&self, literal_values: HashMap<String, bool>) -> PyResult<bool> {
        Ok(self.root.evaluate(&literal_values))
    }

    /// Variables not in the dictionary defaults to the passed `default_value` argument.
    pub fn evaluate_with_default(
        &self,
        literal_values: HashMap<String, bool>,
        default_value: bool,
    ) -> PyResult<bool> {
        Ok(self
            .root
            .evaluate_with_default(&literal_values, default_value))
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
        Self::new(ExpressionNode::Constant(value).into())
    }

    #[staticmethod]
    pub fn mk_literal(name: String) -> PythonExpression {
        Self::new(ExpressionNode::Literal(name).into())
    }

    #[staticmethod]
    pub fn mk_not(expression: &PythonExpression) -> PythonExpression {
        Self::new(RustExpression::negate(&expression.root))
    }

    #[staticmethod]
    pub fn mk_and_binary(left: &PythonExpression, right: &PythonExpression) -> PythonExpression {
        Self::new(RustExpression::binary_and(&left.root, &right.root))
    }

    #[staticmethod]
    pub fn mk_and_n_ary(expressions: Vec<PythonExpression>) -> PythonExpression {
        Self::new(RustExpression::n_ary_and(&Vec::from_iter(
            expressions.into_iter().map(Into::into),
        )))
    }

    #[staticmethod]
    pub fn mk_or_binary(left: &PythonExpression, right: &PythonExpression) -> PythonExpression {
        Self::new(RustExpression::binary_or(&left.root, &right.root))
    }

    #[staticmethod]
    pub fn mk_or_n_ary(expressions: Vec<PythonExpression>) -> PythonExpression {
        Self::new(RustExpression::n_ary_or(&Vec::from_iter(
            expressions.into_iter().map(Into::into),
        )))
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
