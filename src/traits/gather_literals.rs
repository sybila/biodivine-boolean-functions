use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub trait GatherLiterals<TLiteral: Debug + Clone + Eq + Hash> {
    fn gather_literals(&self) -> HashSet<TLiteral> {
        let mut result = HashSet::new();
        self.gather_literals_rec(&mut result);
        result
    }

    fn gather_literals_rec(&self, current: &mut HashSet<TLiteral>);
}
