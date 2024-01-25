use crate::quasigroup::Quasigroup;
use crate::semigroup::Semigroup;

pub trait AssociativeQuasigroup: Quasigroup + Semigroup {}

macro_rules! associative_quasigroup_impl {
    ($($t:ty)*) => ($(
        impl AssociativeQuasigroup for $t {}
    )*)
}

// implement Magma for types that implement Add
// https://doc.rust-lang.org/std/ops/trait.Add.html#implementors

associative_quasigroup_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }
