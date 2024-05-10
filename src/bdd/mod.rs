mod iterators;
mod traits;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Debug;
use std::iter::{zip, Zip};

use crate::bdd::iterators::{ImageIterator, SupportIterator};
use biodivine_lib_bdd::{Bdd as InnerBdd, BddVariable, BddVariableSet};
use num_bigint::BigUint;

use crate::iterators::DomainIterator;
use crate::traits::{BooleanFunction, BooleanPoint, BooleanValuation};

/*
   Conversions:

   Bdd -> Table: iterate support and use it to fill the table with ones.

   Table -> Bdd: Build a `BddPartialValuation` for each one-row in the table and then
   use `BddVariableSet::mk_dnf` to build `Bdd`.

   Bdd -> Expression: `Bdd.to_optimized_dnf()` to convert to `Vec<BddPartialValuation>`, then
   convert each partial valuation to one AND-clause and the whole vector to a disjunction.

   Expression -> Bdd: Run gather literals, create a BddVariableSet, then recursively follow
   the expression structure. Literal/Constant correspond to BddVariableSet.mk_literal/mk_constant,
   not/and/or operators correspond to Bdd.not/and/or.

*/

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Bdd<TLiteral>
where
    TLiteral: Debug + Clone + Eq + Ord,
{
    /// Always-sorted vector of no more than 65k variables (see `lib-bdd`).
    inputs: Vec<TLiteral>,
    /// Holds the `lib_bdd` representation.
    bdd: InnerBdd,
}

impl<TLiteral: Debug + Clone + Eq + Ord + 'static> Bdd<TLiteral> {
    /// Converts a literal representation as a generic user struct
    /// into `BddVariable` used by `lib_bdd::Bdd`.
    ///
    /// If such a variable isn't used in this `Bdd`, the method returns `None`.
    fn map_var_outer_to_inner(&self, variable: &TLiteral) -> Option<BddVariable> {
        self.inputs
            .binary_search(variable)
            .ok()
            .map(|it| BddVariable::from_index(it))
    }

    /// Converts a `BddVariable` used by `lib_bdd::Bdd` into a literal representation
    /// used by `self`.
    ///
    /// If such a variable isn't used in this `Bdd`, the method returns `None`.
    fn map_var_inner_to_outer(&self, variable: BddVariable) -> Option<TLiteral> {
        self.inputs.get(variable.to_index()).cloned()
    }

    /// Creates a `BddVariableSet` used by `lib_bdd::Bdd`.
    ///
    /// Since `lib_bdd` only supports up to 2<sup>16</sup> variables, this method currently panics.
    fn make_inner_variable_set(variables: Vec<TLiteral>) -> BddVariableSet {
        let num_vars = u16::try_from(variables.len()).expect("Too many variables");
        BddVariableSet::new_anonymous(num_vars)
    }
}

