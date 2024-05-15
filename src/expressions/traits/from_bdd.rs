use crate::bdd::Bdd;
use crate::expressions::{Expression, ExpressionNode};
use std::fmt::Debug;

impl<T: Debug + Clone + Ord> From<Bdd<T>> for Expression<T> {
    fn from(value: Bdd<T>) -> Self {
        if value.inner().is_true() {
            return ExpressionNode::Constant(true).into();
        } else if value.inner().is_false() {
            return ExpressionNode::Constant(false).into();
        }

        let and_expressions = value
            .inner()
            .to_optimized_dnf()
            .into_iter()
            .map(|valuation| {
                let and_inner = valuation
                    .to_values()
                    .into_iter()
                    .map(|(literal, literal_value)| {
                        let var = value
                            .map_var_inner_to_outer(literal)
                            .expect("Literal should come from bdd instance");

                        if literal_value {
                            ExpressionNode::Literal(var).into()
                        } else {
                            Expression::negate(&ExpressionNode::Literal(var).into())
                        }
                    })
                    .collect::<Vec<_>>();

                Expression::n_ary_and(&and_inner)
            })
            .collect::<Vec<_>>();

        Expression::n_ary_or(&and_expressions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{BooleanFunction, SemanticEq};
    use biodivine_lib_bdd::BddVariableSet;
    use rstest::rstest;
    use std::str::FromStr;

    #[test]
    fn test_expression_from_bdd_standard() {
        let exp_string = "(b | a & c) & !a".to_string();
        let inputs = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let var_set = BddVariableSet::from(inputs.clone());
        let inner_bdd = var_set.eval_expression_string(&exp_string);
        let bdd = Bdd::new(inner_bdd, inputs);

        let expected = Expression::from_str(&exp_string).unwrap();
        let actual = Expression::from(bdd);

        assert!(
            actual.semantic_eq(&expected),
            "expected: `{expected}`,\nactual: `{actual}`"
        );
    }

    #[test]
    fn test_expression_from_bdd_n_ary() {
        let exp_string = "(a | b | c) | (!a & !b & !c)".to_string();
        let inputs = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let var_set = BddVariableSet::from(inputs.clone());
        let inner_bdd = var_set.eval_expression_string(&exp_string);
        let bdd = Bdd::new(inner_bdd, inputs);

        let expected = Expression::from_str(&exp_string).unwrap();
        let actual = Expression::from(bdd);

        assert!(
            actual.semantic_eq(&expected),
            "expected: `{expected}`,\nactual: `{actual}`"
        );
    }

    #[rstest]
    fn test_bdd_from_expression_const(#[values("true", "false")] exp_string: &str) {
        let inputs = vec![];
        let var_set = BddVariableSet::from(inputs.clone());
        let inner_bdd = var_set.eval_expression_string(exp_string);
        let bdd = Bdd::new(inner_bdd, inputs);

        let expected = Expression::from_str(exp_string).unwrap();
        let actual = Expression::from(bdd);

        println!("{expected}, {actual}");
        assert!(actual.is_equivalent(&expected));
        assert_eq!(actual, expected);
    }
}
