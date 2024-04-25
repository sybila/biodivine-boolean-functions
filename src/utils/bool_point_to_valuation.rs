use crate::traits::BooleanPoint;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::iter::zip;

pub fn boolean_point_to_valuation<T: Debug + Clone + Ord>(
    variables: BTreeSet<T>,
    point: BooleanPoint,
) -> Option<BTreeMap<T, bool>> {
    if point.len() != variables.len() {
        return None;
    }

    Some(BTreeMap::from_iter(zip(variables, point)))
}
