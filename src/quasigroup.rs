use crate::magma::Magma;
use std::ops::Sub;

pub trait Quasigroup: Magma + Sub<Self, Output = Self> {}

macro_rules! quasigroup_impl {
    ($($t:ty)*) => ($(
        impl Quasigroup for $t {}
    )*)
}

// implement Magma for types that implement Add
// https://doc.rust-lang.org/std/ops/trait.Add.html#implementors

quasigroup_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }
