use crate::expressions::Expression;
use crate::expressions::ExpressionNode::Or;
use std::fmt::Debug;

impl<T: Debug + Clone + Eq + Ord> std::ops::BitOr<Expression<T>> for Expression<T> {
    type Output = Expression<T>;

    fn bitor(self, rhs: Expression<T>) -> Self::Output {
        let mut es = Vec::new();
        match (self.node(), rhs.node()) {
            (Or(es1), Or(es2)) => {
                es.extend(es1.iter().cloned());
                es.extend(es2.iter().cloned());
            }
            (Or(es1), _other) => {
                es.extend(es1.iter().cloned());
                es.push(rhs);
            }
            (_other, Or(es2)) => {
                es.push(self);
                es.extend(es2.iter().cloned());
            }
            _ => {
                es.push(self);
                es.push(rhs);
            }
        }

        Or(es).into()
    }
}