impl<T: Debug + Clone + Ord + 'static> BooleanFunction<T> for Bdd<T> {
    type DomainIterator = DomainIterator;
    type RangeIterator = ImageIterator;
    type RelationIterator = Zip<DomainIterator, ImageIterator>;
    type SupportIterator = SupportIterator;

    fn inputs(&self) -> BTreeSet<T> {
        self.inputs.iter().cloned().collect()
    }

    fn essential_inputs(&self) -> BTreeSet<T> {
        self.bdd
            .support_set()
            .into_iter()
            .map(|var| {
                // This unwrap is safe unless the BDD has variables that we don't know about.
                self.map_var_inner_to_outer(var).unwrap()
            })
            .collect()
    }

    fn degree(&self) -> usize {
        self.inputs.len()
    }

    fn essential_degree(&self) -> usize {
        self.bdd.support_set().len()
    }

    fn domain(&self) -> Self::DomainIterator {
        DomainIterator::from_count(self.inputs.len())
    }

    fn image(&self) -> Self::RangeIterator {
        // evaluate for each domain point
        // DomainIterator::new(self).map(|it| self.bdd.eval_in(&BddValuation::new(it)));
        ImageIterator::new(self.inputs.len(), &self.bdd)
    }

    fn relation(&self) -> Self::RelationIterator {
        // zip domain/range
        zip(self.domain(), self.image())
    }

    fn support(&self) -> Self::SupportIterator {
        SupportIterator::new(&self.bdd)
    }

    fn weight(&self) -> BigUint {
        self.bdd.exact_cardinality().to_biguint().unwrap()
    }

    fn restrict(&self, valuation: &BooleanValuation<T>) -> Self {
        let lib_bdd_valuation: Vec<(BddVariable, bool)> = valuation
            .iter()
            .filter_map(|(a, b)| self.map_var_outer_to_inner(a).map(|var| (var, *b)))
            .collect::<Vec<_>>();
        let new_bdd = Bdd {
            inputs: self.inputs.clone(),
            bdd: self.bdd.restrict(&lib_bdd_valuation),
        };
        let restricted_inputs = self
            .inputs
            .iter()
            .filter(|var| !valuation.contains_key(var))
            .cloned()
            .collect::<Vec<_>>();
        prune_bdd_variables(&new_bdd, &restricted_inputs)
    }

    fn substitute(&self, _mapping: &BTreeMap<T, Self>) -> Self {
        // Bdd.substitute exists, but assumes all BDDs share input variables (we need to extend)
        // and does not eliminate the substituted variable from inputs afterward (we need to prune).

        // Bdd.substitute currently assumes that the substituted functions does not depend on the
        // substituted variables. This will be solved in lib-bdd, we can just panic for now.
        todo!()
    }

    fn sat_point(&self) -> Option<BooleanPoint> {
        self.bdd.sat_witness().map(|it| it.vector())
    }

    fn existential_quantification(&self, variables: BTreeSet<T>) -> Self {
        let lib_bdd_variables = variables
            .iter()
            .filter_map(|it| self.map_var_outer_to_inner(it))
            .collect::<Vec<_>>();
        let new_bdd = Bdd {
            inputs: self.inputs.clone(),
            bdd: self.bdd.exists(&lib_bdd_variables),
        };
        let restricted_inputs = self
            .inputs
            .iter()
            .filter(|var| !variables.contains(var))
            .cloned()
            .collect::<Vec<_>>();
        prune_bdd_variables(&new_bdd, &restricted_inputs)
    }

    fn universal_quantification(&self, variables: BTreeSet<T>) -> Self {
        let lib_bdd_variables = variables
            .iter()
            .filter_map(|it| self.map_var_outer_to_inner(it))
            .collect::<Vec<_>>();
        let new_bdd = Bdd {
            inputs: self.inputs.clone(),
            bdd: self.bdd.for_all(&lib_bdd_variables),
        };
        let restricted_inputs = self
            .inputs
            .iter()
            .filter(|var| !variables.contains(var))
            .cloned()
            .collect::<Vec<_>>();
        prune_bdd_variables(&new_bdd, &restricted_inputs)
    }

    fn derivative(&self, variables: BTreeSet<T>) -> Self {
        let lib_bdd_variables = variables
            .iter()
            .filter_map(|it| self.map_var_outer_to_inner(it))
            .collect::<HashSet<_>>();
        let trigger = |var: BddVariable| lib_bdd_variables.contains(&var);

        let new_bdd = Bdd {
            inputs: self.inputs.clone(),
            bdd: InnerBdd::binary_op_nested(
                &self.bdd,
                &self.bdd,
                trigger,
                biodivine_lib_bdd::op_function::and,
                biodivine_lib_bdd::op_function::xor,
            ),
        };

        let restricted_inputs = self
            .inputs
            .iter()
            .filter(|var| !variables.contains(var))
            .cloned()
            .collect::<Vec<_>>();
        prune_bdd_variables(&new_bdd, &restricted_inputs)
    }

    fn is_equivalent(&self, other: &Self) -> bool {
        let mut common_inputs = self.inputs.clone();
        for other in &other.inputs {
            if !common_inputs.contains(other) {
                common_inputs.push(other.clone());
            }
        }
        common_inputs.sort();

        let self_lifted = extend_bdd_variables(&self, &common_inputs);
        let other_lifted = extend_bdd_variables(&other, &common_inputs);
        self_lifted.bdd == other_lifted.bdd
    }

    fn is_implied_by(&self, other: &Self) -> bool {
        let mut common_inputs = self.inputs.clone();
        for other in &other.inputs {
            if !common_inputs.contains(other) {
                common_inputs.push(other.clone());
            }
        }
        common_inputs.sort();

        let self_lifted = extend_bdd_variables(&self, &common_inputs);
        let other_lifted = extend_bdd_variables(&other, &common_inputs);
        other_lifted.bdd.imp(&self_lifted.bdd).is_true()
    }
}

