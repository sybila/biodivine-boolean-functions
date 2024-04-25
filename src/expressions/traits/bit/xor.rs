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
        let expected = (var("a") & !var("b")) | (!var("a") & var("b"));

        assert!(actual.semantic_eq(&expected));
    }
}
