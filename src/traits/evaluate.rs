use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Evaluate<TLiteral: Debug + Clone + Eq + Hash> {
    fn evaluate(&self, literal_values: &HashMap<TLiteral, bool>) -> bool;

    fn evaluate_with_err(&self, literal_values: &HashMap<TLiteral, bool>)
        -> Result<bool, TLiteral>;
}
