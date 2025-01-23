use std::u128;

use crate::{
    convert::{bytes_to_chunks, clamp},
    polynom::polynom_evaluation_mod_2_130_5,
};

pub fn poly1305(m: &[u8], r: u128, s: u128) -> u128 {
    s ^ polynom_evaluation_mod_2_130_5(
        u128::from_le_bytes(clamp(r.to_le_bytes())),
        &bytes_to_chunks(m)
            .iter()
            .map(|bytes: &[u8; 16]| u128::from_le_bytes(*bytes))
            .collect::<Vec<_>>()[..],
    )
}
