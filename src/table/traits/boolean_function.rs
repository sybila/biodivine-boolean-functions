use crate::iterators::{DomainIterator, ImageIterator, RelationIterator, SupportIterator};
use crate::table::TruthTable;
use crate::traits::{BooleanFunction, BooleanValuation, GatherLiterals};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

impl<T: Debug + Clone + Ord + 'static> BooleanFunction<T> for TruthTable<T> {
    type DomainIterator = DomainIterator;
    type RangeIterator = ImageIterator<T>;
    type RelationIterator = RelationIterator<T>;
    type SupportIterator = SupportIterator<T>;

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

    fn substitute(&self, _mapping: &BTreeMap<T, Self>) -> Self {
        todo!()
    }

    fn existential_quantification(&self, _variables: BTreeSet<T>) -> Self {
        todo!()
    }

    fn universal_quantification(&self, _variables: BTreeSet<T>) -> Self {
        todo!()
    }

    fn derivative(&self, _variables: BTreeSet<T>) -> Self {
        todo!()
    }

    fn is_equivalent(&self, _other: &Self) -> bool {
        todo!()
    }

    fn is_implied_by(&self, _other: &Self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::var;
    use crate::traits::Implication;

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
}
