use crate::bdd::Bdd;
use crate::expressions::Expression;
use crate::expressions::ExpressionNode::{And, Constant, Literal, Not, Or};
use crate::traits::GatherLiterals;
use biodivine_lib_bdd::{Bdd as InnerBdd, BddVariable, BddVariableSet};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::num::TryFromIntError;

impl<T: Debug + Clone + Ord> TryFrom<Expression<T>> for Bdd<T> {
    type Error = TryFromIntError;

    fn try_from(value: Expression<T>) -> Result<Self, Self::Error> {
        let literals = value.gather_literals();
        let mapping = BTreeMap::from_iter(
            literals
                .clone()
                .into_iter()
                .enumerate()
                .map(|(index, var)| (var, index)),
        );
        let literal_set = Self::make_inner_variable_set(literals.clone())?;

        Ok(Bdd::new(
            try_from_rec(&value, &literal_set, &mapping),
            mapping.into_keys().collect(),
        ))
    }
}

fn try_from_rec<T: Debug + Clone + Ord>(
    expression: &Expression<T>,
    literal_set: &BddVariableSet,
    literal_index_map: &BTreeMap<T, usize>,
) -> InnerBdd {
    match expression.node() {
        Literal(t) => {
            literal_set.mk_var(BddVariable::from_index(*literal_index_map.get(t).expect(
                "Literal index map should be created from literals occurring in expression",
            )))
        }
        Constant(value) => {
            if *value {
                literal_set.mk_true()
            } else {
                literal_set.mk_false()
            }
        }

        Not(x) => try_from_rec(x, literal_set, literal_index_map).not(),
        And(values) => values
            .iter()
            .map(|e| try_from_rec(e, literal_set, literal_index_map))
            .reduce(|current, next| current.and(&next))
            .unwrap_or_else(|| literal_set.mk_true()),
        Or(values) => values
            .iter()
            .map(|e| try_from_rec(e, literal_set, literal_index_map))
            .reduce(|current, next| current.or(&next))
            .unwrap_or_else(|| literal_set.mk_false()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::BooleanFunction;
    use std::str::FromStr;
    #[test]
    fn test_bdd_from_expression() {
        let exp_string = "(b | a & c) & !a".to_string();
        let input = Expression::from_str(&exp_string).unwrap();
        let actual = Bdd::try_from(input).unwrap();

        let inputs = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let var_set = BddVariableSet::from(inputs.clone());
        let inner_bdd = var_set.eval_expression_string(&exp_string);
        let expected = Bdd::new(inner_bdd, inputs);

        assert!(actual.is_equivalent(&expected))
    }
    #[test]
    fn test_bdd_from_expression_n_ary() {
        let exp_string = "(a | b | c) | (!a & !b & !c)".to_string();
        let input = Expression::from_str(&exp_string).unwrap();
        let actual = Bdd::try_from(input).unwrap();

        let inputs = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let var_set = BddVariableSet::from(inputs.clone());
        let inner_bdd = var_set.eval_expression_string(&exp_string);
        let expected = Bdd::new(inner_bdd, inputs);

        assert!(actual.is_equivalent(&expected))
    }
}
