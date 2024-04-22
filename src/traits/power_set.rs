use crate::traits::GatherLiterals;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

pub trait PowerSet<TLiteral: Debug + Clone + Eq + Hash>: GatherLiterals<TLiteral> {
    fn generate_power_set(&self) -> Vec<HashMap<TLiteral, bool>> {
        Self::generate_arbitrary_power_set(self.gather_literals())
    }

    fn generate_arbitrary_power_set(variables: HashSet<TLiteral>) -> Vec<HashMap<TLiteral, bool>> {
        let mut result = vec![];

        Self::generate_power_set_rec(variables.into_iter().collect(), HashMap::new(), &mut result);

        result
    }

    fn generate_power_set_rec(
        mut initial: Vec<TLiteral>,
        mut current: HashMap<TLiteral, bool>,
        result: &mut Vec<HashMap<TLiteral, bool>>,
    ) {
        // initial is a Vec since HashSet doesn't have pop
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
