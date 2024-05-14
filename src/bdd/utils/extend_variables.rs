use crate::bdd::Bdd;
use biodivine_lib_bdd::BddVariable;
use std::collections::HashMap;
use std::fmt::Debug;

/// Takes a `Bdd` object and extends it to a new input domain described by `new_inputs`.
///
/// Here, extends means that the input variables of the new `Bdd` are exactly `new_inputs`, but
/// the `Bdd` still represents the same function.
///
/// As such, it must hold that `new_inputs` is a superset of `bdd.inputs`. Currently, if this
/// condition is not satisfied, the method will panic.
pub fn extend_bdd_variables<TLiteral: Debug + Clone + Eq + Ord>(
    bdd: &Bdd<TLiteral>,
    new_inputs: &[TLiteral],
) -> Bdd<TLiteral> {
    if bdd.inputs == new_inputs {
        return bdd.clone();
    }

    // Test pre-condition.
    debug_assert!(bdd.inputs.iter().all(|it| new_inputs.contains(it)));

    let mut permutation = HashMap::new();

    // Since both vectors are sorted, we can advance through them simultaneously.
    // Also, since `bdd.inputs` is a subset of `new_inputs`, we know that every
    // `bdd.inputs[old_i]` must (eventually) appear in the `new_inputs` iterator, we just
    // need to skip enough of the new variables.
    for (old_i, var) in bdd.inputs.iter().enumerate() {
        let new_i = new_inputs
            .binary_search(var)
            .expect("Collection `new_inputs` is not a superset of `bdd.inputs`.");

        if new_i != old_i {
            permutation.insert(
                BddVariable::from_index(old_i),
                BddVariable::from_index(new_i),
            );
        }
    }

    let mut new_bdd = bdd.bdd.clone();
    unsafe {
        // These operations are not memory-unsafe, they can just break the BDD
        // in weird ways if you don't know what you are doing.
        new_bdd.set_num_vars(u16::try_from(new_inputs.len()).unwrap());
        if !permutation.is_empty() {
            new_bdd.rename_variables(&permutation);
        }
    }

    Bdd::new(new_bdd, new_inputs.to_owned())
}
