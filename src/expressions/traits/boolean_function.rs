use crate::expressions::iterators::{
    ExpressionDomainIterator, ExpressionImageIterator, ExpressionRelationIterator,
    ExpressionSupportIterator,
};
use crate::expressions::Expression;
use crate::expressions::ExpressionNode::{And, Constant, Literal, Not, Or};
use crate::traits::{
    BooleanFunction, BooleanValuation, Evaluate, GatherLiterals, PowerSet, SemanticEq,
};
use crate::utils::btreeset_to_valuation;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

impl<T: Debug + Clone + Ord> BooleanFunction<T> for Expression<T> {
    type DomainIterator = ExpressionDomainIterator;
    type RangeIterator = ExpressionImageIterator<T>;
    type RelationIterator = ExpressionRelationIterator<T>;
    type SupportIterator = ExpressionSupportIterator<T>;

    fn inputs(&self) -> BTreeSet<T> {
        self.gather_literals()
    }

    fn essential_inputs(&self) -> BTreeSet<T> {
        self.inputs()
            .into_iter()
            .filter(|input| {
                let true_valuation = BTreeMap::from([(input.clone(), true)]);
                let true_fixation = self.restrict(&true_valuation);

                let false_valuation = BTreeMap::from([(input.clone(), false)]);
                let false_fixation = self.restrict(&false_valuation);

                !true_fixation.semantic_eq(&false_fixation)
            })
            .collect()
    }

    fn domain(&self) -> Self::DomainIterator {
        self.into()
    }

    fn image(&self) -> Self::RangeIterator {
        self.into()
    }

    fn relation(&self) -> Self::RelationIterator {
        self.into()
    }

    fn support(&self) -> Self::SupportIterator {
        self.into()
    }

    fn restrict(&self, valuation: &BooleanValuation<T>) -> Self {
        let mapping = BTreeMap::from_iter(
            valuation
                .iter()
                .map(|(key, value)| (key.clone(), Constant(*value).into())),
        );

        self.substitute(&mapping)
    }

    fn substitute(&self, mapping: &BTreeMap<T, Self>) -> Self {
        self.substitute_rec(mapping)
    }

    fn existential_quantification(&self, variables: BTreeSet<T>) -> Self {
        self.restrict(&btreeset_to_valuation(variables.clone(), false))
            | self.restrict(&btreeset_to_valuation(variables, true))
    }

    fn universal_quantification(&self, variables: BTreeSet<T>) -> Self {
        self.restrict(&btreeset_to_valuation(variables.clone(), false))
            & self.restrict(&btreeset_to_valuation(variables, true))
    }

    fn derivative(&self, variables: BTreeSet<T>) -> Self {
        self.restrict(&btreeset_to_valuation(variables.clone(), false))
            ^ self.restrict(&btreeset_to_valuation(variables, true))
    }

    fn is_equivalent(&self, other: &Self) -> bool {
        self.semantic_eq(other)
    }

    fn is_implied_by(&self, other: &Self) -> bool {
        let self_literals = self.gather_literals();
        let other_literals = other.gather_literals();
        let literals_union = BTreeSet::from_iter(self_literals.union(&other_literals).cloned());

        let all_options = Self::generate_arbitrary_power_set(literals_union);

        all_options
            .into_iter()
            .all(|valuation| !other.evaluate(&valuation) | self.evaluate(&valuation))
    }
}

