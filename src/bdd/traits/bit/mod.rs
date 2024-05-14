use crate::bdd::Bdd;
use biodivine_lib_bdd::Bdd as InnerBdd;
use std::fmt::Debug;

mod and;

fn bit_common<T: Debug + Clone + Ord, F: Fn(&InnerBdd, &InnerBdd) -> InnerBdd>(
    me: Bdd<T>,
    other: Bdd<T>,
    op: F,
) -> Bdd<T> {
    if me.inputs == other.inputs {
        Bdd::new(op(&me.bdd, &other.bdd), me.inputs.clone())
    } else {
        let (self_lifted, rhs_lifted, common_inputs) = me.union_and_extend(&other);

        Bdd::new(op(&self_lifted.bdd, &rhs_lifted.bdd), common_inputs)
    }
}
