use crate::expressions::Expression;
use crate::traits::Equality;
use std::fmt::Debug;

impl<T: Debug + Clone + Ord> Equality for Expression<T> {
    type Output = Expression<T>;

    fn iff(self, rhs: Self) -> <Self as Equality<Self>>::Output {
        (self.clone() & rhs.clone()) | (!self & !rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::{bool, var};
    use crate::traits::{Equality, SemanticEq};

    #[test]
    fn test_iff_syntactic_ok() {
        let actual = var("a").iff(var("b"));
        let expected_alternative_implication = (!var("a") | var("b")) & (var("a") | !var("b"));
        let expected_actual_implication = (var("a") & var("b")) | (!var("a") & !var("b"));

        assert!(actual.semantic_eq(&expected_alternative_implication));
        assert_eq!(actual, expected_actual_implication);
    }

    #[test]
    fn test_iff_semantic_ok() {
        assert!(bool(false).iff(bool(false)).semantic_eq(&bool(true)));
        assert!(bool(false).iff(bool(true)).semantic_eq(&bool(false)));
        assert!(bool(true).iff(bool(false)).semantic_eq(&bool(false)));
        assert!(bool(true).iff(bool(true)).semantic_eq(&bool(true)));

        assert!(var("x").iff(bool(false)).semantic_eq(&!var("x")));
        assert!(bool(true).iff(var("x")).semantic_eq(&var("x")));
        assert!(var("x").iff(!var("x")).semantic_eq(&bool(false)));
    }
}
