use crate::iterators::DomainIterator;
use crate::table::TruthTable;
use crate::traits::{Evaluate, GatherLiterals};
use crate::utils::boolean_point_to_valuation;
use std::collections::BTreeSet;
use std::fmt::Debug;

mod and;
mod not;
mod or;
mod xor;

fn bit_common<T: Debug + Clone + Ord, F: Fn(bool, bool) -> bool>(
    me: &TruthTable<T>,
    other: &TruthTable<T>,
    op: F,
) -> TruthTable<T> {
    let self_variables = me.gather_literals();
    let other_variables = other.gather_literals();

    if self_variables == other_variables {
        let inputs = me.inputs.clone();
        let outputs = me
            .outputs
            .clone()
            .into_iter()
            .zip(other.outputs.clone())
            .map(|(self_output, other_output)| op(self_output, other_output))
            .collect();

        TruthTable::new(inputs, outputs)
    } else {
        let inputs_set = BTreeSet::from_iter(self_variables.union(&other_variables).cloned());
        let outputs = DomainIterator::from_count(inputs_set.len())
            .map(|point| {
                boolean_point_to_valuation(inputs_set.clone(), point).expect(
                    "Point should be from domain of the same dimension as the number of inputs",
                )
            })
            .map(|valuation| op(me.evaluate(&valuation), other.evaluate(&valuation)))
            .collect();
        let inputs = Vec::from_iter(inputs_set);

        TruthTable::new(inputs, outputs)
    }
}
