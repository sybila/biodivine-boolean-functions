use crate::bdd::traits::bit::bit_common;
use crate::bdd::Bdd;
use biodivine_lib_bdd::Bdd as InnerBdd;
use std::fmt::Debug;
use std::ops::BitOr;

impl<TLiteral: Debug + Clone + Eq + Ord + 'static> BitOr for Bdd<TLiteral> {
    type Output = Bdd<TLiteral>;

    fn bitor(self, rhs: Self) -> Self::Output {
        bit_common(self, rhs, InnerBdd::or)
    }
}

#[cfg(test)]
mod tests {
    use crate::bdd::Bdd;
    use crate::expressions::{var, Expression};
    use crate::table::TruthTable;
    use crate::traits::{BooleanFunction, GatherLiterals, Implication};

    #[test]
    fn test_or_same_variables() {
        let and = var("a") & var("b");
        let imply = var("a").imply(var("b"));

        let and_table = Bdd::try_from(and.clone()).expect("Should not panic here");
        let imply_table = Bdd::try_from(imply.clone()).expect("Should not panic here");

        let actual = and_table | imply_table;
        let expected = Bdd::try_from(and | imply).expect("Should not panic here");

        assert!(actual.is_equivalent(&expected));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_or_different_variables_1() {
        let and = var("a") & var("b");
        let imply = var("b").imply(var("c"));

        let and_table = Bdd::try_from(and.clone()).expect("Should not panic here");
        let imply_table = Bdd::try_from(imply.clone()).expect("Should not panic here");

        let actual = and_table | imply_table;
        let expected = Bdd::try_from(and | imply).expect("Should not panic here");

        assert!(actual.is_equivalent(&expected));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_or_different_variables_2() {
        let and = var("c") & var("b");
        let imply = var("a").imply(var("d"));

        let and_table = Bdd::try_from(and.clone()).expect("Should not panic here");
        let imply_table = Bdd::try_from(imply.clone()).expect("Should not panic here");

        let actual = and_table | imply_table;
        let expected = Bdd::try_from(and | imply).expect("Should not panic here");

        assert!(actual.is_equivalent(&expected));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_or_different_variables_3() {
        let lhs = Bdd::try_from(TruthTable::new(
            vec!["a", "b"],
            vec![true, true, false, true],
        ))
        .expect("Should not panic here");
        let rhs = Bdd::try_from(TruthTable::new(
            vec!["a", "c"],
            vec![false, true, false, false],
        ))
        .expect("Should not panic here");

        let actual = lhs | rhs;
        let expected = Bdd::try_from(TruthTable::new(
            vec!["a", "b", "c"],
            vec![true, true, true, true, false, true, true, true],
        ))
        .expect("Should not panic here");

        println!(
            "actual: {}",
            actual.bdd.to_dot_string(
                &Bdd::make_inner_variable_set(actual.gather_literals()).unwrap(),
                true
            )
        );

        println!("expr: {}", Expression::from(expected.clone()));

        assert!(actual.is_equivalent(&expected));
        assert_eq!(actual, expected);
    }
}
