use crate::expressions::Expression;
use crate::iterators::SupportIterator;
use std::fmt::Debug;

impl<T: Debug + Clone + Ord + 'static> From<&Expression<T>> for SupportIterator<T> {
    fn from(value: &Expression<T>) -> Self {
        Self::new(value)
    }
}
