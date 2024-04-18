mod display;
mod evaluate;
mod gather_literals;
mod parse;
mod power_set;
mod semantic_eq;

#[cfg(test)]
mod tests {
    use crate::expressions::Expression;
    use crate::expressions::ExpressionNode::{And, Not, Or};
    use std::fmt::Debug;
    use std::hash::Hash;

    /*
       The following traits are only implemented in test builds because it is not yet
       clear whether we should provide them as part of the stable Rust API (overloading
       operators for non-copy types often isn't as useful as it might initially seem).

       However, in tests, we can use them to simplify expression construction.
    */

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

    impl<T: Debug + Clone + Eq + Hash> std::ops::BitOr<Expression<T>> for Expression<T> {
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

    impl<T: Debug + Clone + Eq + Hash> std::ops::Not for Expression<T> {
        type Output = Expression<T>;

        fn not(self) -> Self::Output {
            Not(self).into()
        }
    }
}
