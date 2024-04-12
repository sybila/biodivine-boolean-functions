use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::table::TruthTable;
use crate::traits::GatherLiterals;

impl<TLiteral: Debug + Display + Clone + Eq + Hash> GatherLiterals<TLiteral>
    for TruthTable<TLiteral>
{
    fn gather_literals_rec(&self, current: &mut HashSet<TLiteral>) {
        current.extend(self.inputs.clone())
    }
}
