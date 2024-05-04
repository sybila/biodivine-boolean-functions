use crate::iterators::DomainIterator;
use crate::table::iterators::{ImageIterator, RelationIterator, SupportIterator};
use crate::table::utils::values_to_row_index;
use crate::table::TruthTable;
use crate::traits::{
    BooleanFunction, BooleanValuation, Evaluate, GatherLiterals, PowerSet, SemanticEq,
};
use crate::utils::{boolean_point_to_valuation, btreeset_to_valuation};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

impl<T: Debug + Clone + Ord + 'static> BooleanFunction<T> for TruthTable<T> {
    type DomainIterator = DomainIterator;
    type RangeIterator = ImageIterator;
    type RelationIterator = RelationIterator;
    type SupportIterator = SupportIterator;

    fn inputs(&self) -> BTreeSet<T> {
        self.gather_literals()
    }

    fn essential_inputs(&self) -> BTreeSet<T> {
        let essentials =
            self.inputs()
                .into_iter()
                .rev()
                .enumerate()
                .filter_map(|(var_index, input)| {
                    let outputs_differ = (0..self.row_count())
                        .filter(|row_index| row_index & (1 << var_index) == 0)
                        .map(|row_index| (row_index, row_index ^ (1 << var_index)))
                        .any(|(row_index, flipped_row_index)| {
                            self.outputs[row_index] != self.outputs[flipped_row_index]
                        });

                    outputs_differ.then_some(input)
                });

        BTreeSet::from_iter(essentials)
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
        let inputs_kept = {
            let mut inputs_kept = self.inputs.clone();
            inputs_kept.retain(|input| !valuation.contains_key(input));
            inputs_kept
        };

        let mut outputs_kept = self
            .outputs
            .clone()
            .into_iter()
            .map(|output| (false, output))
            .collect::<Vec<_>>();

        for (input_index, input) in self.inputs.iter().rev().enumerate() {
            if let Some(target_variable_should_be_1) = valuation.get(input) {
                (0..self.row_count())
                    .filter(|row_index| {
                        let target_variable_is_one =
                            row_index & (1 << input_index) == (1 << input_index);
                        (*target_variable_should_be_1 && target_variable_is_one)
                            || (!*target_variable_should_be_1 && !target_variable_is_one)
                    })
                    .for_each(|row_index| {
                        outputs_kept[row_index].0 = true;
                    });
            }
        }

        TruthTable::new(
            inputs_kept,
            outputs_kept
                .into_iter()
                .filter_map(|(should_be_kept, output)| should_be_kept.then_some(output))
                .collect(),
        )
    }

    fn substitute(&self, mapping: &BTreeMap<T, Self>) -> Self {
        let substituted_inputs = BTreeSet::from_iter(mapping.keys().cloned());
        let substituting_inputs =
            BTreeSet::from_iter(mapping.values().flat_map(|value| value.gather_literals()));
        let final_inputs = &(&self.gather_literals() - &substituted_inputs) | &substituting_inputs;

        let mut outputs = vec![];

        for original_point in DomainIterator::from_count(final_inputs.len()) {
            let mut final_point = original_point.clone();
            let original_valuation =
                boolean_point_to_valuation(final_inputs.clone(), original_point.clone()).expect(
                    "Point should be from domain of the same dimension as the number of inputs",
                );

            // alter current valuation based on mappings
            for (index, (_bool, variable)) in
                original_point.into_iter().zip(&final_inputs).enumerate()
            {
                if let Some(substitution_table) = mapping.get(variable) {
                    let output = substitution_table.evaluate(&original_valuation);
                    final_point[index] = output;
                }
            }

            let final_valuation = boolean_point_to_valuation(final_inputs.clone(), final_point)
                .expect(
                    "Point should be from domain of the same dimension as the number of inputs",
                );
            let final_index = values_to_row_index(&self.inputs, &final_valuation);
            outputs.push(self.outputs[final_index])
        }

        TruthTable::new(Vec::from_iter(final_inputs), outputs)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::{bool, var, Expression};
    use crate::traits::{Evaluate, Implication};

    #[test]
    fn test_essential_inputs_all_inputs_ok() {
        let input_expression = var("a") & var("b");
        let input = TruthTable::from(input_expression);

        let actual = input.essential_inputs();
        let expected = BTreeSet::from_iter(["a".to_string(), "b".to_string()]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_essential_inputs_no_inputs_ok() {
        let input_expression = (var("a") & var("b")).imply(var("c") | !var("c"));
        let input = TruthTable::from(input_expression);

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
        );

        let actual = input.essential_inputs();
        let expected = BTreeSet::from_iter(["x", "y"]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_restrict_1st() {
        // "x,y,z,output\n",
        // "0,0,0,0\n",
        // "0,0,1,1\n",
        // "0,1,0,0\n",
        // "0,1,1,0\n",
        // "1,0,0,1\n", x
        // "1,0,1,0\n", x
        // "1,1,0,0\n", x
        // "1,1,1,0\n", x

        let input = TruthTable::new(
            vec!["x", "y", "z"],
            vec![false, true, false, false, true, false, false, false],
        );

        let actual = input.restrict(&BTreeMap::from([("x", true)]));
        let expected = TruthTable::new(vec!["y", "z"], vec![true, false, false, false]);

        assert_eq!(actual, expected);

        let actual = input.restrict(&BTreeMap::from([("x", false)]));
        let expected = TruthTable::new(vec!["y", "z"], vec![false, true, false, false]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_restrict_2nd() {
        // "x,y,z,output\n",
        // "0,0,0,0\n",
        // "0,0,1,1\n",
        // "0,1,0,0\n", x
        // "0,1,1,0\n", x
        // "1,0,0,1\n",
        // "1,0,1,0\n",
        // "1,1,0,0\n", x
        // "1,1,1,0\n", x

        let input = TruthTable::new(
            vec!["x", "y", "z"],
            vec![false, true, false, false, true, false, false, false],
        );

        let actual = input.restrict(&BTreeMap::from([("y", true)]));
        let expected = TruthTable::new(vec!["x", "z"], vec![false, false, false, false]);

        assert_eq!(actual, expected);

        let actual = input.restrict(&BTreeMap::from([("y", false)]));
        let expected = TruthTable::new(vec!["x", "z"], vec![false, true, true, false]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_restrict_3rd() {
        // "x,y,z,output\n",
        // "0,0,0,0\n",
        // "0,0,1,1\n", x
        // "0,1,0,0\n",
        // "0,1,1,0\n", x
        // "1,0,0,1\n",
        // "1,0,1,0\n", x
        // "1,1,0,0\n",
        // "1,1,1,0\n", x

        let input = TruthTable::new(
            vec!["x", "y", "z"],
            vec![false, true, false, false, true, false, false, false],
        );

        let actual = input.restrict(&BTreeMap::from([("z", true)]));
        let expected = TruthTable::new(vec!["x", "y"], vec![true, false, false, false]);

        assert_eq!(actual, expected);

        let actual = input.restrict(&BTreeMap::from([("z", false)]));
        let expected = TruthTable::new(vec!["x", "y"], vec![false, false, true, false]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_substitute_no_variables() {
        let input = TruthTable::from((var("a") | var("b")) & var("c"));
        let valuation = BTreeMap::new();

        let expected = input.clone();
        let actual = input.substitute(&valuation);

        assert_eq!(expected, actual);
        assert!(expected.semantic_eq(&actual));
        assert_eq!(actual.degree(), expected.degree());
    }

    #[test]
    fn test_substitute_variables_same_ok() {
        let input = TruthTable::from((var("a") | var("b")) & var("c") & !var("a") & bool(true));
        let mapping =
            BTreeMap::from_iter([("a".to_string(), TruthTable::from(var("a") | !var("b")))]);

        // cannot use `var("a") | !var("b") | var("b")` for defining expected here
        // since that collapses Or(Or(a, !b), b), which substitute doesn't do
        let expected = TruthTable::from(
            Expression::n_ary_or(&[var("a") | !var("b"), var("b")])
                & var("c")
                & !(var("a") | !var("b"))
                & bool(true),
        );
        let actual = input.substitute(&mapping);

        assert!(expected.semantic_eq(&actual));
        assert_eq!(expected, actual);
        assert_eq!(actual.degree(), expected.degree());
    }

    #[test]
    fn test_substitute_variables_added_ok() {
        let input = TruthTable::from((var("a") | var("b")) & var("c") & !var("a") & bool(true));

        let new_value = TruthTable::from(var("ddd") & (bool(false) | var("a")));
        let mapping = BTreeMap::from_iter([("a".to_string(), new_value.clone())]);

        // cannot use bitwise operators for defining expected here
        // since that collapses Or(Or(a, !b), b), which substitute doesn't do
        let expected = TruthTable::from(
            Expression::n_ary_or(&[var("ddd") & (bool(false) | var("a")), var("b")])
                & var("c")
                & !(var("ddd") & (bool(false) | var("a")))
                & bool(true),
        );
        let actual = input.substitute(&mapping);

        assert!(expected.semantic_eq(&actual));
        assert_eq!(expected, actual);

        assert_eq!(input.degree(), 3);
        assert_eq!(actual.degree(), 4);
        assert_eq!(actual.degree(), expected.degree());
    }

    #[test]
    fn test_substitute_variables_removed_ok() {
        let input = TruthTable::from((var("a") | var("b")) & var("c") & !var("a") & bool(true));

        let new_value = bool(false);
        let mapping = BTreeMap::from_iter([("a".to_string(), TruthTable::from(new_value))]);

        // cannot use bitwise operators for defining expected here
        // since that collapses Or(Or(a, !b), b), which substitute doesn't do
        let expected = TruthTable::from(
            Expression::n_ary_or(&[bool(false), var("b")]) & var("c") & !bool(false) & bool(true),
        );
        let actual = input.substitute(&mapping);

        assert!(expected.semantic_eq(&actual));
        // assert_eq!(expected, actual);

        assert_eq!(input.degree(), 3);
        assert_eq!(actual.degree(), 2);
        assert_eq!(actual.degree(), expected.degree());
    }

    #[test]
    fn test_existential_and_ok() {
        let target = "a".to_string();
        let target_set = BTreeSet::from([target.clone()]);
        let input = TruthTable::from(var(target.clone()) & var("b"));

        let actual = input.existential_quantification(target_set.clone());
        assert!(!actual.inputs().contains(&target.clone()));

        let evaluated_with_true = actual.evaluate(&btreeset_to_valuation(target_set.clone(), true));
        let evaluated_with_false = actual.evaluate(&btreeset_to_valuation(target_set, false));

        assert_eq!(evaluated_with_true, evaluated_with_false);
        assert!(!evaluated_with_true)
    }

    #[test]
    fn test_existential_or_ok() {
        let target = "a".to_string();
        let target_set = BTreeSet::from([target.clone()]);
        let input = TruthTable::from(var(target.clone()) | var("b"));

        let actual = input.existential_quantification(target_set.clone());
        assert!(!actual.inputs().contains(&target.clone()));

        let evaluated_with_true = actual.evaluate(&btreeset_to_valuation(target_set.clone(), true));
        let evaluated_with_false = actual.evaluate(&btreeset_to_valuation(target_set, false));

        assert_eq!(evaluated_with_true, evaluated_with_false);
        assert!(evaluated_with_true)
    }

    #[test]
    fn test_universal_and_ok() {
        let target = "a".to_string();
        let target_set = BTreeSet::from([target.clone()]);
        let input = TruthTable::from(var(target.clone()) & var("b"));

        let actual = input.universal_quantification(target_set.clone());
        assert!(!actual.inputs().contains(&target.clone()));

        let evaluated_with_true = actual.evaluate(&btreeset_to_valuation(target_set.clone(), true));
        let evaluated_with_false = actual.evaluate(&btreeset_to_valuation(target_set, false));

        assert_eq!(evaluated_with_true, evaluated_with_false);
        assert!(!evaluated_with_true)
    }

    #[test]
    fn test_universal_or_ok() {
        let target = "a".to_string();
        let target_set = BTreeSet::from([target.clone()]);
        let input = TruthTable::from(var(target.clone()) | var("b"));

        let actual = input.universal_quantification(target_set.clone());
        assert!(!actual.inputs().contains(&target.clone()));

        let evaluated_with_true = actual.evaluate(&btreeset_to_valuation(target_set.clone(), true));
        let evaluated_with_false = actual.evaluate(&btreeset_to_valuation(target_set, false));

        assert_eq!(evaluated_with_true, evaluated_with_false);
        assert!(!evaluated_with_true)
    }

    #[test]
    fn test_derivative_and_ok() {
        let target = "a".to_string();
        let target_set = BTreeSet::from([target.clone()]);
        let input = TruthTable::from(var(target.clone()) & var("b"));

        let actual = input.derivative(target_set.clone());
        assert!(!actual.inputs().contains(&target.clone()));

        let evaluated_with_true = actual.evaluate(&btreeset_to_valuation(target_set.clone(), true));
        let evaluated_with_false = actual.evaluate(&btreeset_to_valuation(target_set, false));

        assert_eq!(evaluated_with_true, evaluated_with_false);
        assert!(!evaluated_with_true)
    }

    #[test]
    fn test_derivative_or_ok() {
        let target = "a".to_string();
        let target_set = BTreeSet::from([target.clone()]);
        let input = TruthTable::from(var(target.clone()) | var("b"));

        let actual = input.derivative(target_set.clone());
        assert!(!actual.inputs().contains(&target.clone()));

        let evaluated_with_true = actual.evaluate(&btreeset_to_valuation(target_set.clone(), true));
        let evaluated_with_false = actual.evaluate(&btreeset_to_valuation(target_set, false));

        assert_eq!(evaluated_with_true, evaluated_with_false);
        assert!(evaluated_with_true)
    }

    #[test]
    fn test_derivative_xor_ok() {
        let target = "a".to_string();
        let target_set = BTreeSet::from([target.clone()]);
        let input = TruthTable::from(var(target.clone()) ^ var("b"));

        let actual = input.derivative(target_set.clone());
        assert!(!actual.inputs().contains(&target.clone()));

        let evaluated_with_true = actual.evaluate(&btreeset_to_valuation(target_set.clone(), true));
        let evaluated_with_false = actual.evaluate(&btreeset_to_valuation(target_set, false));

        assert_eq!(evaluated_with_true, evaluated_with_false);
        assert!(evaluated_with_true)
    }
}
