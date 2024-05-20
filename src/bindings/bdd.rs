use num_bigint::BigUint;
use pyo3::exceptions::PyRuntimeError;
use pyo3::{pyclass, pymethods, PyResult};

use crate::bdd::Bdd;
use crate::bindings::expression::PythonExpression;
use crate::expressions::Expression;
use crate::traits::BooleanFunction;

#[pyclass(frozen, name = "Bdd")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PythonBdd {
    root: Bdd<String>,
}

#[pymethods]
impl PythonBdd {
    #[staticmethod]
    pub fn from_expression(expression: &PythonExpression) -> PyResult<Self> {
        let native: Expression<String> = expression.into();
        match Bdd::try_from(native) {
            Ok(bdd) => Ok(Self::new(bdd)),
            Err(_e) => Err(PyRuntimeError::new_err(
                "Conversion failed. Too many variables.",
            )),
        }
    }

    #[staticmethod]
    pub fn mk_and(left: &PythonBdd, right: &PythonBdd) -> PythonBdd {
        PythonBdd::new(&left.root & &right.root)
    }

    #[staticmethod]
    pub fn mk_or(left: &PythonBdd, right: &PythonBdd) -> PythonBdd {
        PythonBdd::new(&left.root | &right.root)
    }

    #[staticmethod]
    pub fn mk_xor(left: &PythonBdd, right: &PythonBdd) -> PythonBdd {
        PythonBdd::new(&left.root ^ &right.root)
    }

    #[staticmethod]
    pub fn mk_const(value: bool) -> PythonBdd {
        PythonBdd::new(Bdd::mk_const(value))
    }

    #[staticmethod]
    pub fn mk_literal(variable: &str, value: bool) -> PythonBdd {
        PythonBdd::new(Bdd::mk_literal(variable.to_string(), value))
    }

    pub fn weight(&self) -> BigUint {
        self.root.weight()
    }

    pub fn node_count(&self) -> usize {
        self.root.node_count()
    }

    pub fn __str__(&self) -> String {
        format!("{:?}", self.root)
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

impl PythonBdd {
    pub fn new(root: Bdd<String>) -> PythonBdd {
        PythonBdd { root }
    }
}
