use crate::table::TruthTable;
use std::fmt::Debug;
use std::vec::IntoIter;

pub struct ImageIterator {
    outputs: IntoIter<bool>,
}

impl<T: Debug + Clone + Ord> From<&TruthTable<T>> for ImageIterator {
    fn from(value: &TruthTable<T>) -> Self {
        Self {
            outputs: value.outputs.clone().into_iter(),
        }
    }
}

impl Iterator for ImageIterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.outputs.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::BooleanFunction;

    #[test]
    fn test_image() {
        let input = TruthTable::new(
            vec!["x", "y", "z"],
            vec![false, true, false, false, true, false, false, false],
        );

        let actual = input.image().collect::<Vec<_>>();
        let expected = input.outputs;

        assert_eq!(actual, expected)
    }
}
