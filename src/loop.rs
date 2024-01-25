use crate::quasigroup::Quasigroup;
use crate::unital_magma::UnitalMagma;

pub trait Loop: Quasigroup + UnitalMagma {}

macro_rules! loop_impl {
    ($($t:ty)*) => ($(
        impl Loop for $t {}
    )*)
}

// implement Magma for types that implement Add
// https://doc.rust-lang.org/std/ops/trait.Add.html#implementors

loop_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }
