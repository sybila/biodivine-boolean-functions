use crate::table::traits::bit::bit_common;
use crate::table::TruthTable;
use std::fmt::Debug;
use std::ops::BitOr;

impl<T: Debug + Clone + Ord> BitOr for TruthTable<T> {
    type Output = TruthTable<T>;

    fn bitor(self, other: Self) -> Self::Output {
        bit_common(self, other, BitOr::bitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::var;
    use crate::table::TruthTable;
    use crate::traits::{Implication, SemanticEq};

    #[test]
    fn test_and_same_variables() {
        let and = var("a") & var("b");
        let imply = var("a").imply(var("b"));

        let and_table = TruthTable::from(&and);
        let imply_table = TruthTable::from(&imply);

        let actual = and_table | imply_table;
        let expected = TruthTable::from(and | imply);

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_and_different_variables_1() {
        let and = var("a") & var("b");
        let imply = var("b").imply(var("c"));

        let and_table = TruthTable::from(&and);
        let imply_table = TruthTable::from(&imply);

        let actual = and_table | imply_table;
        let expected = TruthTable::from(and | imply);

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_and_different_variables_2() {
        let and = var("c") & var("b");
        let imply = var("a").imply(var("d"));

        let and_table = TruthTable::from(&and);
        let imply_table = TruthTable::from(&imply);

        let actual = and_table | imply_table;
        let expected = TruthTable::from(and | imply);

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_and_different_variables_3() {
        let lhs = TruthTable::new(vec!["a", "b"], vec![true, true, false, true]);
        let rhs = TruthTable::new(vec!["a", "c"], vec![false, true, false, true]);

        let actual = lhs | rhs;
        let expected = TruthTable::new(
            vec!["a", "b", "c"],
            vec![true, true, true, true, false, true, true, true],
        );

        assert!(actual.semantic_eq(&expected));
        assert_eq!(actual, expected);
    }
}
