use crate::associative_quasigroup::AssociativeQuasigroup;
use crate::commutativity::CommutativeAdd;
use crate::monoid::Monoid;
use crate::r#loop::Loop;
use std::ops::Neg;

pub trait Group: Loop + AssociativeQuasigroup + Monoid + Neg<Output = Self> {}

pub trait AbelianGroup: Group + CommutativeAdd {}

impl Group for f32 {}
impl Group for f64 {}
impl Group for i8 {}
impl Group for i16 {}
impl Group for i32 {}
impl Group for i64 {}
impl Group for i128 {}
impl Group for isize {}

impl AbelianGroup for f32 {}
impl AbelianGroup for f64 {}
impl AbelianGroup for i8 {}
impl AbelianGroup for i16 {}
impl AbelianGroup for i32 {}
impl AbelianGroup for i64 {}
impl AbelianGroup for i128 {}
impl AbelianGroup for isize {}
