use crate::expressions::Expression;
use crate::traits::{Evaluate, GatherLiterals, PowerSet, SemanticEq};
use std::collections::BTreeSet;
use std::fmt::Debug;

impl<TLiteral: Debug + Clone + Eq + Ord> SemanticEq<TLiteral> for Expression<TLiteral> {
    fn semantic_eq(&self, other: &Self) -> bool {
        let self_literals = self.gather_literals();
        let other_literals = other.gather_literals();
        let literals_union = BTreeSet::from_iter(self_literals.union(&other_literals).cloned());

        let all_options = Self::generate_arbitrary_power_set(literals_union);

        all_options.into_iter().all(|literal_settings| {
            self.evaluate(&literal_settings) == other.evaluate(&literal_settings)
        })
    }
}
