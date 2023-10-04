use crate::quasigroup::Quasigroup;
use crate::semigroup::Semigroup;

pub trait AssociativeQuasigroup: Quasigroup + Semigroup {}

impl AssociativeQuasigroup for f32 {}
impl AssociativeQuasigroup for f64 {}
impl AssociativeQuasigroup for i8 {}
impl AssociativeQuasigroup for i16 {}
impl AssociativeQuasigroup for i32 {}
impl AssociativeQuasigroup for i64 {}
impl AssociativeQuasigroup for i128 {}
impl AssociativeQuasigroup for isize {}
impl AssociativeQuasigroup for u8 {}
impl AssociativeQuasigroup for u16 {}
impl AssociativeQuasigroup for u32 {}
impl AssociativeQuasigroup for u64 {}
impl AssociativeQuasigroup for u128 {}
impl AssociativeQuasigroup for usize {}
