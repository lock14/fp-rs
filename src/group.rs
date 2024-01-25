use crate::associative_quasigroup::AssociativeQuasigroup;
use crate::commutativity::CommutativeAdd;
use crate::monoid::Monoid;
use crate::r#loop::Loop;
use std::ops::Neg;

pub trait Group: Loop + AssociativeQuasigroup + Monoid + Neg<Output = Self> {}

pub trait AbelianGroup: Group + CommutativeAdd {}

macro_rules! group_impl {
    ($($t:ty)*) => ($(
        impl Group for $t {}
    )*)
}

// implement Magma for types that implement Add
// https://doc.rust-lang.org/std/ops/trait.Add.html#implementors

group_impl! { isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! abelian_group_impl {
    ($($t:ty)*) => ($(
        impl AbelianGroup for $t {}
    )*)
}

// implement Magma for types that implement Add
// https://doc.rust-lang.org/std/ops/trait.Add.html#implementors

abelian_group_impl! { isize i8 i16 i32 i64 i128 f32 f64 }
