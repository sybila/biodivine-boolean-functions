use crate::traits::GatherLiterals;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

pub trait PowerSet<TLiteral: Debug + Clone + Eq + Ord>: GatherLiterals<TLiteral> {
    fn generate_power_set(&self) -> Vec<BTreeMap<TLiteral, bool>> {
        Self::generate_arbitrary_power_set(self.gather_literals())
    }

    fn generate_arbitrary_power_set(
        variables: BTreeSet<TLiteral>,
    ) -> Vec<BTreeMap<TLiteral, bool>> {
        let mut result = vec![];

        Self::generate_power_set_rec(
            variables.into_iter().collect(),
            BTreeMap::new(),
            &mut result,
        );

        result
    }

    fn generate_power_set_rec(
        mut initial: Vec<TLiteral>,
        mut current: BTreeMap<TLiteral, bool>,
        result: &mut Vec<BTreeMap<TLiteral, bool>>,
    ) {
        // initial is a Vec since BTreeSet doesn't have pop
        if let Some(literal) = initial.pop() {
            current.insert(literal.clone(), true);
            Self::generate_power_set_rec(initial.clone(), current.clone(), result);

            current.insert(literal, false);
            Self::generate_power_set_rec(initial, current.clone(), result);
        } else {
            result.push(current);
        }
    }
}
