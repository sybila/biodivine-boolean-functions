use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::table::TruthTable;
use crate::table::utils::{values_to_row_index, values_to_row_index_checked};
use crate::traits::Evaluate;

impl<TLiteral: Debug + Display + Clone + Eq + Hash + Ord> Evaluate<TLiteral>
    for TruthTable<TLiteral>
{
    fn evaluate(&self, literal_values: &HashMap<TLiteral, bool>) -> bool {
        let index = values_to_row_index(&self.inputs, literal_values);

        self.outputs[index]
    }

    fn evaluate_with_err(
        &self,
        literal_values: &HashMap<TLiteral, bool>,
    ) -> Result<bool, TLiteral> {
        let index = values_to_row_index_checked(&self.inputs, literal_values)
            // TODO change after evaluate trait refactor
            .map_err(|not_found| not_found[0].clone())?;

        Ok(self.outputs[index])
    }
}
