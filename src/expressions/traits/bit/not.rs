use crate::expressions::Expression;
use crate::expressions::ExpressionNode::Not;
use std::fmt::Debug;

impl<T: Debug + Clone + Eq + Ord> std::ops::Not for Expression<T> {
    type Output = Expression<T>;

    fn not(self) -> Self::Output {
        Not(self).into()
    }
}
