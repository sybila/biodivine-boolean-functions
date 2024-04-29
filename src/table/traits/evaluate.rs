use std::collections::BTreeMap;
use std::fmt::Debug;

use crate::table::utils::{values_to_row_index_checked, values_to_row_index_with_default};
use crate::table::TruthTable;
use crate::traits::Evaluate;

impl<TLiteral: Debug + Clone + Eq + Ord> Evaluate<TLiteral> for TruthTable<TLiteral> {
    fn evaluate_with_default(
        &self,
        literal_values: &BTreeMap<TLiteral, bool>,
        default_value: bool,
    ) -> bool {
        let index = values_to_row_index_with_default(&self.inputs, literal_values, default_value);

        self.outputs[index]
    }

    fn evaluate_checked(
        &self,
        literal_values: &BTreeMap<TLiteral, bool>,
    ) -> Result<bool, Vec<TLiteral>> {
        let index = values_to_row_index_checked(&self.inputs, literal_values)?;

        Ok(self.outputs[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_variables_match_ok() {
        let input_table = TruthTable::new(vec!["a", "b"], vec![true, true, true, false]);

        let pairs = [("a", true), ("b", true)];
        let mapping = BTreeMap::<&str, bool>::from_iter(pairs);

        assert!(!input_table.evaluate(&mapping));
        assert!(!input_table.evaluate_with_default(&mapping, true));
        assert_eq!(input_table.evaluate_checked(&mapping), Ok(false));
    }

    #[test]
    fn test_evaluate_too_many_variables_ok() {
        let input_table = TruthTable::new(vec!["a", "b"], vec![true, true, true, false]);

        let pairs = [("a", true), ("b", true), ("c", false)];
        let mapping = BTreeMap::<&str, bool>::from_iter(pairs);

        assert!(!input_table.evaluate(&mapping));
        assert!(!input_table.evaluate_with_default(&mapping, true));
        assert_eq!(input_table.evaluate_checked(&mapping), Ok(false));
    }

    #[test]
    fn test_evaluate_too_few_variables_ok() {
        let input_table = TruthTable::new(vec!["a", "b"], vec![true, true, true, false]);

        let pairs = [("a", true)];
        let mapping = BTreeMap::<&str, bool>::from_iter(pairs);

        assert!(input_table.evaluate(&mapping));
        assert!(!input_table.evaluate_with_default(&mapping, true));
        assert_eq!(input_table.evaluate_checked(&mapping), Err(vec!["b"]));
    }
}