/// Takes a `Bdd` object and extends it to a new input domain described by `new_inputs`.
///
/// Here, extends means that the input variables of the new `Bdd` are exactly `new_inputs`, but
/// the `Bdd` still represents the same function.
///
/// As such, it must hold that `new_inputs` is a superset of `bdd.inputs`. Currently, if this
/// condition is not satisfied, the method will panic.
fn extend_bdd_variables<TLiteral: Debug + Clone + Eq + Ord + 'static>(
    bdd: &Bdd<TLiteral>,
    new_inputs: &Vec<TLiteral>,
) -> Bdd<TLiteral> {
    // Test pre-condition.
    debug_assert!(bdd.inputs.iter().all(|it| new_inputs.contains(it)));

    let mut permutation = HashMap::new();

    // Since both vectors are sorted, we can advance through them simultaneously.
    // Also, since `bdd.inputs` is a subset of `new_inputs`, we know that every
    // `bdd.inputs[old_i]` must (eventually) appear in the `new_inputs` iterator, we just
    // need to skip enough of the new variables.
    let mut old_i = 0;
    for (new_i, var) in new_inputs.iter().enumerate() {
        if &bdd.inputs[old_i] == var {
            permutation.insert(
                BddVariable::from_index(old_i),
                BddVariable::from_index(new_i),
            );
            old_i += 1;
        }
    }

    Bdd {
        inputs: new_inputs.clone(),
        bdd: {
            let mut new_bdd = bdd.bdd.clone();
            unsafe {
                // These operations are not memory-unsafe, they can just break the BDD
                // in weird ways if you don't know what you are doing.
                new_bdd.set_num_vars(u16::try_from(new_inputs.len()).unwrap());
                new_bdd.rename_variables(&permutation);
            }
            new_bdd
        },
    }
}

/// Takes a `Bdd` object and only retains the variables given in `new_inputs`.
///
/// This expects the `Bdd` to only depend on the variables in `new_inputs`, meaning that
/// `bdd.essential_inputs` is a subset of `new_inputs`. If this is not satisfied, panic.
fn prune_bdd_variables<TLiteral: Debug + Clone + Eq + Ord + 'static>(
    bdd: &Bdd<TLiteral>,
    new_inputs: &Vec<TLiteral>,
) -> Bdd<TLiteral> {
    // Test pre-condition.
    debug_assert!(bdd
        .essential_inputs()
        .into_iter()
        .all(|it| new_inputs.contains(&it)));

    let mut permutation = HashMap::new();

    // "Inverse" of `expand_bdd_variables`. This works because both input lists are sorted
    // and the `new_inputs` is a subset of `bdd.inputs`.
    let mut new_i = 0usize;
    for (old_i, var) in bdd.inputs.iter().enumerate() {
        if &new_inputs[new_i] == var {
            permutation.insert(
                BddVariable::from_index(old_i),
                BddVariable::from_index(new_i),
            );
            new_i += 1;
        }
    }

    Bdd {
        inputs: new_inputs.clone(),
        bdd: {
            let mut new_bdd = bdd.bdd.clone();
            unsafe {
                // These operations are not memory-unsafe, they can just break the BDD
                // in weird ways if you don't know what you are doing.
                new_bdd.rename_variables(&permutation);
                new_bdd.set_num_vars(u16::try_from(new_inputs.len()).unwrap());
                // Also, notice that here, we are setting the variable count *after*
                // the permutation, not before, because it is actually decreasing, not
                // increasing.
            }
            new_bdd
        },
    }
}
