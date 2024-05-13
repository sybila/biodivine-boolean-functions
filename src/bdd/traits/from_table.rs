use crate::bdd::Bdd;
use crate::table::TruthTable;
use crate::traits::{BooleanFunction, GatherLiterals};
use biodivine_lib_bdd::{BddPartialValuation, BddVariable};
use std::fmt::Debug;
use std::num::TryFromIntError;

impl<T: Debug + Clone + Ord> TryFrom<TruthTable<T>> for Bdd<T> {
    type Error = TryFromIntError;

    fn try_from(value: TruthTable<T>) -> Result<Self, Self::Error> {
        let literals = value.gather_literals();
        let literal_set = Self::make_inner_variable_set(literals.clone())?;

        let valuations = value
            .domain()
            .map(|point| {
                point
                    .into_iter()
                    .enumerate()
                    .map(|(index, value)| (BddVariable::from_index(index), value))
            })
            .map(|point_with_index| {
                BddPartialValuation::from_values(&point_with_index.collect::<Vec<_>>())
            })
            .collect::<Vec<_>>();

        Ok(Bdd::new(
            literal_set.mk_dnf(&valuations),
            literals.into_iter().collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::Expression;
    use itertools::Itertools;
    use std::str::FromStr;

    #[test]
    fn test_bdd_from_table_too_many_variables_nok() {
        let disallowed_count = u16::MAX as usize + 1;
        let inputs = (0..disallowed_count).collect_vec();
        // not accurate but doesn't effect test result
        let outputs = vec![true; 2usize.pow(16)];

        let table = TruthTable::new(inputs, outputs);
        let bdd = Bdd::try_from(table);

        assert!(bdd.is_err());
    }

    #[test]
    fn test_bdd_from_table_ok() {
        let exp_string = "(b | a & c & false) & !a | true".to_string();
        let source = Expression::from_str(&exp_string).unwrap();
        let expected = Bdd::try_from(source.clone()).unwrap();

        let input = TruthTable::from(source);
        let actual = Bdd::try_from(input).unwrap();

        assert!(actual.is_equivalent(&expected));
    }
}
