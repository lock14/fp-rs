use crate::magma::Magma;
use std::ops::Sub;

pub trait Quasigroup: Magma + Sub<Self, Output = Self> {}

impl Quasigroup for f32 {}
impl Quasigroup for f64 {}
impl Quasigroup for i8 {}
impl Quasigroup for i16 {}
impl Quasigroup for i32 {}
impl Quasigroup for i64 {}
impl Quasigroup for i128 {}
impl Quasigroup for isize {}
impl Quasigroup for u8 {}
impl Quasigroup for u16 {}
impl Quasigroup for u32 {}
impl Quasigroup for u64 {}
impl Quasigroup for u128 {}
impl Quasigroup for usize {}
