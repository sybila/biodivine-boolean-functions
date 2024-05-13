use crate::bdd::iterators::{ImageIterator, SupportIterator};
use crate::bdd::Bdd;
use crate::iterators::DomainIterator;
use crate::traits::{BooleanFunction, BooleanPoint, BooleanValuation};
use biodivine_lib_bdd::Bdd as InnerBdd;
use biodivine_lib_bdd::BddVariable;
use num_bigint::BigUint;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Debug;
use std::iter::{zip, Zip};

impl<T: Debug + Clone + Ord> BooleanFunction<T> for Bdd<T> {
    type DomainIterator = DomainIterator;
    type RangeIterator = ImageIterator;
    type RelationIterator = Zip<DomainIterator, ImageIterator>;
    type SupportIterator = SupportIterator;

    fn inputs(&self) -> BTreeSet<T> {
        self.inputs.iter().cloned().collect()
    }

    fn essential_inputs(&self) -> BTreeSet<T> {
        self.bdd
            .support_set()
            .into_iter()
            .map(|var| {
                // This unwrap is safe unless the BDD has variables that we don't know about.
                self.map_var_inner_to_outer(var).unwrap()
            })
            .collect()
    }

    fn degree(&self) -> usize {
        self.inputs.len()
    }

    fn essential_degree(&self) -> usize {
        self.bdd.support_set().len()
    }

    fn domain(&self) -> Self::DomainIterator {
        DomainIterator::from_count(self.inputs.len())
    }

    fn image(&self) -> Self::RangeIterator {
        // evaluate for each domain point
        // DomainIterator::new(self).map(|it| self.bdd.eval_in(&BddValuation::new(it)));
        ImageIterator::new(self.inputs.len(), &self.bdd)
    }

    fn relation(&self) -> Self::RelationIterator {
        // zip domain/range
        zip(self.domain(), self.image())
    }

    fn support(&self) -> Self::SupportIterator {
        SupportIterator::new(&self.bdd)
    }

    fn weight(&self) -> BigUint {
        self.bdd.exact_cardinality().to_biguint().unwrap()
    }

    fn restrict(&self, valuation: &BooleanValuation<T>) -> Self {
        let lib_bdd_valuation: Vec<(BddVariable, bool)> = valuation
            .iter()
            .filter_map(|(a, b)| self.map_var_outer_to_inner(a).map(|var| (var, *b)))
            .collect::<Vec<_>>();
        let new_bdd = Bdd::new(self.bdd.restrict(&lib_bdd_valuation), self.inputs.clone());

        self.restrict_and_prune_map(valuation, &new_bdd)
    }

    fn substitute(&self, mapping: &BTreeMap<T, Self>) -> Self {
        // Bdd.substitute exists, but assumes all BDDs share input variables (we need to extend)
        // and does not eliminate the substituted variable from inputs afterward (we need to prune).

        // Bdd.substitute currently assumes that the substituted functions does not depend on the
        // substituted variables. This will be solved in lib-bdd, we can just panic for now.
        if mapping
            .iter()
            .any(|(key, value_bdd)| value_bdd.inputs.contains(key))
        {
            panic!("Currently not allowed to have the substituted variable also appear in the substituting BDD value");
        }

        let mut extended_mapping = mapping.clone();
        let (mut self_lifted, _common_inputs) = self.union_and_extend_n_ary(&mut extended_mapping);

        for (k, v) in extended_mapping.iter() {
            self_lifted.bdd = self_lifted
                .bdd
                .substitute(self_lifted.map_var_outer_to_inner(k).unwrap(), &v.bdd)
        }

        self_lifted.restrict_and_prune_map(&extended_mapping, &self_lifted)
    }

    fn sat_point(&self) -> Option<BooleanPoint> {
        self.bdd.sat_witness().map(|it| it.vector())
    }

    fn existential_quantification(&self, variables: BTreeSet<T>) -> Self {
        let lib_bdd_variables = variables
            .iter()
            .filter_map(|it| self.map_var_outer_to_inner(it))
            .collect::<Vec<_>>();
        let new_bdd = Bdd::new(self.bdd.exists(&lib_bdd_variables), self.inputs.clone());

        self.restrict_and_prune_set(&variables, &new_bdd)
    }

