use crate::traits::Evaluate;
use std::collections::BTreeMap;
use std::fmt::Debug;

use crate::bdd::Bdd;
use biodivine_lib_bdd::BddValuation;
use itertools::Itertools;

impl<TLiteral: Debug + Clone + Eq + Ord> Evaluate<TLiteral> for Bdd<TLiteral> {
    fn evaluate_with_default(
        &self,
        literal_values: &BTreeMap<TLiteral, bool>,
        default_value: bool,
    ) -> bool {
        let v = self
            .inputs
            .iter()
            .map(|input| {
                if let Some(var_is_true) = literal_values.get(input) {
                    *var_is_true
                } else {
                    default_value
                }
            })
            .collect_vec();

        self.bdd.eval_in(&BddValuation::new(v))
    }

    fn evaluate_checked(
        &self,
        literal_values: &BTreeMap<TLiteral, bool>,
    ) -> Result<bool, Vec<TLiteral>> {
        let (point, errors): (Vec<_>, Vec<_>) = self
            .inputs
            .iter()
            .map(|input| {
                if let Some(var_is_true) = literal_values.get(input) {
                    Ok(*var_is_true)
                } else {
                    Err(input.clone())
                }
            })
            .partition_result();

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(self.bdd.eval_in(&BddValuation::new(point)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::{bool as boolFn, var};

    #[test]
    fn test_evaluate_variables_match_and_ok() {
        let input = Bdd::try_from(!(var("a") & var("b"))).expect("Should not panic here");

        let pairs = [("a", true), ("b", true)];
        let mapping = BTreeMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = false;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert!(!input.evaluate_with_default(&mapping, true));
        assert_eq!(input.evaluate_checked(&mapping), Ok(false));
    }

    #[test]
    fn test_evaluate_too_many_variables_and_ok() {
        let input = Bdd::try_from(!(var("a") & var("b"))).expect("Should not panic here");

        let pairs = [("a", true), ("b", true), ("c", false)];
        let mapping = BTreeMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = false;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert!(!input.evaluate_with_default(&mapping, true));
        assert_eq!(input.evaluate_checked(&mapping), Ok(false));
    }

    #[test]
    fn test_evaluate_too_few_variables_and_ok() {
        let input = Bdd::try_from(!(var("a") & var("b"))).expect("Should not panic here");

        let pairs = [("a", true)];
        let mapping = BTreeMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = true;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert!(!input.evaluate_with_default(&mapping, true));
        assert_eq!(input.evaluate_checked(&mapping), Err(vec!["b".to_string()]));
    }

    #[test]
    fn test_evaluate_variables_match_or_ok() {
        let input = Bdd::try_from(var("a") | var("b")).expect("Should not panic here");

        let pairs = [("a", false), ("b", false)];
        let mapping = BTreeMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = false;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert!(!input.evaluate_with_default(&mapping, true));
        assert_eq!(input.evaluate_checked(&mapping), Ok(false));
    }

    #[test]
    fn test_evaluate_too_many_variables_or_ok() {
        let input = Bdd::try_from(var("a") | var("b")).expect("Should not panic here");

        let pairs = [("a", false), ("b", false), ("c", true)];
        let mapping = BTreeMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = false;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert!(!input.evaluate_with_default(&mapping, true));
        assert_eq!(input.evaluate_checked(&mapping), Ok(false));
    }

    #[test]
    fn test_evaluate_too_few_variables_or_ok() {
        let input = Bdd::try_from(var("a") | var("b")).expect("Should not panic here");

        let pairs = [("a", false)];
        let mapping = BTreeMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = false;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert!(input.evaluate_with_default(&mapping, true));
        assert_eq!(input.evaluate_checked(&mapping), Err(vec!["b".to_string()]));
    }

    #[test]
    fn test_evaluate_variables_match_const_ok() {
        let input = Bdd::try_from(boolFn(true) | boolFn(false)).expect("Should not panic here");

        let pairs: [(&str, bool); 0] = [];
        let mapping = BTreeMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = true;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert!(input.evaluate_with_default(&mapping, true));
        assert_eq!(input.evaluate_checked(&mapping), Ok(true));
    }

    #[test]
    fn test_evaluate_too_many_variables_const_ok() {
        let input = Bdd::try_from(boolFn(true) | boolFn(false)).expect("Should not panic here");

        let pairs = [("a", false), ("b", false), ("c", true)];
        let mapping = BTreeMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = true;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert!(input.evaluate_with_default(&mapping, true));
        assert_eq!(input.evaluate_checked(&mapping), Ok(true));
    }
}
