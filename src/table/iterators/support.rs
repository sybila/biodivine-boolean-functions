use crate::table::TruthTable;
use crate::traits::BooleanPoint;
use crate::utils::row_index_to_bool_point;
use std::fmt::Debug;

pub struct SupportIterator {
    outputs: Box<dyn Iterator<Item = usize>>,
    variable_count: usize,
}

impl<T: Debug + Clone + Ord> From<&TruthTable<T>> for SupportIterator {
    fn from(value: &TruthTable<T>) -> Self {
        Self {
            outputs: Box::new(
                value
                    .outputs
                    .clone()
                    .into_iter()
                    .enumerate()
                    .filter(|(_row_index, output_is_true)| *output_is_true)
                    .map(|(row_index, _output_is_true)| row_index),
            ),
            variable_count: value.variable_count(),
        }
    }
}

impl Iterator for SupportIterator {
    type Item = BooleanPoint;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(row_index) = self.outputs.next() {
            let point = row_index_to_bool_point(row_index, self.variable_count);
            Some(point)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::table::TruthTable;
    use crate::traits::BooleanFunction;

    #[test]
    fn test_support_ok() {
        let input = TruthTable::new(
            vec!["x", "y", "z"],
            vec![false, true, false, false, true, false, false, false],
        );

        let mut actual = input.support();
        let expected = [
            Some(vec![false, false, true]),
            Some(vec![true, false, false]),
        ];

        assert_eq!(actual.next(), expected[0]);
        assert_eq!(actual.next(), expected[1]);

        assert_eq!(actual.next(), None);
        assert_eq!(actual.next(), None);
    }

    #[test]
    fn test_no_support_2_ok() {
        let input = TruthTable::new(
            vec!["x", "y", "z"],
            vec![false, false, false, false, false, false, false, false],
        );

        let mut actual = input.support();

        assert_eq!(actual.next(), None);
        assert_eq!(actual.next(), None);
    }
}
