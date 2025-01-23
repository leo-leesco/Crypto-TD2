use std::convert::TryInto;
use std::u8;

use crate::KEY_SIZE;

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

pub fn extract_rs(rs: &str) -> (u128, u128) {
    assert_eq!(rs.len(), 2 * KEY_SIZE);
    (
        u128::from_str_radix(&rs[..KEY_SIZE], 16).unwrap(), // possibly reverse using .swap_bytes()
        u128::from_str_radix(&rs[KEY_SIZE..], 16).unwrap(), // if key is little-endian
    )
}

fn pad_chunk(chunk: &[u8]) -> [u8; KEY_SIZE] {
    assert!(chunk.len() <= KEY_SIZE);
    let mut out = [0u8; KEY_SIZE];

    out[..chunk.len()].copy_from_slice(chunk);
    if chunk.len() < KEY_SIZE {
        out[KEY_SIZE - 1] = 0x01;
    }
    out
}

pub fn bytes_to_chunks(bytes: &[u8]) -> Vec<[u8; KEY_SIZE]> {
    bytes
        .chunks(KEY_SIZE)
        .map(pad_chunk)
        .collect::<Vec<[u8; KEY_SIZE]>>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::convert::extract_rs;

    #[test]
    fn hexstring() {
        assert_eq!(
            usize::from_str_radix("1234", 16).unwrap(),
            16usize.pow(3) + 2 * 16usize.pow(2) + 3 * 16 + 4
        );
    }

    #[test]
    #[should_panic]
    #[ignore = "dropped swapping bytes + probable integer overflow"]
    fn read_r() {
        let be_r: u128 = 0x12_00_00_00_00_00_00_00_00_00_00_00_00_00_00_34;
        let le_r: u128 = 0x34_00_00_00_00_00_00_00_00_00_00_00_00_00_00_12;
        eprintln!("{be_r:x}");
        assert_eq!(extract_rs(&format!("{be_r:0<64}")).0, le_r);
    }

    #[test]
    //#[should_panic = "testing largest value"]
    fn max_u128() {
        eprintln!("{:x}", u128::MAX);
        panic!("testing largest value");
    }
}
