use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub trait GatherLiterals<TLiteral: Debug + Clone + Eq + Hash> {
    fn gather_literals(&self) -> HashSet<TLiteral> {
        self.gather_literals_rec(HashSet::new())
    }

    fn gather_literals_rec(&self, current: HashSet<TLiteral>) -> HashSet<TLiteral>;
}
