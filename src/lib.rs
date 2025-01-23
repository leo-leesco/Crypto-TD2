#![allow(non_snake_case)]

pub const KEY_SIZE: usize = 16;
pub const LIMBS: usize = 5;
pub const LIMB_SIZE: usize = 26;

pub mod bernstein;
pub mod convert;
pub mod polynom;
