use crate::quasigroup::Quasigroup;
use crate::unital_magma::UnitalMagma;

pub trait Loop: Quasigroup + UnitalMagma {}

impl Loop for f32 {}
impl Loop for f64 {}
impl Loop for i8 {}
impl Loop for i16 {}
impl Loop for i32 {}
impl Loop for i64 {}
impl Loop for i128 {}
impl Loop for isize {}
impl Loop for u8 {}
impl Loop for u16 {}
impl Loop for u32 {}
impl Loop for u64 {}
impl Loop for u128 {}
impl Loop for usize {}
