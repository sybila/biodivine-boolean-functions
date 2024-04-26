use crate::expressions::Expression;
use crate::traits::Implication;
use std::fmt::Debug;

impl<T: Debug + Clone + Eq + Ord> Implication<Expression<T>> for Expression<T> {
    type Output = Expression<T>;

    fn imply(self, rhs: Expression<T>) -> <Self as Implication>::Output {
        !self | rhs
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::var;
    use crate::traits::{Implication, SemanticEq};

    #[test]
    fn test_imply_ok() {
        let actual = var("a").imply(var("b"));
        let expected_alternative_implication = !(var("a") & !var("b"));
        let expected_actual_implication = !var("a") | var("b");

        assert!(actual.semantic_eq(&expected_alternative_implication));
        assert_eq!(actual, expected_actual_implication);
    }
}
