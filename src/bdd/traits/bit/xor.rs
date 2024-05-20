use crate::bdd::traits::bit::bit_common;
use crate::bdd::Bdd;
use biodivine_lib_bdd::Bdd as InnerBdd;
use std::fmt::Debug;
use std::ops::{BitXor, BitXorAssign};

impl<TLiteral: Debug + Clone + Eq + Ord + 'static> BitXor for Bdd<TLiteral> {
    type Output = Bdd<TLiteral>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        bit_common(&self, &rhs, InnerBdd::xor)
    }
}

impl<TLiteral: Debug + Clone + Eq + Ord + 'static> BitXor for &Bdd<TLiteral> {
    type Output = Bdd<TLiteral>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        bit_common(self, rhs, InnerBdd::xor)
    }
}

impl<TLiteral: Debug + Clone + Eq + Ord + 'static> BitXorAssign for Bdd<TLiteral> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = bit_common(self, &rhs, InnerBdd::xor)
    }
}

#[cfg(test)]
mod tests {
    use crate::bdd::Bdd;
    use crate::expressions::var;
    use crate::table::TruthTable;
    use crate::traits::{BooleanFunction, Implication};

    #[test]
    fn test_and_same_variables() {
        let and = var("a") & var("b");
        let imply = var("a").imply(var("b"));

        let and_table = Bdd::try_from(and.clone()).expect("Should not panic here");
        let imply_table = Bdd::try_from(imply.clone()).expect("Should not panic here");

        let actual = and_table ^ imply_table;
        let expected = Bdd::try_from(and ^ imply).expect("Should not panic here");

        assert!(actual.is_equivalent(&expected));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_and_different_variables_1() {
        let and = var("a") & var("b");
        let imply = var("b").imply(var("c"));

        let and_table = Bdd::try_from(and.clone()).expect("Should not panic here");
        let imply_table = Bdd::try_from(imply.clone()).expect("Should not panic here");

        let actual = and_table ^ imply_table;
        let expected = Bdd::try_from(and ^ imply).expect("Should not panic here");

        assert!(actual.is_equivalent(&expected));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_and_different_variables_2() {
        let and = var("c") & var("b");
        let imply = var("a").imply(var("d"));

        let and_table = Bdd::try_from(and.clone()).expect("Should not panic here");
        let imply_table = Bdd::try_from(imply.clone()).expect("Should not panic here");

        let actual = and_table ^ imply_table;
        let expected = Bdd::try_from(and ^ imply).expect("Should not panic here");

        assert!(actual.is_equivalent(&expected));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_and_different_variables_3() {
        let lhs = Bdd::try_from(TruthTable::new(
            vec!["a", "b"],
            vec![true, true, false, true],
        ))
        .expect("Should not panic here");
        let rhs = Bdd::try_from(TruthTable::new(
            vec!["a", "c"],
            vec![false, true, false, true],
        ))
        .expect("Should not panic here");

        let actual = lhs | rhs;
        let expected = Bdd::try_from(TruthTable::new(
            vec!["a", "b", "c"],
            vec![true, false, true, false, false, true, true, false],
        ))
        .expect("Should not panic here");

        assert!(actual.is_equivalent(&expected));
        assert_eq!(actual, expected);
    }
}
