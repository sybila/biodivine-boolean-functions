use crate::expressions::Expression;
use crate::expressions::ExpressionNode;
use std::fmt::Debug;
use std::ops::Not;

impl<T: Debug + Clone + Eq + Ord> Not for Expression<T> {
    type Output = Expression<T>;

    fn not(self) -> Self::Output {
        ExpressionNode::Not(self).into()
    }
}
