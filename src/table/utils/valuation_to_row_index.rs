use std::collections::BTreeMap;
use std::fmt::Debug;

// Any errors are ignored
pub fn values_to_row_index<TLiteral: Debug + Clone + Eq + Ord>(
    order: &[TLiteral],
    valuation: &BTreeMap<TLiteral, bool>,
) -> usize {
    let (index, _errors) = values_to_row_index_common(order, valuation, Some(false));
    index
}

pub fn values_to_row_index_with_default<TLiteral: Debug + Clone + Eq + Ord>(
    order: &[TLiteral],
    valuation: &BTreeMap<TLiteral, bool>,
    default_value: bool,
) -> usize {
    let (index, _errors) = values_to_row_index_common(order, valuation, Some(default_value));
    index
}

pub fn values_to_row_index_checked<TLiteral: Debug + Clone + Eq + Ord>(
    order: &[TLiteral],
    valuation: &BTreeMap<TLiteral, bool>,
) -> Result<usize, Vec<TLiteral>> {
    let (index, errors) = values_to_row_index_common(order, valuation, None);

    if errors.is_empty() {
        Ok(index)
    } else {
        Err(errors)
    }
}

fn values_to_row_index_common<TLiteral: Debug + Clone + Eq + Ord>(
    order: &[TLiteral],
    valuation: &BTreeMap<TLiteral, bool>,
    default_value: Option<bool>,
) -> (usize, Vec<TLiteral>) {
    let result = order
        .iter()
        .enumerate()
        .rev()
        .map(|(order_index, literal)| {
            match (valuation.get(literal), &default_value) {
                (Some(true), _) | (None, Some(true)) => {
                    // found true, so do not use default or not found, but can use default true
                    Ok(2_usize.pow((order.len() - order_index - 1) as u32))
                }

                (Some(false), _) | (None, Some(false)) => Ok(0), // found false or can use default false

                (None, None) => Err(literal), // did not find and cannot use default
            }
        })
        .fold(
            (0usize, Vec::new()),
            |(acc, mut not_found), item| match item {
                Ok(number) => (acc + number, not_found),
                Err(literal) => {
                    not_found.push(literal.clone());
                    (acc, not_found)
                }
            },
        );

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_common_nodefault_unknownvar() {
        let order = vec!["1", "2"];
        let valuation = BTreeMap::from_iter(vec![("1", true)]);
        let default = None;

        let (_index, errors) = values_to_row_index_common(&order, &valuation, default);

        assert!(!errors.is_empty());
        assert_eq!(errors, vec!["2"]);
    }

    #[rstest]
    fn test_common_defaultfalse_unknownvar() {
        let order = vec!["1", "2"];
        let valuation = BTreeMap::from_iter(vec![("1", true)]);
        let default = Some(false);

        let expected_index = 1usize << 1;

        let (index, errors) = values_to_row_index_common(&order, &valuation, default);

        assert!(errors.is_empty());
        assert_eq!(index, expected_index);
    }

    #[rstest]
    fn test_common_defaulttrue_unknownvar() {
        let order = vec!["1", "2"];
        let valuation = BTreeMap::from_iter(vec![("1", true)]);
        let default = Some(true);

        let expected_index = (1usize << 1) + (1usize << 0);

        let (index, errors) = values_to_row_index_common(&order, &valuation, default);

        assert!(errors.is_empty());
        assert_eq!(index, expected_index);
    }

    #[rstest]
    fn test_common_nodefault_knownvar() {
        let order = vec!["1", "2"];
        let valuation = BTreeMap::from_iter(vec![("1", true), ("2", true)]);
        let default = None;

        let expected_index = (1usize << 1) + (1usize << 0);

        let (index, errors) = values_to_row_index_common(&order, &valuation, default);

        assert!(errors.is_empty());
        assert_eq!(index, expected_index);
    }

    #[rstest]
    fn test_common_defaultfalse_knownvar() {
        let order = vec!["1", "2"];
        let valuation = BTreeMap::from_iter(vec![("1", true), ("2", true)]);
        let default = Some(false);

        let expected_index = (1usize << 1) + (1usize << 0);

        let (index, errors) = values_to_row_index_common(&order, &valuation, default);

        assert!(errors.is_empty());
        assert_eq!(index, expected_index);
    }

    #[rstest]
    fn test_common_defaulttrue_knownvar() {
        let order = vec!["1", "2"];
        let valuation = BTreeMap::from_iter(vec![("1", true), ("2", false)]);
        let default = Some(true);

        let expected_index = 1usize << 1;

        let (index, errors) = values_to_row_index_common(&order, &valuation, default);

        assert!(errors.is_empty());
        assert_eq!(index, expected_index);
    }

    #[rstest]
    fn test_checked_unknownvar() {
        let order = vec!["1", "2"];
        let valuation = BTreeMap::from_iter(vec![("1", true)]);

        let actual = values_to_row_index_checked(&order, &valuation);
        let expected = Err(vec!["2"]);

        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_checked_knownvar() {
        let order = vec!["1", "2"];
        let valuation = BTreeMap::from_iter(vec![("1", true), ("2", true)]);

        let actual = values_to_row_index_checked(&order, &valuation);
        let expected = Ok((1usize << 1) + (1usize << 0));

        assert_eq!(actual, expected);
    }
}
