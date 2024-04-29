use crate::table::TruthTable;
use crate::traits::{Evaluate, GatherLiterals, PowerSet, SemanticEq};
use std::collections::BTreeSet;
use std::fmt::Debug;

impl<TLiteral: Debug + Clone + Eq + Ord> SemanticEq<TLiteral> for TruthTable<TLiteral> {
    fn semantic_eq(&self, other: &Self) -> bool {
        let inputs_from_both = BTreeSet::from_iter(
            self.gather_literals()
                .union(&other.gather_literals())
                .cloned(),
        );

        let all_valuations = Self::generate_arbitrary_power_set(inputs_from_both);

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
