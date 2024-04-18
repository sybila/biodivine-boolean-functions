use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

// Any errors are ignored
pub fn values_to_row_index<TLiteral: Debug + Clone + Eq + Hash + Ord>(
    order: &[TLiteral],
    valuation: &HashMap<TLiteral, bool>,
) -> usize {
    let (index, _errors) = values_to_row_index_common(order, valuation, Some(false));
    index
}

pub fn values_to_row_index_with_default<TLiteral: Debug + Clone + Eq + Hash + Ord>(
    order: &[TLiteral],
    valuation: &HashMap<TLiteral, bool>,
    default_value: bool,
) -> usize {
    let (index, _errors) = values_to_row_index_common(order, valuation, Some(default_value));
    index
}

pub fn values_to_row_index_checked<TLiteral: Debug + Display + Clone + Eq + Hash + Ord>(
    order: &[TLiteral],
    valuation: &HashMap<TLiteral, bool>,
) -> Result<usize, Vec<TLiteral>> {
    let (index, errors) = values_to_row_index_common(order, valuation, None);

    if errors.is_empty() {
        Ok(index)
    } else {
        Err(errors)
    }
}

fn values_to_row_index_common<TLiteral: Debug + Clone + Eq + Hash + Ord>(
    order: &[TLiteral],
    valuation: &HashMap<TLiteral, bool>,
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
