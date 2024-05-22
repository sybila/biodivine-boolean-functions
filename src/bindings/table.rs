use num_bigint::BigUint;
use std::collections::{BTreeMap, BTreeSet};

use crate::bdd::Bdd;
use crate::bindings::bdd::PythonBdd;
use pyo3::PyResult;

use crate::bindings::error::PythonExpressionError::UnknownVariableWhileEvaluating;
use crate::bindings::expression::PythonExpression;
use crate::bindings::iterators::{
    PythonDomainIterator, PythonTableRangeIterator, PythonTableRelationIterator,
    PythonTableSupportIterator,
};
use crate::expressions::Expression as RustExpression;
use crate::table::display_formatted::{TableBooleanFormatting, TableStyle};
use crate::table::TruthTable;
use crate::traits::{
    BooleanFunction, BooleanPoint, BooleanValuation, Evaluate, GatherLiterals, SemanticEq,
};

#[pyo3::pyclass(frozen, name = "Table")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PythonTruthTable {
    root: TruthTable<String>,
}

impl From<TruthTable<String>> for PythonTruthTable {
    fn from(value: TruthTable<String>) -> Self {
        PythonTruthTable::new(value)
    }
}

impl From<PythonTruthTable> for TruthTable<String> {
    fn from(value: PythonTruthTable) -> Self {
        (&value).into()
    }
}

impl From<&PythonTruthTable> for TruthTable<String> {
    fn from(value: &PythonTruthTable) -> Self {
        value.root.clone()
    }
}

#[pyo3::pymethods]
impl PythonTruthTable {
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
    pub fn evaluate_safe(&self, literal_values: BTreeMap<String, bool>) -> bool {
        self.root.evaluate(&literal_values)
    }

    /// Variables not in the dictionary defaults to the passed `default_value` argument.
    pub fn evaluate_with_default(
        &self,
        literal_values: BTreeMap<String, bool>,
        default_value: bool,
    ) -> bool {
        self.root
            .evaluate_with_default(&literal_values, default_value)
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
        format!("PythonTruthTable(\n{})", self.__str__())
    }

    #[staticmethod]
    pub fn mk_and(left: &Self, right: &Self) -> Self {
        PythonTruthTable::new(&left.root & &right.root)
    }

    #[staticmethod]
    pub fn mk_or(left: &Self, right: &Self) -> Self {
        PythonTruthTable::new(&left.root | &right.root)
    }

    #[staticmethod]
    pub fn mk_xor(left: &Self, right: &Self) -> Self {
        PythonTruthTable::new(&left.root ^ &right.root)
    }

    #[staticmethod]
    pub fn mk_not(left: &Self) -> Self {
        PythonTruthTable::new(!&left.root)
    }

    /// A set of all the variable instances that (syntactically) appear in this Boolean
    /// function.
    ///
    /// See also [BooleanFunction::essential_inputs].
    ///
    /// ### Examples
    ///
    /// The inputs of the expression `(a & b) => (c | !c)` are `{a, b, c}`.
    ///
    /// ### Implementation notes
    ///
    /// This operation should be at worst `O(|F|)` for any function representation.
    fn inputs(&self) -> BTreeSet<String> {
        self.root.inputs()
    }

    /// A set of all variable instances that are *essential* in this Boolean function.
    ///
    /// Intuitively, a variable `v` is essential in function `F` if `v` has some observable
    /// impact on the output of `F`. In other words, there is some input vector `X` such that
    /// `F(X[v=0]) != F(X[v=1])` (here `X[v=b]` denotes a copy of `X` with the value of `v`
    /// fixed to `b`).
    ///
    /// For a proper formal definition, see for example the introduction in
    /// [this paper](https://arxiv.org/pdf/0812.1979.pdf).
    ///
    /// See also [BooleanFunction::inputs].
    ///
    /// ### Examples
    ///
    /// The essential inputs of the expression `(a & b) => (c | !c)` are `{}`, because
    /// this expression is a tautology.
    ///
    /// ### Implementation notes
    ///
    ///  * BDD: The operation takes `O(|F|)` time by scanning the variables stored in the
    ///    BDD nodes.
    ///  * Table: The operation takes `O(n * |F|)` time by scanning the corresponding output
    ///    pairs for each variable.
    ///  * Expression: The operation is non-trivial, as we need to determine for each variable
    ///    whether `F[v = 0]` and `F[v = 1]` are semantically equal.
    ///
    fn essential_inputs(&self) -> BTreeSet<String> {
        self.root.essential_inputs()
    }

    /// The number of variables that (syntactically) appear in this Boolean function.
    ///
    /// This is equivalent to the length of [BooleanFunction::inputs].
    fn degree(&self) -> usize {
        self.root.degree()
    }

    /// The number of variables that are essential in this Boolean function.
    ///
    /// This is equivalent to the length of [BooleanFunction::essential_inputs].
    fn essential_degree(&self) -> usize {
        self.root.essential_degree()
    }

    /// The iterator over Boolean points that are valid as inputs for this Boolean function.
    ///
    /// This is always the complete space of `2^n` Boolean vectors.
    fn domain(&self) -> PythonDomainIterator {
        self.root.domain().into()
    }

    /// The iterator over all the output values of this Boolean function.
    ///
    /// The iteration order should correspond to the elements of [BooleanFunction::domain].
    fn image(&self) -> PythonTableRangeIterator {
        self.root.image().into()
    }

