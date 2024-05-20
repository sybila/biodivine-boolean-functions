use std::ops::{BitOr, Not};

pub trait Equality<Rhs = Self>: BitOr<Rhs> + Not {
    type Output;

    fn iff(self, rhs: Rhs) -> <Self as Equality<Rhs>>::Output;
}
