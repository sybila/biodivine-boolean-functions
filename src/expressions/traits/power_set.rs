use crate::expressions::Expression;
use crate::traits::PowerSet;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

impl<TLiteral: Debug + Clone + Eq + Hash> PowerSet<TLiteral> for Expression<TLiteral> {
    fn generate_power_set_rec(
        mut initial: Vec<TLiteral>,
        mut current: HashMap<TLiteral, bool>,
        result: &mut Vec<HashMap<TLiteral, bool>>,
    ) {
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
