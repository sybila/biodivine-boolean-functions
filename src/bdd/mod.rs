mod iterators;
mod traits;
mod utils;

use crate::bdd::utils::extend_bdd_variables;
use biodivine_lib_bdd::{Bdd as InnerBdd, BddVariable, BddVariableSet};
use std::fmt::Debug;

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