    /// The combined iterator of all input points together with their corresponding outputs.
    ///
    /// See also [BooleanFunction::domain] and [BooleanFunction::image].
    fn relation(&self) -> PythonTableRelationIterator {
        self.root.relation().into()
    }

    /// The iterator over all Boolean points for which this function evaluates to `1`.
    fn support(&self) -> PythonTableSupportIterator {
        self.root.support().into()
    }

    /// The number of input points for which this function evaluates to `1`.
    ///
    /// See also [BooleanFunction::support].
    fn weight(&self) -> BigUint {
        self.root.weight()
    }

    /// Create a Boolean function that is a restriction of this function for the given variables.
    ///
    /// A restriction fixes all variables specified by the `valuation` to their respective
    /// constant values. That is, the resulting function no longer depends on these variables.
    ///
    /// ### Examples
    ///
    /// A Boolean expression `(a | b) & c` restricted to `{ a: 0, c: 1 }` is `(false | b) & true`
    /// semantically equal to `b`.
    ///
    /// ### Implementation notes
    ///
    /// It is not an error to supply a valuation that also fixes variables that are not the inputs
    /// of this function. Such variables are simply ignored.
    fn restrict(&self, valuation: BooleanValuation<String>) -> Self {
        self.root.restrict(&valuation).into()
    }

    /// Create a Boolean function in which the variables specified by `mapping` are substituted
    /// for their supplied functions.
    ///
    /// ### Examples
    ///
    /// Substituting `a` for `(a | b | c)` in the expression `a & !c` produces `(a | b | c) & !c`.
    ///
    /// ### Implementation notes
    ///
    /// Note that the same variable can be substituted and at the same time appear in one of the
    /// substituted functions (as in the example). Also note that this operation can increase the
    /// degree of a function if the substituted functions contain previously unused variables.
    fn substitute(&self, mapping: BTreeMap<String, Self>) -> Self {
        self.root
            .substitute(&BTreeMap::from_iter(
                mapping.into_iter().map(|(k, v)| (k, v.root)),
            ))
            .into()
    }

    /// Produce one [BooleanPoint] for which this function evaluates to `1`, i.e. one of the
    /// points in [BooleanFunction::support].
    ///
    /// This value should be deterministic, but otherwise can be arbitrary. Returns `None` if
    /// the function is not satisfiable.
    ///
    /// ### Implementation notes
    ///
    /// This operation is `O(|F|)` for tables, `O(1)` for BDDs, and NP-complete for expressions.
    fn sat_point(&self) -> Option<BooleanPoint> {
        self.root.sat_point()
    }

    /// Eliminate the specified `variables` using *existential* quantification. The resulting
    /// function does not depend on any of the eliminated variables.
    ///
    /// For each variable, this computes `F = F[v = 0] | F[v = 1]`. In other words, the resulting
    /// function is satisfied for input `x` if there *exists* a value `b` of `v` such that the
    /// original function was satisfied for `x[v=b]`.
    ///
    fn existential_quantification(&self, variables: BTreeSet<String>) -> Self {
        self.root.existential_quantification(variables).into()
    }

    /// Eliminate the specified `variables` using *universal* quantification. The resulting
    /// function does not depend on any of the eliminated variables.
    ///
    /// For each variable, this computes `F = F[v = 0] & F[v = 1]`. In other words, the resulting
    /// function is satisfied for `x` if the original function was satisfied for both `x[v=0]`
    /// and `x[v=1]`.
    ///
    fn universal_quantification(&self, variables: BTreeSet<String>) -> Self {
        self.root.universal_quantification(variables).into()
    }

    /// Computes the derivative of this function with respect to the given `variables`.
    /// The resulting function does not depend on any of the eliminated variables.
    ///
    /// For each variable, this computes `F = F[v = 0] ^ F[v = 1]`. In other words, the resulting
    /// function is satisfied for `x`, if the values of `F(x[v=0])` and `F(x[v=1])` are different.
    /// (Hence the name "derivative": the result is a function that is true for all inputs in
    /// which the input function can change its value).
    ///
    fn derivative(&self, variables: BTreeSet<String>) -> Self {
        self.root.derivative(variables).into()
    }

    /// Returns `true` if the two functions are *semantically* equivalent. That is, they output
    /// the same values for the same inputs.
    fn is_equivalent(&self, other: &Self) -> bool {
        self.root.is_equivalent(&other.root)
    }

    /// Returns `true` if this function is *implied* by the `other` function. That is, it outputs
    /// `1` *at least* for those inputs where `other` outputs one.
    fn is_implied_by(&self, other: &Self) -> bool {
        self.root.is_implied_by(&other.root)
    }

    #[staticmethod]
    pub fn from_expression(expression: &PythonExpression) -> Self {
        let rust_expression: RustExpression<String> = expression.into();
        let rust_table: TruthTable<String> = rust_expression.into();

        rust_table.into()
    }

    #[staticmethod]
    pub fn from_bdd(expression: &PythonBdd) -> Self {
        let rust_expression: Bdd<String> = expression.into();
        let rust_table: TruthTable<String> = rust_expression.into();

        rust_table.into()
    }

    pub fn to_expression(&self) -> PythonExpression {
        PythonExpression::from_table(self)
    }

    pub fn to_bdd(&self) -> PyResult<PythonBdd> {
        PythonBdd::from_table(self)
    }
}

impl PythonTruthTable {
    fn new(table: TruthTable<String>) -> Self {
        PythonTruthTable { root: table }
    }
}
