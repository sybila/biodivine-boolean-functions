use crate::iterators::{DomainIterator, ImageIterator, RelationIterator, SupportIterator};
use crate::table::TruthTable;
use crate::traits::{BooleanFunction, BooleanValuation, GatherLiterals};
use itertools::Itertools;
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
                    let variable_exponent = 2usize.pow(var_index as u32);

                    let outputs_differ = (0..self.row_count())
                        .chunks(variable_exponent)
                        .into_iter()
                        .enumerate()
                        .filter(|(chunk_index, _row_indexes)| *chunk_index % 2 == 0)
                        .any(|(_chunk_index, mut row_indexes)| {
                            row_indexes.any(|row_index| {
                                self.outputs[row_index]
                                    != self.outputs[row_index + variable_exponent]
                            })
                        });

                    if outputs_differ {
                        Some(input)
                    } else {
                        None
                    }
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

    fn restrict(&self, _valuation: &BooleanValuation<T>) -> Self {
        todo!()
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
}
