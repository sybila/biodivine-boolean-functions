use crate::bdd::Bdd;
use crate::table::utils::boolean_point_to_row_index;
use crate::table::TruthTable;
use crate::traits::BooleanFunction;
use std::fmt::Debug;

impl<T: Debug + Clone + Ord> From<Bdd<T>> for TruthTable<T> {
    fn from(value: Bdd<T>) -> Self {
        let inputs = value.inputs();
        let mut outputs = vec![false; 2usize.pow(value.inputs().len() as u32)];

        value
            .support()
            .map(|point| boolean_point_to_row_index(&point))
            .for_each(|index| outputs[index] = true);

        TruthTable::new(inputs.into_iter().collect(), outputs)
    }
}
