use crate::expressions::Expression;
use crate::iterators::ImageIterator;
use std::fmt::Debug;

impl<T: Debug + Clone + Ord + 'static> From<&Expression<T>> for ImageIterator<T> {
    fn from(value: &Expression<T>) -> Self {
        Self::new(value)
    }
}
