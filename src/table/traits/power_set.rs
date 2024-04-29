use crate::table::TruthTable;
use crate::traits::PowerSet;
use std::fmt::Debug;

impl<TLiteral: Debug + Clone + Eq + Ord> PowerSet<TLiteral> for TruthTable<TLiteral> {}
