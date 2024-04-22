use crate::expressions::Expression;
use crate::expressions::ExpressionNode::{And, Constant, Literal, Not, Or};
use crate::traits::Evaluate;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{BitAnd, BitOr};

impl<TLiteral: Debug + Clone + Eq + Hash> Evaluate<TLiteral> for Expression<TLiteral> {
    fn evaluate_with_default(
        &self,
        literal_values: &HashMap<TLiteral, bool>,
        default_value: bool,
    ) -> bool {
        match self.node() {
            Literal(t) => *literal_values.get(t).unwrap_or(&default_value),
            Constant(value) => *value,
            And(values) => values
                .iter()
                .all(|e| e.evaluate_with_default(literal_values, default_value)),
            Or(values) => values
                .iter()
                .any(|e| e.evaluate_with_default(literal_values, default_value)),
            Not(x) => !x.evaluate_with_default(literal_values, default_value),
        }
    }

    fn evaluate_checked(
        &self,
        literal_values: &HashMap<TLiteral, bool>,
    ) -> Result<bool, Vec<TLiteral>> {
        let mut errors = vec![];

        let ok_result = self.evaluate_checked_rec(literal_values, &mut errors);

        if errors.is_empty() {
            Ok(ok_result)
        } else {
            Err(errors)
        }
    }
}

impl<TLiteral: Debug + Clone + Eq + Hash> Expression<TLiteral> {
    fn evaluate_checked_rec(
        &self,
        literal_values: &HashMap<TLiteral, bool>,
        err_values: &mut Vec<TLiteral>,
    ) -> bool {
        match self.node() {
            Literal(t) => match literal_values.get(t) {
                None => {
                    err_values.push(t.clone());
                    true // will be unused
                }
                Some(valuation) => *valuation,
            },
            Constant(value) => *value,
            Not(inner) => !inner.evaluate_checked_rec(literal_values, err_values),
            And(expressions) => expressions
                .iter()
                .map(|e| e.evaluate_checked_rec(literal_values, err_values))
                .fold(true, BitAnd::bitand),
            Or(expressions) => expressions
                .iter()
                .map(|e| e.evaluate_checked_rec(literal_values, err_values))
                .fold(false, BitOr::bitor),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::{bool as boolFn, var};

    #[test]
    fn test_evaluate_variables_match_and_ok() {
        let input = !(var("a") & var("b"));

        let pairs = [("a", true), ("b", true)];
        let mapping = HashMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = false;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, true), false);
        assert_eq!(input.evaluate_checked(&mapping), Ok(false));
    }

    #[test]
    fn test_evaluate_too_many_variables_and_ok() {
        let input = !(var("a") & var("b"));

        let pairs = [("a", true), ("b", true), ("c", false)];
        let mapping = HashMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = false;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, true), false);
        assert_eq!(input.evaluate_checked(&mapping), Ok(false));
    }

    #[test]
    fn test_evaluate_too_few_variables_and_ok() {
        let input = !(var("a") & var("b"));

        let pairs = [("a", true)];
        let mapping = HashMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = true;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, true), false);
        assert_eq!(input.evaluate_checked(&mapping), Err(vec!["b".to_string()]));
    }

    #[test]
    fn test_evaluate_variables_match_or_ok() {
        let input = var("a") | var("b");

        let pairs = [("a", false), ("b", false)];
        let mapping = HashMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = false;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, true), false);
        assert_eq!(input.evaluate_checked(&mapping), Ok(false));
    }

    #[test]
    fn test_evaluate_too_many_variables_or_ok() {
        let input = var("a") | var("b");

        let pairs = [("a", false), ("b", false), ("c", true)];
        let mapping = HashMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = false;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, true), false);
        assert_eq!(input.evaluate_checked(&mapping), Ok(false));
    }

    #[test]
    fn test_evaluate_too_few_variables_or_ok() {
        let input = var("a") | var("b");

        let pairs = [("a", false)];
        let mapping = HashMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = false;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, true), true);
        assert_eq!(input.evaluate_checked(&mapping), Err(vec!["b".to_string()]));
    }

    #[test]
    fn test_evaluate_variables_match_const_ok() {
        let input = boolFn(true) | boolFn(false);

        let pairs: [(&str, bool); 0] = [];
        let mapping = HashMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = true;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, true), true);
        assert_eq!(input.evaluate_checked(&mapping), Ok(true));
    }

    #[test]
    fn test_evaluate_too_many_variables_const_ok() {
        let input = boolFn(true) | boolFn(false);

        let pairs = [("a", false), ("b", false), ("c", true)];
        let mapping = HashMap::<String, bool>::from_iter(
            pairs.map(|(name, value)| (name.to_string(), value)),
        );
        let expected_base = true;

        assert_eq!(input.evaluate(&mapping), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, false), expected_base);
        assert_eq!(input.evaluate_with_default(&mapping, true), true);
        assert_eq!(input.evaluate_checked(&mapping), Ok(true));
    }
}
