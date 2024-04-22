use crate::expressions::Expression;
use crate::traits::PowerSet;
use std::fmt::Debug;
use std::hash::Hash;

impl<TLiteral: Debug + Clone + Eq + Hash> PowerSet<TLiteral> for Expression<TLiteral> {}
