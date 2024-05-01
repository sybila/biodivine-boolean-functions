use crate::expressions::Expression;
use crate::iterators::DomainIterator;
use std::fmt::Debug;

impl<T: Debug + Clone + Ord> From<&Expression<T>> for DomainIterator {
    fn from(value: &Expression<T>) -> Self {
        Self::new(value)
    }
}
