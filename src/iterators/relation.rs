use crate::traits::{BooleanPoint, Evaluate, GatherLiterals};
use crate::utils::{boolean_point_to_valuation, row_index_to_bool_point};
use std::collections::BTreeSet;
use std::fmt::Debug;

pub struct RelationIterator<T: Debug + Clone + Ord> {
    variables: BTreeSet<T>,
    evaluatable: Box<dyn Evaluate<T>>,
    index: usize,
}

impl<T: Debug + Clone + Ord> RelationIterator<T> {
    pub(crate) fn new(value: &(impl Evaluate<T> + GatherLiterals<T> + Clone + 'static)) -> Self {
        Self {
            variables: value.gather_literals(),
            evaluatable: Box::from(value.clone()),
            index: 0,
        }
    }
}

impl<T: Debug + Clone + Ord> Iterator for RelationIterator<T> {
    type Item = (BooleanPoint, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 2_usize.pow(self.variables.len() as u32) {
            return None;
        }

        let boolean_point = row_index_to_bool_point(self.index, self.variables.len());
        let valuation = boolean_point_to_valuation(self.variables.clone(), boolean_point.clone())?;
        let result = self.evaluatable.evaluate(&valuation);

        self.index += 1;

        Some((boolean_point, result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::var;
    use crate::traits::BooleanFunction;
    use std::collections::BTreeMap;

    #[test]
    fn test_relation_ok() {
        let input = var("d") & var("b") | var("a");

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
                    ("a".to_string(), point[0]),
                    ("b".to_string(), point[1]),
                    ("d".to_string(), point[2]),
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
