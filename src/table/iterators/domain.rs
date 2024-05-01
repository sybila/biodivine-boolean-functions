use crate::iterators::DomainIterator;
use crate::table::TruthTable;
use std::fmt::Debug;

impl<T: Debug + Clone + Ord> From<&TruthTable<T>> for DomainIterator {
    fn from(value: &TruthTable<T>) -> Self {
        Self::new(value)
    }
}
