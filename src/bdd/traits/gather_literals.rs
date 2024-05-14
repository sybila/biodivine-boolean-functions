use crate::bdd::Bdd;
use std::collections::BTreeSet;
use std::fmt::Debug;

use crate::traits::GatherLiterals;

impl<TLiteral: Debug + Clone + Eq + Ord> GatherLiterals<TLiteral> for Bdd<TLiteral> {
    fn gather_literals_rec(&self, current: &mut BTreeSet<TLiteral>) {
        current.extend(self.inputs.clone())
    }
}
