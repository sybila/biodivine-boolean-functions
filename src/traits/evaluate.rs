use std::collections::BTreeMap;
use std::fmt::Debug;

pub trait Evaluate<TLiteral: Debug + Clone + Eq + Ord> {
    /// Evaluates the expression with a given valuation of variable names to their values (i.e. `x_0: true` or `x_0: false`),
    /// returning a boolean value.
    /// If a variable is found in the expression but not in the `literal_values` parameter, it is treated as false.
    ///
    /// Defaults to `self.evaluate_with_default(literal_values, false)`.
    fn evaluate(&self, literal_values: &BTreeMap<TLiteral, bool>) -> bool {
        self.evaluate_with_default(literal_values, false)
    }

    /// Evaluates the expression with a given valuation of variable names to their values (i.e. `x_0: true` or `x_0: false`),
    /// returning a boolean value.
    ///
    /// If a variable is found in the expression but not in the `literal_values` parameter, the passed `default_value` is used.
    fn evaluate_with_default(
        &self,
        literal_values: &BTreeMap<TLiteral, bool>,
        default_value: bool,
    ) -> bool;

    /// Evaluates the expression with a given valuation of variable names to their values (i.e. `x_0: true` or `x_0: false`)/
    ///
    /// If a variable is found in the expression but not in the `literal_values` parameter,
    /// the function returns an `Vector` of all such variables.
    ///
    /// Otherwise, an `Ok(value)` is returned.
    fn evaluate_checked(
        &self,
        literal_values: &BTreeMap<TLiteral, bool>,
    ) -> Result<bool, Vec<TLiteral>>;
}
