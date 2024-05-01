use crate::expressions::Expression;
use crate::traits::{BooleanPoint, GatherLiterals};
use crate::utils::boolean_point_to_valuation;
use std::collections::BTreeMap;
use std::fmt::Debug;

mod domain;
mod image;
mod relation;
mod support;

impl<T: Debug + Clone + Eq + Ord> Expression<T> {
    pub fn boolean_point_to_valuation(&self, point: BooleanPoint) -> Option<BTreeMap<T, bool>> {
        boolean_point_to_valuation(self.gather_literals(), point)
    }
}
