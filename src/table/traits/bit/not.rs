use crate::table::TruthTable;
use std::fmt::Debug;
use std::ops::Not;

impl<T: Debug + Clone + Ord> Not for &TruthTable<T> {
    type Output = TruthTable<T>;

    fn not(self) -> Self::Output {
        let mut cloned = self.clone();
        for output in cloned.outputs.iter_mut() {
            *output = !*output;
        }

        cloned
    }
}

impl<T: Debug + Clone + Ord> Not for TruthTable<T> {
    type Output = TruthTable<T>;

    fn not(self) -> Self::Output {
        (&self).not()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_owned() {
        let input = TruthTable::new(vec!["a", "b"], vec![true, false, true, false]);

        let actual = !input;
        let expected = vec![false, true, false, true];

        assert_eq!(actual.outputs, expected);
    }

    #[test]
    fn test_not_ref() {
        let input = TruthTable::new(vec!["a", "b"], vec![true, false, true, false]);
        let input_ref = &input;

        let actual = !input_ref;
        let expected = vec![false, true, false, true];

        assert_eq!(actual.outputs, expected);
    }
}
