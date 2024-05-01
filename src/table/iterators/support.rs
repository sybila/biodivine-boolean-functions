use crate::iterators::SupportIterator;
use crate::table::TruthTable;
use std::fmt::Debug;

impl<T: Debug + Clone + Ord + 'static> From<&TruthTable<T>> for SupportIterator<T> {
    fn from(value: &TruthTable<T>) -> Self {
        Self::new(value)
    }
}
