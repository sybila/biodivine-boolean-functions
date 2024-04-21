use crate::expressions::{Expression, ExpressionNode::And};
use std::fmt::Debug;
use std::hash::Hash;

impl<T: Debug + Clone + Eq + Hash> std::ops::BitAnd<Expression<T>> for Expression<T> {
    type Output = Expression<T>;

    fn bitand(self, rhs: Expression<T>) -> Self::Output {
        let mut es = Vec::new();
        match (self.node(), rhs.node()) {
            (And(es1), And(es2)) => {
                es.extend(es1.iter().cloned());
                es.extend(es2.iter().cloned());
            }
            (And(es1), _other) => {
                es.extend(es1.iter().cloned());
                es.push(rhs);
            }
            (_other, And(es2)) => {
                es.push(self);
                es.extend(es2.iter().cloned());
            }
            _ => {
                es.push(self);
                es.push(rhs);
            }
        }

        And(es).into()
    }
}
