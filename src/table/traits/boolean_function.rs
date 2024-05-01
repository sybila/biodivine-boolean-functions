use crate::iterators::{DomainIterator, ImageIterator, RelationIterator, SupportIterator};
use crate::table::TruthTable;
use crate::traits::{BooleanFunction, BooleanValuation};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

impl<T: Debug + Clone + Ord + 'static> BooleanFunction<T> for TruthTable<T> {
    type DomainIterator = DomainIterator;
    type RangeIterator = ImageIterator<T>;
    type RelationIterator = RelationIterator<T>;
    type SupportIterator = SupportIterator<T>;

    fn inputs(&self) -> BTreeSet<T> {
        todo!()
    }

    fn essential_inputs(&self) -> BTreeSet<T> {
        todo!()
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

    fn substitute(&self, _mapping: &BTreeMap<T, Self>) -> Self {
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

    fn is_equivalent(&self, _other: &Self) -> bool {
        todo!()
    }

    fn is_implied_by(&self, _other: &Self) -> bool {
        todo!()
    }
}
