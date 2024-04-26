use crate::expressions::Expression;
use std::fmt::Debug;
use std::ops::BitXor;

impl<T: Debug + Clone + Eq + Ord> BitXor<Expression<T>> for Expression<T> {
    type Output = Expression<T>;

    fn bitxor(self, other: Expression<T>) -> Self::Output {
        (self.clone() | other.clone()) & !(self & other)
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::var;
    use crate::traits::SemanticEq;

    #[test]
    fn test_xor() {
        let actual = var("a") ^ var("b");

        let expected_alternative_xor = (var("a") & !var("b")) | (!var("a") & var("b"));
        let expected_actual_xor = (var("a") | var("b")) & !(var("a") & var("b"));

        assert!(actual.semantic_eq(&expected_alternative_xor));
        assert_eq!(actual, expected_actual_xor);
    }

    #[test]
    fn test_xor_merge() {
        let actual = (var("a") | var("b")) ^ (var("c") & var("d"));

        let expected_alternative_xor = ((var("a") | var("b")) & !(var("c") & var("d")))
            | (!(var("a") | var("b")) & (var("c") & var("d")));
        let expected_actual_xor = ((var("a") | var("b")) | (var("c") & var("d")))
            & !((var("a") | var("b")) & (var("c") & var("d")));

        assert!(actual.semantic_eq(&expected_alternative_xor));
        assert_eq!(actual, expected_actual_xor);
    }
}
