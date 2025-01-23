use std::convert::TryInto;
use std::fmt::Write;
use std::u8;

const KEY_SIZE: usize = 16;

pub fn clamp(mut r: [u8; KEY_SIZE]) -> [u8; KEY_SIZE] {
    r[3] &= 15;
    r[7] &= 15;
    r[11] &= 15;
    r[15] &= 15;
    r[4] &= 252;
    r[8] &= 252;
    r[12] &= 252;
    r
}

// credit to https://stackoverflow.com/a/52992629
fn decode_hex(s: &str) -> Option<Vec<u8>> {
    if let Ok(vec) = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], KEY_SIZE))
        .collect()
    {
        Some(vec)
    } else {
        None
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

// credit to https://stackoverflow.com/a/29570662
fn into_array<T, const N: usize>(v: Vec<T>) -> Option<[T; N]> {
    if let Ok(arr) = v.try_into() {
        Some(arr)
    } else {
        None
    }
}

pub fn extract_rs(rs: &str) -> ([u8; KEY_SIZE], [u8; KEY_SIZE]) {
    assert_eq!(rs.len(), 2 * KEY_SIZE);
    (
        u128::from_str_radix(&rs[..KEY_SIZE], 16),
        u128::from_str_radix(&rs[KEY_SIZE..], 16),
    )
}
