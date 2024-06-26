use num_bigint::BigUint;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

pub type BooleanPoint = Vec<bool>;
pub type BooleanValuation<T> = BTreeMap<T, bool>;

/// A trait implemented by a data structure which represents a Boolean function.
///
/// In method descriptions, we write `n` to denote the number of variables supported by the
/// function and `|F|` to denote the actual size of the function representation.
pub trait BooleanFunction<T>: Sized
where
    T: Debug + Clone + Eq + Ord,
{
    type DomainIterator: Iterator<Item = BooleanPoint>;
    type RangeIterator: Iterator<Item = bool>;
    type RelationIterator: Iterator<Item = (BooleanPoint, bool)>;
    type SupportIterator: Iterator<Item = BooleanPoint>;

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
    ///
    fn inputs(&self) -> BTreeSet<T>;

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
    fn essential_inputs(&self) -> BTreeSet<T>;

    /// The number of variables that (syntactically) appear in this Boolean function.
    ///
    /// This is equivalent to the length of [BooleanFunction::inputs].
    fn degree(&self) -> usize {
        self.inputs().len()
    }

    /// The number of variables that are essential in this Boolean function.
    ///
    /// This is equivalent to the length of [BooleanFunction::essential_inputs].
    fn essential_degree(&self) -> usize {
        self.essential_inputs().len()
    }

    /// The iterator over Boolean points that are valid as inputs for this Boolean function.
    ///
    /// This is always the complete space of `2^n` Boolean vectors.
    fn domain(&self) -> Self::DomainIterator;

    /// The iterator over all the output values of this Boolean function.
    ///
    /// The iteration order should correspond to the elements of [BooleanFunction::domain].
    fn image(&self) -> Self::RangeIterator;

    /// The combined iterator of all input points together with their corresponding outputs.
    ///
    /// See also [BooleanFunction::domain] and [BooleanFunction::image].
    fn relation(&self) -> Self::RelationIterator;

    /// The iterator over all Boolean points for which this function evaluates to `1`.
    fn support(&self) -> Self::SupportIterator;

    /// The number of input points for which this function evaluates to `1`.
    ///
    /// See also [BooleanFunction::support].
    fn weight(&self) -> BigUint {
        self.support().count().into()
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
    fn restrict(&self, valuation: &BooleanValuation<T>) -> Self;

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
    fn substitute(&self, mapping: &BTreeMap<T, Self>) -> Self;

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
        self.support().next()
    }

    /// Eliminate the specified `variables` using *existential* quantification. The resulting
    /// function does not depend on any of the eliminated variables.
    ///
    /// For each variable, this computes `F = F[v = 0] | F[v = 1]`. In other words, the resulting
    /// function is satisfied for input `x` if there *exists* a value `b` of `v` such that the
    /// original function was satisfied for `x[v=b]`.
    ///
    fn existential_quantification(&self, variables: BTreeSet<T>) -> Self;

    /// Eliminate the specified `variables` using *universal* quantification. The resulting
    /// function does not depend on any of the eliminated variables.
    ///
    /// For each variable, this computes `F = F[v = 0] & F[v = 1]`. In other words, the resulting
    /// function is satisfied for `x` if the original function was satisfied for both `x[v=0]`
    /// and `x[v=1]`.
    ///
    fn universal_quantification(&self, variables: BTreeSet<T>) -> Self;

    /// Computes the derivative of this function with respect to the given `variables`.
    /// The resulting function does not depend on any of the eliminated variables.
    ///
    /// For each variable, this computes `F = F[v = 0] ^ F[v = 1]`. In other words, the resulting
    /// function is satisfied for `x`, if the values of `F(x[v=0])` and `F(x[v=1])` are different.
    /// (Hence the name "derivative": the result is a function that is true for all inputs in
    /// which the input function can change its value).
    ///
    fn derivative(&self, variables: BTreeSet<T>) -> Self;

    /// Returns `true` if the two functions are *semantically* equivalent. That is, they output
    /// the same values for the same inputs.
    fn is_equivalent(&self, other: &Self) -> bool;

    /// Returns `true` if this function is *implied* by the `other` function. That is, it outputs
    /// `1` *at least* for those inputs where `other` outputs one.
    fn is_implied_by(&self, other: &Self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::{bool, var, Expression};
    use crate::table::TruthTable;
    use crate::traits::{GatherLiterals, Implication, SemanticEq};

    #[test]
    fn test_degree() {
        let ascii_start = '!';
        for var_count in ascii_start..'}' {
            let vars = (ascii_start..var_count)
                .map(|c: char| var(c))
                .collect::<Vec<_>>();
            let input = Expression::n_ary_and(&vars);

            let expected = var_count as usize - ascii_start as usize;
            let actual = input.degree();

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_essential_degree() {
        // the boolean function doesn't depend on Z, but does on X and Y
        // "x,y,z,output\n",
        // "0,0,1,1\n",
        // "0,0,0,1\n",
        // "0,1,1,0\n",
        // "0,1,0,0\n",
        // "1,0,1,0\n",
        // "1,0,0,0\n",
        // "1,1,1,0\n",
        // "1,1,0,0\n",

        let input = TruthTable::new(
            vec!["x", "y", "z"],
            vec![false, false, true, true, true, true, true, true],
        )
        .to_expression_trivial();

        let actual = input.essential_degree();
        let expected = BTreeSet::from_iter(["x", "y"]).len();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_saturating_point_some() {
        let input = var("0") & var("1") & var("2");

        let actual = input.sat_point();
        let expected = Some(vec![true, true, true]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_saturating_none() {
        // (p ∨ q) ∧ (¬p) ∧ (¬q)
        let input = Expression::n_ary_and(&[var("a") | var("b"), !var("a"), !var("b")]);

        let actual = input.sat_point();
        let expected = None;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_weight_one() {
        let input = var("0") & var("1") & var("2");

        let actual = input.weight();
        let expected = BigUint::from(1u8);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_weight_zero() {
        let input = Expression::n_ary_and(&[var("a") | var("b"), !var("a"), !var("b")]);

        let actual = input.weight();
        let expected = BigUint::from(0u8);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_weight_all() {
        // tautology
        let input = ((var("A").imply(var("B"))) & (!var("A").imply(!var("B")))).imply(var("A"));
        assert!(input.semantic_eq(&bool(true)));

        let actual = input.weight();
        let expected = 2usize.pow(input.gather_literals().len() as u32).into();

        assert_eq!(actual, expected);
    }
}
