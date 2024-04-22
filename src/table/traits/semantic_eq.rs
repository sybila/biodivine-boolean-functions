use crate::table::TruthTable;
use crate::traits::{Evaluate, PowerSet, SemanticEq};
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;

impl<TLiteral: Debug + Display + Clone + Eq + Hash + Ord> SemanticEq<TLiteral>
    for TruthTable<TLiteral>
{
    fn semantic_eq(&self, other: &Self) -> bool {
        let mut inputs_from_both =
            Vec::from_iter(self.inputs.clone().into_iter().chain(other.inputs.clone()));

        inputs_from_both.sort();
        inputs_from_both.dedup();

        let all_valuations = TruthTable::generate_power_set(HashSet::from_iter(inputs_from_both));

        all_valuations
            .into_iter()
            .all(|valuation| self.evaluate(&valuation) == other.evaluate(&valuation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_eq_same_variables_ok_equal() {
        let input_1 = TruthTable::new(vec!["a", "b"], vec![true, false, true, false]);
        let input_2 = TruthTable::new(vec!["a", "b"], vec![true, false, true, false]);

        assert!(input_1.semantic_eq(&input_2));
    }

    #[test]
    fn test_semantic_eq_same_variables_ok_not_equal() {
        let input_1 = TruthTable::new(vec!["a", "b"], vec![true, false, true, false]);
        let input_2 = TruthTable::new(vec!["a", "b"], vec![true, true, true, false]);

        assert!(!input_1.semantic_eq(&input_2));
    }

    #[test]
    fn test_semantic_eq_additional_variable_useless_ok_equal() {
        let input_1 = TruthTable::new(vec!["a", "b"], vec![true, false, true, false]);
        // c being 0 or 1 doesn't change output for given a and b
        let input_2 = TruthTable::new(
            vec!["a", "b", "c"],
            vec![true, true, false, false, true, true, false, false],
        );

        assert!(input_1.semantic_eq(&input_2));
    }

    #[test]
    fn test_semantic_eq_different_variables_ok_not_equal() {
        let input_1 = TruthTable::new(vec!["a", "b"], vec![true, false, true, false]);
        let input_2 = TruthTable::new(vec!["a", "c"], vec![true, false, true, false]);

        assert!(!input_1.semantic_eq(&input_2));
    }
}
