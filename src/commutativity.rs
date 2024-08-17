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

macro_rules! commutative_add_impl {
    ($($t:ty)*) => ($(
        impl CommutativeAdd for $t {}
    )*)
}

macro_rules! commutative_mul_impl {
    ($($t:ty)*) => ($(
        impl CommutativeMul for $t {}
    )*)
}

// implement Group and AbelianGroup for types that implement Add
// https://doc.rust-lang.org/std/ops/trait.Add.html#implementors
commutative_add_impl! {f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize}
commutative_mul_impl! {f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize}
