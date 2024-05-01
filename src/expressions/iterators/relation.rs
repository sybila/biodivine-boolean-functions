use crate::expressions::Expression;
use crate::iterators::RelationIterator;
use std::fmt::Debug;

impl<T: Debug + Clone + Ord + 'static> From<&Expression<T>> for RelationIterator<T> {
    fn from(value: &Expression<T>) -> Self {
        Self::new(value)
    }
}
