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
