use crate::expressions::Expression;
use crate::expressions::ExpressionNode::Not;
use std::fmt::Debug;
use std::hash::Hash;

impl<T: Debug + Clone + Eq + Hash> std::ops::Not for Expression<T> {
    type Output = Expression<T>;

    fn not(self) -> Self::Output {
        Not(self).into()
    }
}
