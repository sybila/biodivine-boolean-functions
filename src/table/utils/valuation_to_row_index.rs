use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub fn values_to_row_index<TLiteral: Debug + Clone + Eq + Hash + Ord>(
    order: &[TLiteral],
    valuation: &HashMap<TLiteral, bool>,
) -> usize {
    let (index, _errors) = values_to_row_index_common(order, valuation);
    index
}

pub fn values_to_row_index_checked<TLiteral: Debug + Display + Clone + Eq + Hash + Ord>(
    order: &[TLiteral],
    valuation: &HashMap<TLiteral, bool>,
) -> Result<usize, Vec<TLiteral>> {
    let (index, errors) = values_to_row_index_common(order, valuation);

    if errors.is_empty() {
        Ok(index)
    } else {
        Err(errors)
    }
}

fn values_to_row_index_common<TLiteral: Debug + Clone + Eq + Hash + Ord>(
    order: &[TLiteral],
    valuation: &HashMap<TLiteral, bool>,
) -> (usize, Vec<TLiteral>) {
    let result = order
        .iter()
        .enumerate()
        .rev()
        .map(|(order_index, literal)| {
            if let Some(literal_is_true) = valuation.get(literal) {
                if *literal_is_true {
                    Ok(2_usize.pow((order.len() - order_index - 1) as u32))
                } else {
                    Ok(0)
                }
            } else {
                Err(literal)
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
    // .fold_ok(0, std::ops::Add::add);

    result
}
