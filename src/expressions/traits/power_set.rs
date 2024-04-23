use crate::expressions::Expression;
use crate::traits::PowerSet;
use std::fmt::Debug;

impl<TLiteral: Debug + Clone + Eq + Ord> PowerSet<TLiteral> for Expression<TLiteral> {}
