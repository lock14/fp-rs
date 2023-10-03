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

impl CommutativeAdd for f32 {}
impl CommutativeAdd for f64 {}
impl CommutativeAdd for i8 {}
impl CommutativeAdd for i16 {}
impl CommutativeAdd for i32 {}
impl CommutativeAdd for i64 {}
impl CommutativeAdd for i128 {}
impl CommutativeAdd for isize {}
impl CommutativeAdd for u8 {}
impl CommutativeAdd for u16 {}
impl CommutativeAdd for u32 {}
impl CommutativeAdd for u64 {}
impl CommutativeAdd for u128 {}
impl CommutativeAdd for usize {}

impl CommutativeMul for f32 {}
impl CommutativeMul for f64 {}
impl CommutativeMul for i8 {}
impl CommutativeMul for i16 {}
impl CommutativeMul for i32 {}
impl CommutativeMul for i64 {}
impl CommutativeMul for i128 {}
impl CommutativeMul for isize {}
impl CommutativeMul for u8 {}
impl CommutativeMul for u16 {}
impl CommutativeMul for u32 {}
impl CommutativeMul for u64 {}
impl CommutativeMul for u128 {}
impl CommutativeMul for usize {}
