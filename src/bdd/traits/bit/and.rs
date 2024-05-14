use crate::bdd::traits::bit::bit_common;
use crate::bdd::Bdd;
use biodivine_lib_bdd::Bdd as InnerBdd;
use std::fmt::Debug;
use std::ops::BitAnd;

impl<TLiteral: Debug + Clone + Eq + Ord + 'static> BitAnd for Bdd<TLiteral> {
    type Output = Bdd<TLiteral>;

    fn bitand(self, rhs: Self) -> Self::Output {
        bit_common(self, rhs, InnerBdd::and)
    }
}
