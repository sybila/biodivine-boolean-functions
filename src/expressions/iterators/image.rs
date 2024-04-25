use crate::expressions::Expression;
use crate::traits::{Evaluate, GatherLiterals};
use crate::utils::{boolean_point_to_valuation, row_index_to_bool_point};
use std::collections::BTreeSet;
use std::fmt::Debug;

pub struct ExpressionImageIterator<T: Debug + Clone + Ord> {
    variables: BTreeSet<T>,
    expression: Expression<T>,
    index: usize,
}

impl<T: Debug + Clone + Ord> From<&Expression<T>> for ExpressionImageIterator<T> {
    fn from(value: &Expression<T>) -> Self {
        Self {
            variables: value.gather_literals(),
            expression: value.clone(),
            index: 0,
        }
    }
}

impl<T: Debug + Clone + Ord> Iterator for ExpressionImageIterator<T> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 2_usize.pow(self.variables.len() as u32) {
            return None;
        }

        let boolean_point = row_index_to_bool_point(self.index, self.variables.len());
        let valuation = boolean_point_to_valuation(self.variables.clone(), boolean_point)?;
        let result = self.expression.evaluate(&valuation);

        self.index += 1;

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::var;
    use crate::traits::BooleanFunction;
    use std::collections::BTreeMap;

    #[test]
    fn test_image_ok() {
        let input = var("d") & var("b") | var("a");

        let mut actual = input.image();
        let expected = [
            Some(input.evaluate(&BTreeMap::from([
                ("a".to_string(), false),
                ("b".to_string(), false),
                ("d".to_string(), false),
            ]))),
            Some(input.evaluate(&BTreeMap::from([
                ("a".to_string(), false),
                ("b".to_string(), false),
                ("d".to_string(), true),
            ]))),
            Some(input.evaluate(&BTreeMap::from([
                ("a".to_string(), false),
                ("b".to_string(), true),
                ("d".to_string(), false),
            ]))),
            Some(input.evaluate(&BTreeMap::from([
                ("a".to_string(), false),
                ("b".to_string(), true),
                ("d".to_string(), true),
            ]))),
            Some(input.evaluate(&BTreeMap::from([
                ("a".to_string(), true),
                ("b".to_string(), false),
                ("d".to_string(), false),
            ]))),
            Some(input.evaluate(&BTreeMap::from([
                ("a".to_string(), true),
                ("b".to_string(), false),
                ("d".to_string(), true),
            ]))),
            Some(input.evaluate(&BTreeMap::from([
                ("a".to_string(), true),
                ("b".to_string(), true),
                ("d".to_string(), false),
            ]))),
            Some(input.evaluate(&BTreeMap::from([
                ("a".to_string(), true),
                ("b".to_string(), true),
                ("d".to_string(), true),
            ]))),
        ];

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
