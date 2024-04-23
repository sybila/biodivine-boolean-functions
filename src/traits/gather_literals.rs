use std::collections::BTreeSet;
use std::fmt::Debug;

pub trait GatherLiterals<TLiteral: Debug + Clone + Eq + Ord> {
    fn gather_literals(&self) -> BTreeSet<TLiteral> {
        let mut result = BTreeSet::new();
        self.gather_literals_rec(&mut result);
        result
    }

    fn gather_literals_rec(&self, current: &mut BTreeSet<TLiteral>);
}
