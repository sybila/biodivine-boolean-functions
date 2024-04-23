use std::collections::BTreeSet;
use std::fmt::{Debug, Display};

use crate::table::TruthTable;
use crate::traits::GatherLiterals;

impl<TLiteral: Debug + Display + Clone + Eq + Ord> GatherLiterals<TLiteral>
    for TruthTable<TLiteral>
{
    fn gather_literals_rec(&self, current: &mut BTreeSet<TLiteral>) {
        current.extend(self.inputs.clone())
    }
}
