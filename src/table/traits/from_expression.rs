use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::expressions::Expression;
use crate::table::utils::values_to_row_index;
use crate::table::TruthTable;
use crate::traits::{Evaluate, GatherLiterals, PowerSet};

impl<TLiteral: Debug + Display + Clone + Eq + Ord + Hash> From<Expression<TLiteral>>
    for TruthTable<TLiteral>
{
    fn from(expression: Expression<TLiteral>) -> Self {
        Self::from(&expression)
    }
}

impl<TLiteral: Debug + Display + Clone + Eq + Ord + Hash> From<&Expression<TLiteral>>
    for TruthTable<TLiteral>
{
    fn from(expression: &Expression<TLiteral>) -> Self {
        let literals = expression.gather_literals();
        let all_options = Expression::generate_power_set(literals.clone());
        let literals = {
            let mut literals = Vec::from_iter(literals);
            literals.sort();
            literals
        };

        let mut outputs = vec![false; 2_usize.pow(literals.len() as u32)];
        for option in all_options {
            let index = values_to_row_index(&literals, &option);
            let value = expression.evaluate(&option);

            outputs[index] = value;
        }

        Self::new(literals, outputs)
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::Expression;
    use crate::expressions::{bool, var, vars};

    use super::*;

    #[test]
    fn test_from_expression_literals_sorted_ok() {
        let input = Expression::n_ary_and(&vars(["x3", "x2", "x1", "x3", "x0"]));

        let expected = vec!["x0", "x1", "x2", "x3"];
        let actual = TruthTable::from(input).inputs;

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_from_expression_basic_case_x0_one_variable_ok() {
        let input = var("x0");

        let actual = TruthTable::from(input);
        let expected = TruthTable {
            inputs: vec!["x0".to_string()],
            outputs: vec![false, true],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_expression_basic_case_not_x0_one_variable_ok() {
        let input = !var("x0");

        let actual = TruthTable::from(input);
        let expected = TruthTable {
            inputs: vec!["x0".to_string()],
            outputs: vec![true, false],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_expression_basic_case_x0_two_variables_ok() {
        let input = var("x0") & (var("x1") | !var("x1")); // tautology

        let actual = TruthTable::from(input);
        let expected = TruthTable {
            inputs: vec!["x0".to_string(), "x1".to_string()],
            outputs: vec![false, false, true, true],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_expression_basic_case_not_x0_two_variables_ok() {
        let input = !var("x0") & (var("x1") | !var("x1")); // tautology

        let actual = TruthTable::from(input);
        let expected = TruthTable {
            inputs: vec!["x0".to_string(), "x1".to_string()],
            outputs: vec![true, true, false, false],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_expression_basic_case_and_ok() {
        let input = var("x0") & var("x1");

        let actual = TruthTable::from(input);
        let expected = TruthTable {
            inputs: vec!["x0".to_string(), "x1".to_string()],
            outputs: vec![false, false, false, true],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_expression_basic_case_or_ok() {
        let input = var("x0") | var("x1");

        let actual = TruthTable::from(input);
        let expected = TruthTable {
            inputs: vec!["x0".to_string(), "x1".to_string()],
            outputs: vec![false, true, true, true],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_expression_variableless_tautology_ok() {
        let input = bool(true) | bool(false);

        let actual = TruthTable::from(input);

        println!("{}", actual);
        let expected = TruthTable {
            inputs: vec![],
            outputs: vec![true],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_expression_variableless_contradiction_ok() {
        let input = bool(true) & bool(false);

        let actual = TruthTable::from(input);

        println!("{}", actual);
        let expected = TruthTable {
            inputs: vec![],
            outputs: vec![false],
        };

        assert_eq!(actual, expected);
    }
}