impl<T: Debug + Clone + Eq + Ord> Expression<T> {
    fn substitute_rec(&self, mapping: &BTreeMap<T, Self>) -> Self {
        match self.node() {
            Literal(name) => match mapping.get(name) {
                None => Literal(name.clone()).into(),
                Some(new_value) => new_value.clone(),
            },
            Not(e) => Not(e.substitute_rec(mapping)).into(),
            And(es) => And(es.iter().map(|e| e.substitute_rec(mapping)).collect()).into(),
            Or(es) => Or(es.iter().map(|e| e.substitute_rec(mapping)).collect()).into(),
            Constant(const_value) => Constant(*const_value).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::{bool, var, Expression};
    use crate::table::TruthTable;
    use crate::traits::{BooleanFunction, Implication, SemanticEq};
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn test_inputs_ok() {
        for var_count in 0..100 {
            let vars = (0..var_count).map(var).collect::<Vec<_>>();
            let input = Expression::n_ary_and(&vars);

            let expected = BTreeSet::from_iter((0..var_count).map(|c| c.to_string()));
            let actual = input.inputs();

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_essential_inputs_all_inputs_ok() {
        let input = var("a") & var("b");

        let actual = input.essential_inputs();
        let expected = BTreeSet::from_iter(["a".to_string(), "b".to_string()]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_essential_inputs_no_inputs_ok() {
        let input = (var("a") & var("b")).imply(var("c") | !var("c"));

        let actual = input.essential_inputs();
        let expected = BTreeSet::new();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_essential_inputs_some_inputs_ok() {
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

        let actual = input.essential_inputs();
        let expected = BTreeSet::from_iter(["x", "y"]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_restrict_ok() {
        let input = (var("a") | var("b")) & var("c");
        let valuation = BTreeMap::from_iter([("a".to_string(), false), ("c".to_string(), true)]);

        let expected = (bool(false) | var("b")) & bool(true);
        let actual = input.restrict(&valuation);

        assert_eq!(expected, actual);
        assert!(expected.semantic_eq(&actual));

        assert_eq!(actual.degree(), 1);
    }

    #[test]
    fn test_restrict_too_many_variables_ok() {
        let input = (var("a") | var("b")) & var("c");
        let valuation = BTreeMap::from_iter([
            ("a".to_string(), false),
            ("c".to_string(), true),
            ("notinthere".to_string(), true),
        ]);

        let expected = (bool(false) | var("b")) & bool(true);
        let actual = input.restrict(&valuation);

        assert_eq!(expected, actual);
        assert!(expected.semantic_eq(&actual));
        assert_eq!(actual.degree(), 1);
    }

    #[test]
    fn test_restrict_no_variables_ok() {
        let input = (var("a") | var("b")) & var("c");
        let valuation = BTreeMap::new();

        let expected = input.clone();
        let actual = input.restrict(&valuation);

        assert_eq!(expected, actual);
        assert!(expected.semantic_eq(&actual));
        assert_eq!(actual.degree(), expected.degree());
    }

    #[test]
    fn test_substitute_no_variables() {
        let input = (var("a") | var("b")) & var("c");
        let valuation = BTreeMap::new();

        let expected = input.clone();
        let actual = input.substitute(&valuation);

        assert_eq!(expected, actual);
        assert!(expected.semantic_eq(&actual));
        assert_eq!(actual.degree(), expected.degree());
    }

    #[test]
    fn test_substitute_variables_same_ok() {
        let input = (var("a") | var("b")) & var("c") & !var("a") & bool(true);
        let mapping = BTreeMap::from_iter([("a".to_string(), var("a") | !var("b"))]);

        // cannot use `var("a") | !var("b") | var("b")` for defining expected here
        // since that collapses Or(Or(a, !b), b), which substitute doesn't do
        let expected = Expression::n_ary_or(&[var("a") | !var("b"), var("b")])
            & var("c")
            & !(var("a") | !var("b"))
            & bool(true);
        let actual = input.substitute(&mapping);

        assert_eq!(expected, actual);
        assert!(expected.semantic_eq(&actual));
        assert_eq!(actual.degree(), expected.degree());
    }

    #[test]
    fn test_substitute_variables_added_ok() {
        let input = (var("a") | var("b")) & var("c") & !var("a") & bool(true);

        let new_value = var("ddd") & (bool(false) | var("a"));
        let mapping = BTreeMap::from_iter([("a".to_string(), new_value.clone())]);

        // cannot use bitwise operators for defining expected here
        // since that collapses Or(Or(a, !b), b), which substitute doesn't do
        let expected = Expression::n_ary_or(&[new_value.clone(), var("b")])
            & var("c")
            & !new_value.clone()
            & bool(true);
        let actual = input.substitute(&mapping);

        assert_eq!(expected, actual);
        assert!(expected.semantic_eq(&actual));

        assert_eq!(input.degree(), 3);
        assert_eq!(actual.degree(), 4);
        assert_eq!(actual.degree(), expected.degree());
    }

    #[test]
    fn test_substitute_variables_removed_ok() {
        let input = (var("a") | var("b")) & var("c") & !var("a") & bool(true);

        let new_value = bool(false);
        let mapping = BTreeMap::from_iter([("a".to_string(), new_value.clone())]);

        // cannot use bitwise operators for defining expected here
        // since that collapses Or(Or(a, !b), b), which substitute doesn't do
        let expected = Expression::n_ary_or(&[new_value.clone(), var("b")])
            & var("c")
            & !new_value.clone()
            & bool(true);
        let actual = input.substitute(&mapping);

        assert_eq!(expected, actual);
        assert!(expected.semantic_eq(&actual));

        assert_eq!(input.degree(), 3);
        assert_eq!(actual.degree(), 2);
        assert_eq!(actual.degree(), expected.degree());
    }
}
