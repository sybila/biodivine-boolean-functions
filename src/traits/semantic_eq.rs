use crate::traits::power_set::PowerSet;
use std::fmt::Debug;

pub trait SemanticEq<TLiteral: Debug + Clone + Eq + Ord>: PowerSet<TLiteral> {
    fn semantic_eq(&self, other: &Self) -> bool;

    fn semantic_ne(&self, other: &Self) -> bool {
        !self.semantic_eq(other)
    }
}
