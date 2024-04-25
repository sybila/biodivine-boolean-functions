use crate::expressions::iterators::{
    ExpressionDomainIterator, ExpressionImageIterator, ExpressionRelationIterator,
    ExpressionSupportIterator,
};
use crate::expressions::Expression;
use crate::traits::{BooleanFunction, BooleanValuation, Evaluate, GatherLiterals};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

impl<T: Debug + Clone + Eq + Ord> BooleanFunction<T> for Expression<T> {
    type DomainIterator = ExpressionDomainIterator;
    type RangeIterator = ExpressionImageIterator<T>;
    type RelationIterator = ExpressionRelationIterator<T>;
    type SupportIterator = ExpressionSupportIterator<T>;

    fn inputs(&self) -> BTreeSet<T> {
        self.gather_literals()
    }

    fn essential_inputs(&self) -> BTreeSet<T> {
        self.inputs()
            .into_iter()
            .filter(|input| {
                let true_valuation = BTreeMap::from([(input.clone(), true)]);
                let false_valuation = BTreeMap::from([(input.clone(), false)]);
                self.evaluate(&true_valuation) != self.evaluate(&false_valuation)
            })
            .collect()
    }

    fn domain(&self) -> Self::DomainIterator {
        self.into()
    }

    fn image(&self) -> Self::RangeIterator {
        self.into()
    }

    fn relation(&self) -> Self::RelationIterator {
        self.into()
    }

    fn support(&self) -> Self::SupportIterator {
        self.into()
    }

    fn restrict(&self, _valuation: &BooleanValuation<T>) -> Self {
        todo!()
    }

    fn substitute(&self, _mapping: BTreeMap<T, Self>) -> Self {
        todo!()
    }

    fn existential_quantification(&self, _variables: BTreeSet<T>) -> Self {
        todo!()
    }

    fn universal_quantification(&self, _variables: BTreeSet<T>) -> Self {
        todo!()
    }

    fn derivative(&self, _variables: BTreeSet<T>) -> Self {
        todo!()
    }

    fn is_equivalent(&self, _other: Self) -> bool {
        todo!()
    }

    fn is_implied_by(&self, _other: Self) -> bool {
        todo!()
    }
}
