use crate::traits::power_set::PowerSet;
use std::fmt::Debug;
use std::hash::Hash;

pub trait SemanticEq<TLiteral: Debug + Clone + Eq + Hash>: PowerSet<TLiteral> {
    fn semantic_eq(&self, other: &Self) -> bool;

    fn semantic_ne(&self, other: &Self) -> bool {
        !self.semantic_eq(other)
    }
}
