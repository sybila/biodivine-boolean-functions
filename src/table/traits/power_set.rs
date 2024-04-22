use crate::table::TruthTable;
use crate::traits::PowerSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;

impl<TLiteral: Debug + Display + Clone + Eq + Hash> PowerSet<TLiteral> for TruthTable<TLiteral> {}
