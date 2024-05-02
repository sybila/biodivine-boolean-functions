use crate::table::TruthTable;
use crate::traits::BooleanPoint;
use crate::utils::row_index_to_bool_point;
use std::fmt::Debug;
use std::iter::Enumerate;
use std::vec::IntoIter;

pub struct RelationIterator {
    outputs: Enumerate<IntoIter<bool>>,
    variable_count: usize,
}

impl<T: Debug + Clone + Ord> From<&TruthTable<T>> for RelationIterator {
    fn from(value: &TruthTable<T>) -> Self {
        Self {
            outputs: value.outputs.clone().into_iter().enumerate(),
            variable_count: value.variable_count(),
        }
    }
}

impl Iterator for RelationIterator {
    type Item = (BooleanPoint, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((row_index, output)) = self.outputs.next() {
            let point = row_index_to_bool_point(row_index, self.variable_count);
            Some((point, output))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{BooleanFunction, Evaluate};
    use std::collections::BTreeMap;

    #[test]
    fn test_relation_ok() {
        let input = TruthTable::new(
            vec!["x", "y", "z"],
            vec![false, true, false, false, true, false, false, false],
        );

        let mut actual = input.relation();
        let expected = vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
            vec![false, true, true],
            vec![true, false, false],
            vec![true, false, true],
            vec![true, true, false],
            vec![true, true, true],
        ]
        .into_iter()
        .map(|point| {
            Some((
                point.clone(),
                input.evaluate(&BTreeMap::from_iter(vec![
                    ("x", point[0]),
                    ("y", point[1]),
                    ("z", point[2]),
                ])),
            ))
        })
        .collect::<Vec<_>>();

        assert_eq!(actual.next(), expected[0]);
        assert_eq!(actual.next(), expected[1]);
        assert_eq!(actual.next(), expected[2]);
        assert_eq!(actual.next(), expected[3]);
        assert_eq!(actual.next(), expected[4]);
        assert_eq!(actual.next(), expected[5]);
        assert_eq!(actual.next(), expected[6]);
        assert_eq!(actual.next(), expected[7]);

        assert_eq!(actual.next(), None);
        assert_eq!(actual.next(), None);
    }
}
