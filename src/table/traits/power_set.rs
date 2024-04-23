use crate::table::TruthTable;
use crate::traits::PowerSet;
use std::fmt::{Debug, Display};

impl<TLiteral: Debug + Display + Clone + Eq + Ord> PowerSet<TLiteral> for TruthTable<TLiteral> {}
