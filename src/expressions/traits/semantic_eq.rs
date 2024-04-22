use crate::expressions::Expression;
use crate::traits::{Evaluate, GatherLiterals, PowerSet, SemanticEq};
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

impl<TLiteral: Debug + Clone + Eq + Hash> SemanticEq<TLiteral> for Expression<TLiteral> {
    fn semantic_eq(&self, other: &Self) -> bool {
        let self_literals = self.gather_literals();
        let other_literals = other.gather_literals();
        let literals_union = HashSet::from_iter(self_literals.union(&other_literals).cloned());

        let all_options = Self::generate_arbitrary_power_set(literals_union);

        all_options.into_iter().all(|literal_settings| {
            self.evaluate(&literal_settings) == other.evaluate(&literal_settings)
        })
    }
}
