use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

pub fn btreeset_to_valuation<T: Debug + Clone + Eq + Ord>(
    set: BTreeSet<T>,
    bool_value: bool,
) -> BTreeMap<T, bool> {
    BTreeMap::from_iter(set.into_iter().map(|element| (element, bool_value)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_btreeset_to_valuation(#[values(true, false)] constant: bool) {
        let input_variables = ["x", "1", "?"];
        let input = BTreeSet::from_iter(&input_variables);

        let expected = BTreeMap::from_iter(input_variables.iter().map(|v| (v, constant)));
        let actual = btreeset_to_valuation(input, constant);

        assert_eq!(actual, expected);
    }
}
