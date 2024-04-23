use crate::expressions::structs::expression::Expression;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExpressionNode<T>
where
    T: Debug + Clone + Eq + Ord,
{
    Literal(T),
    Constant(bool),
    Not(Expression<T>),
    And(Vec<Expression<T>>),
    Or(Vec<Expression<T>>),
}