    fn universal_quantification(&self, variables: BTreeSet<T>) -> Self {
        let lib_bdd_variables = variables
            .iter()
            .filter_map(|it| self.map_var_outer_to_inner(it))
            .collect::<Vec<_>>();
        let new_bdd = Bdd::new(self.bdd.for_all(&lib_bdd_variables), self.inputs.clone());

        self.restrict_and_prune_set(&variables, &new_bdd)
    }

    fn derivative(&self, variables: BTreeSet<T>) -> Self {
        let lib_bdd_variables = variables
            .iter()
            .filter_map(|it| self.map_var_outer_to_inner(it))
            .collect::<HashSet<_>>();
        let trigger = |var: BddVariable| lib_bdd_variables.contains(&var);

        let new_bdd = Bdd::new(
            InnerBdd::binary_op_nested(
                &self.bdd,
                &self.bdd,
                trigger,
                biodivine_lib_bdd::op_function::and,
                biodivine_lib_bdd::op_function::xor,
            ),
            self.inputs.clone(),
        );

        self.restrict_and_prune_set(&variables, &new_bdd)
    }

    fn is_equivalent(&self, other: &Self) -> bool {
        let (self_lifted, other_lifted, _common_inputs) = self.union_and_extend(other);

        self_lifted.bdd == other_lifted.bdd
    }

