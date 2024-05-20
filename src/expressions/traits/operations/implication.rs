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
    use crate::expressions::{bool, var};
    use crate::traits::{Implication, SemanticEq};

    #[test]
    fn test_imply_syntactic_ok() {
        let actual = var("a").imply(var("b"));
        let expected_alternative_implication = !(var("a") & !var("b"));
        let expected_actual_implication = !var("a") | var("b");

        assert!(actual.semantic_eq(&expected_alternative_implication));
        assert_eq!(actual, expected_actual_implication);
    }

    #[test]
    fn test_imply_semantic_ok() {
        assert!(bool(false).imply(bool(false)).semantic_eq(&bool(true)));
        assert!(bool(false).imply(bool(true)).semantic_eq(&bool(true)));
        assert!(bool(true).imply(bool(false)).semantic_eq(&bool(false)));
        assert!(bool(true).imply(bool(true)).semantic_eq(&bool(true)));

        assert!(var("x").imply(bool(false)).semantic_eq(&!var("x")));
        assert!(bool(true).imply(var("x")).semantic_eq(&var("x")));
        assert!(!var("x").imply(var("x")).semantic_eq(&var("x")));
    }
}
