use crate::bdd::Bdd;
use std::fmt::Debug;
use std::ops::BitAnd;

impl<TLiteral: Debug + Clone + Eq + Ord + 'static> BitAnd for Bdd<TLiteral> {
    type Output = Bdd<TLiteral>;

    fn bitand(self, rhs: Self) -> Self::Output {
        if self.inputs == rhs.inputs {
            Bdd::new(self.bdd.and(&rhs.bdd), self.inputs.clone())
        } else {
            let (self_lifted, rhs_lifted, common_inputs) = self.union_and_extend(&rhs);

            Bdd::new(self_lifted.bdd.and(&rhs_lifted.bdd), common_inputs)
        }
    }
}