    fn is_implied_by(&self, other: &Self) -> bool {
        let (self_lifted, other_lifted, _common_inputs) = self.union_and_extend(other);

        other_lifted.bdd.imp(&self_lifted.bdd).is_true()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::{bool, var, Expression};
    use crate::table::TruthTable;
    use crate::traits::{Evaluate, Implication};
    use crate::utils::btreeset_to_valuation;

    #[test]
    fn test_inputs_ok() {
        for var_count in 0..100 {
            let vars = (0..var_count).map(var).collect::<Vec<_>>();
            let input = Bdd::try_from(Expression::n_ary_and(&vars)).expect("Should not panic here");

            let expected = BTreeSet::from_iter((0..var_count).map(|c| c.to_string()));
            let actual = input.inputs();

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_essential_inputs_all_inputs_ok() {
        let input = Bdd::try_from(var("a") & var("b")).expect("Should not panic here");

        let actual = input.essential_inputs();
        let expected = BTreeSet::from_iter(["a".to_string(), "b".to_string()]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_essential_inputs_no_inputs_ok() {
        let input = Bdd::try_from((var("a") & var("b")).imply(var("c") | !var("c")))
            .expect("Should not panic here");

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

        let input = Bdd::try_from(
            TruthTable::new(
                vec!["x", "y", "z"],
                vec![false, false, true, true, true, true, true, true],
            )
            .to_expression_trivial(),
        )
        .expect("Should not panic here");

        let actual = input.essential_inputs();
        let expected = BTreeSet::from_iter(["x", "y"]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_essential_inputs_xor_all_inputs_ok() {
        let input = Bdd::try_from(var("a") ^ var("b")).expect("Should not panic here");

        let actual = input.essential_inputs();
        let expected = BTreeSet::from_iter(["a".to_string(), "b".to_string()]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_domain_ok() {
        let input = Bdd::try_from(var("d") & var("b") | var("a")).expect("Should not panic here");

        let actual = input.domain().collect::<Vec<_>>();

        let expected = vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
            vec![false, true, true],
            vec![true, false, false],
            vec![true, false, true],
            vec![true, true, false],
            vec![true, true, true],
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_restrict_ok() {
        let input = Bdd::try_from((var("a") | var("b")) & var("c")).expect("Should not panic here");
        let valuation = BTreeMap::from_iter([("a".to_string(), false), ("c".to_string(), true)]);

        let expected =
            Bdd::try_from((bool(false) | var("b")) & bool(true)).expect("Should not panic here");
        let actual = input.restrict(&valuation);

        assert_eq!(expected, actual);
        assert!(expected.is_equivalent(&actual));

        assert_eq!(actual.degree(), 1);
    }

    #[test]
    fn test_restrict_too_many_variables_ok() {
        let input = Bdd::try_from((var("a") | var("b")) & var("c")).expect("Should not panic here");
        let valuation = BTreeMap::from_iter([
            ("a".to_string(), false),
            ("c".to_string(), true),
            ("notinthere".to_string(), true),
        ]);

        let expected =
            Bdd::try_from((bool(false) | var("b")) & bool(true)).expect("Should not panic here");
        let actual = input.restrict(&valuation);

        assert_eq!(expected, actual);
        assert!(expected.is_equivalent(&actual));
        assert_eq!(actual.degree(), 1);
    }

    #[test]
    fn test_restrict_no_variables_ok() {
        let input = Bdd::try_from((var("a") | var("b")) & var("c")).expect("Should not panic here");
        let valuation = BTreeMap::new();

        let expected = input.clone();
        let actual = input.restrict(&valuation);

        assert_eq!(expected, actual);
        assert!(expected.is_equivalent(&actual));
        assert_eq!(actual.degree(), expected.degree());
    }

    #[test]
    fn test_substitute_no_variables() {
        let input = Bdd::try_from((var("a") | var("b")) & var("c")).expect("Should not panic here");
        let valuation = BTreeMap::new();

        let expected = input.clone();
        let actual = input.substitute(&valuation);

        assert_eq!(expected, actual);
        assert!(expected.is_equivalent(&actual));
        assert_eq!(actual.degree(), expected.degree());
    }

    // #[test]
    // fn test_substitute_variables_same_substituted_ok() {
    //     let input = Bdd::try_from((var("a") | var("b")) & var("c") & !var("a") & bool(true))
    //         .expect("Should not panic here");
    //
    //     let new_value = var("a") | !var("b");
    //     let new_value_bdd = Bdd::try_from(new_value.clone()).expect("Should not panic here");
    //     let mapping = BTreeMap::from_iter([("a".to_string(), new_value_bdd)]);
    //
    //     // cannot use `var("a") | !var("b") | var("b")` for defining expected here
    //     // since that collapses Or(Or(a, !b), b), which substitute doesn't do
    //     let expected = Bdd::try_from(
    //         Expression::n_ary_or(&[new_value.clone(), var("b")])
    //             & var("c")
    //             & !new_value.clone()
    //             & bool(true),
    //     )
    //     .expect("Should not panic here");
    //     let actual = input.substitute(&mapping);
    //
    //     assert_eq!(expected, actual);
    //     assert!(expected.is_equivalent(&actual));
    //     assert_eq!(actual.degree(), expected.degree());
    // }
    //
    // #[test]
    // fn test_substitute_variables_same_unsubstituted_ok() {
    //     let input = Bdd::try_from((var("a") | var("b")) & var("c") & !var("a") & bool(true))
    //         .expect("Should not panic here");
    //
    //     let new_value = var("c") | !var("b");
    //     let new_value_bdd = Bdd::try_from(new_value.clone()).expect("Should not panic here");
    //     let mapping = BTreeMap::from_iter([("a".to_string(), new_value_bdd)]);
    //
    //     // cannot use `var("a") | !var("b") | var("b")` for defining expected here
    //     // since that collapses Or(Or(a, !b), b), which substitute doesn't do
    //     let expected = Bdd::try_from(
    //         Expression::n_ary_or(&[new_value.clone(), var("b")])
    //             & var("c")
    //             & !new_value.clone()
    //             & bool(true),
    //     )
    //     .expect("Should not panic here");
    //     let actual = input.substitute(&mapping);
    //
    //     assert_eq!(expected, actual);
    //     assert!(expected.is_equivalent(&actual));
    //     assert_eq!(actual.degree(), expected.degree());
    // }

    #[test]
    fn test_substitute_variables_added_only_ok() {
        let input = Bdd::try_from((var("a") | var("b")) & var("c") & !var("a") & bool(true))
            .expect("Should not panic here");

        let new_value = var("ddd") & (bool(false));
        let new_value_bdd = Bdd::try_from(new_value.clone()).expect("Should not panic here");
        let mapping = BTreeMap::from_iter([("a".to_string(), new_value_bdd)]);

        // cannot use bitwise operators for defining expected here
        // since that collapses Or(Or(a, !b), b), which substitute doesn't do
        let expected = Bdd::try_from(
            Expression::n_ary_or(&[new_value.clone(), var("b")])
                & var("c")
                & !new_value.clone()
                & bool(true),
        )
        .expect("Should not panic here");
        let actual = input.substitute(&mapping);

        assert_eq!(expected, actual);
        assert!(expected.is_equivalent(&actual));

        assert_eq!(input.degree(), 3);
        assert_eq!(actual.degree(), 3);
        assert_eq!(actual.degree(), expected.degree());
    }

    // #[test]
    // fn test_substitute_variables_added_and_substituted_ok() {
    //     let input = Bdd::try_from((var("a") | var("b")) & var("c") & !var("a") & bool(true))
    //         .expect("Should not panic here");
    //
    //     let new_value = var("ddd") & (bool(false) | var("a"));
    //     let new_value_bdd = Bdd::try_from(new_value.clone()).expect("Should not panic here");
    //     let mapping = BTreeMap::from_iter([("a".to_string(), new_value_bdd)]);
    //
    //     // cannot use bitwise operators for defining expected here
    //     // since that collapses Or(Or(a, !b), b), which substitute doesn't do
    //     let expected = Bdd::try_from(
    //         Expression::n_ary_or(&[new_value.clone(), var("b")])
    //             & var("c")
    //             & !new_value.clone()
    //             & bool(true),
    //     )
    //     .expect("Should not panic here");
    //     let actual = input.substitute(&mapping);
    //
    //     assert_eq!(expected, actual);
    //     assert!(expected.is_equivalent(&actual));
    //
    //     assert_eq!(input.degree(), 3);
    //     assert_eq!(actual.degree(), 4);
    //     assert_eq!(expected.degree(), 4);
    // }
    //
    // #[test]
    // fn test_substitute_variables_added_and_unsubstituted_ok() {
    //     let input = Bdd::try_from((var("a") | var("b")) & var("c") & !var("a") & bool(true))
    //         .expect("Should not panic here");
    //
    //     let new_value = var("ddd") & (bool(false) | var("b"));
    //     let new_value_bdd = Bdd::try_from(new_value.clone()).expect("Should not panic here");
    //     let mapping = BTreeMap::from_iter([("a".to_string(), new_value_bdd)]);
    //
    //     // cannot use bitwise operators for defining expected here
    //     // since that collapses Or(Or(a, !b), b), which substitute doesn't do
    //     let expected = Bdd::try_from(
    //         Expression::n_ary_or(&[new_value.clone(), var("b")])
    //             & var("c")
    //             & !new_value.clone()
    //             & bool(true),
    //     )
    //     .expect("Should not panic here");
    //     let actual = input.substitute(&mapping);
    //
    //     assert_eq!(expected, actual);
    //     assert!(expected.is_equivalent(&actual));
    //
    //     assert_eq!(input.degree(), 3);
    //     assert_eq!(actual.degree(), 3);
    //     assert_eq!(expected.degree(), 3);
    // }

    #[test]
    fn test_substitute_variables_removed_ok() {
        let input = Bdd::try_from((var("a") | var("b")) & var("c") & !var("a") & bool(true))
            .expect("Should not panic here");

        let new_value = bool(false);
        let new_value_bdd = Bdd::try_from(new_value.clone()).expect("Should not panic here");
        let mapping = BTreeMap::from_iter([("a".to_string(), new_value_bdd)]);

        // cannot use bitwise operators for defining expected here
        // since that collapses Or(Or(a, !b), b), which substitute doesn't do
        let expected = Bdd::try_from(
            Expression::n_ary_or(&[new_value.clone(), var("b")])
                & var("c")
                & !new_value.clone()
                & bool(true),
        )
        .expect("Should not panic here");
        let actual = input.substitute(&mapping);

        assert_eq!(expected, actual);
        assert!(expected.is_equivalent(&actual));

        assert_eq!(input.degree(), 3);
        assert_eq!(actual.degree(), 2);
        assert_eq!(actual.degree(), expected.degree());
    }

    #[test]
    fn test_existential_and_ok() {
        let target = "a".to_string();
        let set = BTreeSet::from([target.clone()]);
        let input = Bdd::try_from(var(target.clone()) & var("b")).expect("Should not panic here");

        let actual = input.existential_quantification(set.clone());
        assert!(!actual.inputs().contains(&target.clone()));

        let evaluated_with_true = actual.evaluate(&btreeset_to_valuation(set.clone(), true));
        let evaluated_with_false = actual.evaluate(&btreeset_to_valuation(set, false));

        assert_eq!(evaluated_with_true, evaluated_with_false);
        assert!(!evaluated_with_true)
    }

    #[test]
    fn test_existential_or_ok() {
        let target = "a".to_string();
        let set = BTreeSet::from([target.clone()]);
        let input = Bdd::try_from(var(target.clone()) | var("b")).expect("Should not panic here");

        let actual = input.existential_quantification(set.clone());
        assert!(!actual.inputs().contains(&target.clone()));

        let evaluated_with_true = actual.evaluate(&btreeset_to_valuation(set.clone(), true));
        let evaluated_with_false = actual.evaluate(&btreeset_to_valuation(set, false));

        assert_eq!(evaluated_with_true, evaluated_with_false);
        assert!(evaluated_with_true)
    }

    #[test]
    fn test_universal_and_ok() {
        let target = "a".to_string();
        let set = BTreeSet::from([target.clone()]);
        let input = Bdd::try_from(var(target.clone()) & var("b")).expect("Should not panic here");

        let actual = input.universal_quantification(set.clone());
        assert!(!actual.inputs().contains(&target.clone()));

        let evaluated_with_true = actual.evaluate(&btreeset_to_valuation(set.clone(), true));
        let evaluated_with_false = actual.evaluate(&btreeset_to_valuation(set, false));

        assert_eq!(evaluated_with_true, evaluated_with_false);
        assert!(!evaluated_with_true)
    }

    #[test]
    fn test_universal_or_ok() {
        let target = "a".to_string();
        let set = BTreeSet::from([target.clone()]);
        let input = Bdd::try_from(var(target.clone()) | var("b")).expect("Should not panic here");

        let actual = input.universal_quantification(set.clone());
        assert!(!actual.inputs().contains(&target.clone()));

        let evaluated_with_true = actual.evaluate(&btreeset_to_valuation(set.clone(), true));
        let evaluated_with_false = actual.evaluate(&btreeset_to_valuation(set, false));

        assert_eq!(evaluated_with_true, evaluated_with_false);
        assert!(!evaluated_with_true)
    }

    #[test]
    fn test_derivative_and_ok() {
        let target = "a".to_string();
        let set = BTreeSet::from([target.clone()]);
        let input = Bdd::try_from(var(target.clone()) & var("b")).expect("Should not panic here");

        let actual = input.derivative(set.clone());
        assert!(!actual.inputs().contains(&target.clone()));

        let evaluated_with_true = actual.evaluate(&btreeset_to_valuation(set.clone(), true));
        let evaluated_with_false = actual.evaluate(&btreeset_to_valuation(set, false));

        assert_eq!(evaluated_with_true, evaluated_with_false);
        assert!(!evaluated_with_true)
    }

    #[test]
    fn test_derivative_or_ok() {
        let target = "a".to_string();
        let set = BTreeSet::from([target.clone()]);
        let input = Bdd::try_from(var(target.clone()) | var("b")).expect("Should not panic here");

        let actual = input.derivative(set.clone());
        assert!(!actual.inputs().contains(&target.clone()));

        let evaluated_with_true = actual.evaluate(&btreeset_to_valuation(set.clone(), true));
        let evaluated_with_false = actual.evaluate(&btreeset_to_valuation(set, false));

        assert_eq!(evaluated_with_true, evaluated_with_false);
        assert!(evaluated_with_true)
    }

    #[test]
    fn test_derivative_xor_ok() {
        let target = "a".to_string();
        let set = BTreeSet::from([target.clone()]);
        let input = Bdd::try_from(var(target.clone()) ^ var("b")).expect("Should not panic here");

        let actual = input.derivative(set.clone());
        assert!(!actual.inputs().contains(&target.clone()));

        let evaluated_with_true = actual.evaluate(&btreeset_to_valuation(set.clone(), true));
        let evaluated_with_false = actual.evaluate(&btreeset_to_valuation(set, false));

        assert_eq!(evaluated_with_true, evaluated_with_false);
        assert!(evaluated_with_true)
    }
}
