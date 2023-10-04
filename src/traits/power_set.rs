use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

pub trait PowerSet<TLiteral: Debug + Clone + Eq + Hash> {
    fn generate_power_set(literals: HashSet<TLiteral>) -> Vec<HashMap<TLiteral, bool>> {
        let mut result = vec![];

        Self::generate_power_set_rec(literals.into_iter().collect(), HashMap::new(), &mut result);

        result
    }

    fn generate_power_set_rec(
        initial: Vec<TLiteral>,
        current: HashMap<TLiteral, bool>,
        result: &mut Vec<HashMap<TLiteral, bool>>,
    );
}
