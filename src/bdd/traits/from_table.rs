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

        Ok(Bdd {
            inputs: literals.into_iter().collect(),
            bdd: literal_set.mk_dnf(&valuations),
        })
    }
}
