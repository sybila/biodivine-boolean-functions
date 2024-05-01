use crate::traits::{BooleanPoint, Evaluate, GatherLiterals};
use crate::utils::{boolean_point_to_valuation, row_index_to_bool_point};
use std::collections::BTreeSet;
use std::fmt::Debug;

pub struct SupportIterator<T: Debug + Clone + Ord> {
    variables: BTreeSet<T>,
    evaluatable: Box<dyn Evaluate<T>>,
    index: usize,
}

impl<T: Debug + Clone + Ord> SupportIterator<T> {
    pub(crate) fn new(value: &(impl Evaluate<T> + GatherLiterals<T> + Clone + 'static)) -> Self {
        Self {
            variables: value.gather_literals(),
            evaluatable: Box::from(value.clone()),
            index: 0,
        }
    }
}

impl<T: Debug + Clone + Ord> Iterator for SupportIterator<T> {
    type Item = BooleanPoint;

    fn next(&mut self) -> Option<Self::Item> {
        let mut supporting_point = None;

        while supporting_point.is_none() {
            if self.index >= 2_usize.pow(self.variables.len() as u32) {
                return None;
            }

            let point = row_index_to_bool_point(self.index, self.variables.len());
            let valuation = boolean_point_to_valuation(self.variables.clone(), point.clone())?;

            // point is supporting
            if self.evaluatable.evaluate(&valuation) {
                supporting_point = Some(point)
            }

            self.index += 1;
        }

        supporting_point
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::var;
    use crate::traits::BooleanFunction;

    #[test]
    fn test_support_ok() {
        let input = var("d") & var("b") | var("a");

        let mut actual = input.support();
        let expected = [
            Some(vec![false, true, true]),
            Some(vec![true, false, false]),
            Some(vec![true, false, true]),
            Some(vec![true, true, false]),
            Some(vec![true, true, true]),
        ];

        assert_eq!(actual.next(), expected[0]);
        assert_eq!(actual.next(), expected[1]);
        assert_eq!(actual.next(), expected[2]);
        assert_eq!(actual.next(), expected[3]);
        assert_eq!(actual.next(), expected[4]);

        assert_eq!(actual.next(), None);
        assert_eq!(actual.next(), None);
    }

    #[test]
    fn test_support_2_ok() {
        let input = var("d") & var("b") | var("c");

        let mut actual = input.support();
        let expected = [
            Some(vec![false, true, false]),
            Some(vec![false, true, true]),
            Some(vec![true, false, true]),
            Some(vec![true, true, false]),
            Some(vec![true, true, true]),
        ];

        assert_eq!(actual.next(), expected[0]);
        assert_eq!(actual.next(), expected[1]);
        assert_eq!(actual.next(), expected[2]);
        assert_eq!(actual.next(), expected[3]);
        assert_eq!(actual.next(), expected[4]);

        assert_eq!(actual.next(), None);
        assert_eq!(actual.next(), None);
    }

    #[test]
    fn test_no_support_2_ok() {
        let input = var("a") & !var("a");

        let mut actual = input.support();

        assert_eq!(actual.next(), None);
        assert_eq!(actual.next(), None);
    }
}
