use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Display};
use std::num::TryFromIntError;

use biodivine_lib_bdd::{Bdd as InnerBdd, BddVariable, BddVariableSet};

use crate::bdd::utils::{extend_bdd_variables, prune_bdd_variables};

pub mod iterators;
mod traits;
mod utils;

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

impl<TLiteral: Debug + Clone + Eq + Ord + Display> Bdd<TLiteral> {
    /// Creates the String-based `BddVariableSet` from `self` that `lib-bdd` operates with to map BDD node indexes to variables with names.
    pub fn make_variable_set(&self) -> BddVariableSet {
        BddVariableSet::from_iter(self.inputs.iter().map(ToString::to_string))
    }
}

impl<TLiteral: Debug + Clone + Eq + Ord> Bdd<TLiteral> {
    pub fn mk_const(value: bool) -> Bdd<TLiteral> {
        let empty_set = BddVariableSet::new(&[]);
        let inner = if value {
            empty_set.mk_true()
        } else {
            empty_set.mk_false()
        };
        Bdd::new(inner, Vec::new())
    }

    pub fn mk_literal(variable: TLiteral, value: bool) -> Bdd<TLiteral> {
        let singleton_set = BddVariableSet::new_anonymous(1);
        let inner = singleton_set.mk_literal(BddVariable::from_index(0), value);
        Bdd::new(inner, vec![variable])
    }

    pub fn node_count(&self) -> usize {
        self.bdd.size()
    }
}

impl<TLiteral: Debug + Clone + Eq + Ord> Bdd<TLiteral> {
    pub(crate) fn new(inner: InnerBdd, inputs: Vec<TLiteral>) -> Self {
        Self { bdd: inner, inputs }
    }

    /// Converts a literal representation as a generic user struct
    /// into `BddVariable` used by `lib_bdd::Bdd`.
    ///
    /// If such a variable isn't used in this `Bdd`, the method returns `None`.
    fn map_var_outer_to_inner(&self, variable: &TLiteral) -> Option<BddVariable> {
        self.inputs
            .binary_search(variable)
            .ok()
            .map(BddVariable::from_index)
    }

    /// Converts a `BddVariable` used by `lib_bdd::Bdd` into a literal representation
    /// used by `self`.
    ///
    /// If such a variable isn't used in this `Bdd`, the method returns `None`.
    pub(crate) fn map_var_inner_to_outer(&self, variable: BddVariable) -> Option<TLiteral> {
        self.inputs.get(variable.to_index()).cloned()
    }

    /// Creates a `BddVariableSet` used by `lib_bdd::Bdd`.
    ///
    /// Since `lib_bdd` only supports up to 2<sup>16</sup> variables, this method returns an `Err` if
    /// the number of `variables` is above that.
    fn make_inner_variable_set(
        variables: BTreeSet<TLiteral>,
    ) -> Result<BddVariableSet, TryFromIntError> {
        let num_vars = u16::try_from(variables.len())?;
        Ok(BddVariableSet::new_anonymous(num_vars))
    }

    pub fn inner(&self) -> &InnerBdd {
        &self.bdd
    }

    fn restrict_and_prune_map<TValue>(
        &self,
        valuation: &BTreeMap<TLiteral, TValue>,
        new_bdd: &Bdd<TLiteral>,
    ) -> Bdd<TLiteral> {
        self.restrict_and_prune_common(valuation, new_bdd, |set, var| set.contains_key(var))
    }

    fn restrict_and_prune_set(
        &self,
        valuation: &BTreeSet<TLiteral>,
        new_bdd: &Bdd<TLiteral>,
    ) -> Bdd<TLiteral> {
        self.restrict_and_prune_common(valuation, new_bdd, |set, var| set.contains(var))
    }

    fn restrict_and_prune_common<TCollection, P: Fn(&TCollection, &&TLiteral) -> bool>(
        &self,
        valuation: &TCollection,
        new_bdd: &Bdd<TLiteral>,
        contains: P,
    ) -> Bdd<TLiteral> {
        let restricted_inputs = self
            .inputs
            .iter()
            .filter(|var| !contains(valuation, var))
            .cloned()
            .collect::<Vec<_>>();
        prune_bdd_variables(new_bdd, &restricted_inputs)
    }

    fn union_and_extend(
        &self,
        other: &Bdd<TLiteral>,
    ) -> (Bdd<TLiteral>, Bdd<TLiteral>, Vec<TLiteral>) {
        let mut common_inputs = self.inputs.clone();
        for other in &other.inputs {
            if !common_inputs.contains(other) {
                common_inputs.push(other.clone());
            }
        }
        common_inputs.sort();

        let self_lifted = extend_bdd_variables(self, &common_inputs);
        let other_lifted = extend_bdd_variables(other, &common_inputs);

        (self_lifted, other_lifted, common_inputs)
    }

    fn union_and_extend_n_ary(
        &self,
        others: &mut BTreeMap<TLiteral, Bdd<TLiteral>>,
    ) -> (Bdd<TLiteral>, Vec<TLiteral>) {
        let mut common_inputs = self.inputs.clone();

        for other in others.values() {
            for input in &other.inputs {
                if !common_inputs.contains(input) {
                    common_inputs.push(input.clone());
                }
            }
        }

        common_inputs.sort();

        let self_lifted = extend_bdd_variables(self, &common_inputs);
        for other in others.values_mut() {
            *other = extend_bdd_variables(other, &common_inputs);
        }

        (self_lifted, common_inputs)
    }
}
