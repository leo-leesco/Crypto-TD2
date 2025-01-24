#![allow(non_snake_case)]

const KEY_SIZE: usize = 16;
const LIMBS: usize = 5;
const LIMB_SIZE: usize = 26;

pub mod bernstein;
pub mod convert;
mod polynom;
