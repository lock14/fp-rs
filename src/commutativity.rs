use std::ops::{Add, Mul};

/// The `CommutativeAdd` trait denotes that the `Add` operation is [commutative](https://proofwiki.org/wiki/Definition:Commutative)
pub trait CommutativeAdd: Add<Self, Output = Self>
where
    Self: Sized,
{
}

/// The `CommutativeMul` trait denotes that the `Mul` operation is [commutative](https://proofwiki.org/wiki/Definition:Commutative)
pub trait CommutativeMul: Mul<Self, Output = Self>
where
    Self: Sized,
{
}
