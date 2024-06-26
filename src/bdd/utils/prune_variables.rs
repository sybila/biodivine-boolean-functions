use crate::bdd::Bdd;
use crate::traits::BooleanFunction;
use biodivine_lib_bdd::BddVariable;
use std::collections::HashMap;
use std::fmt::Debug;

/// Takes a `Bdd` object and only retains the variables given in `new_inputs`.
///
/// This expects the `Bdd` to only depend on the variables in `new_inputs`, meaning that
/// `bdd.essential_inputs` is a subset of `new_inputs`. If this is not satisfied, panic.
pub fn prune_bdd_variables<TLiteral: Debug + Clone + Eq + Ord>(
    bdd: &Bdd<TLiteral>,
    new_inputs: &[TLiteral],
) -> Bdd<TLiteral> {
    if bdd.inputs == new_inputs {
        return bdd.clone();
    }

    // Test pre-condition.
    debug_assert!(bdd
        .essential_inputs()
        .into_iter()
        .all(|it| new_inputs.contains(&it)));

    let mut permutation = HashMap::new();

    // "Inverse" of `expand_bdd_variables`. This works because both input lists are sorted
    // and the `new_inputs` is a subset of `bdd.inputs`.
    for (new_i, var) in new_inputs.iter().enumerate() {
        let old_i = bdd
            .inputs
            .binary_search(var)
            .expect("Collection `new_inputs` is not a subset of `bdd.inputs`.");

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
        if !permutation.is_empty() {
            new_bdd.rename_variables(&permutation);
        }
        new_bdd.set_num_vars(u16::try_from(new_inputs.len()).unwrap());
        // Also, notice that here, we are setting the variable count *after*
        // the permutation, not before, because it is actually decreasing, not
        // increasing.
    }

    Bdd::new(new_bdd, new_inputs.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::var;

    #[test]
    #[should_panic]
    fn test_prune_variables_nonsubset_nok() {
        let input = Bdd::try_from(var("a") & var("b")).unwrap();
        let new_vars = vec!["a".to_string(), "c".to_string()];

        let _ = prune_bdd_variables(&input, &new_vars);
    }
}
