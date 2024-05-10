use crate::bdd::Bdd;
use crate::expressions::{Expression, ExpressionNode};
use std::fmt::Debug;

impl<T: Debug + Clone + Ord> From<Bdd<T>> for Expression<T> {
    fn from(value: Bdd<T>) -> Self {
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
