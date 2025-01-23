//! Calculate polynomial evaluation modulo $2^130-5$

use crate::{
    convert::{from_limb, to_limb},
    LIMBS, LIMB_SIZE,
};

/// coefficients should be ordered the highest degree to lowest
/// the polynom evaluation is evaluated modulo $2^130-5$
pub fn polynom_evaluation(x: u128, coefficients: &[u128]) -> u128 {
    let x = to_limb(x);
    from_limb(
        coefficients
            .iter()
            .map(|a: &u128| to_limb(*a))
            .fold([0u32; LIMBS], |evaluation, coef| {
                add(evaluation, mult(x, coef))
            }),
    )
}

fn mult_without_carry(a: [u32; LIMBS], b: [u32; LIMBS]) -> [u64; LIMBS] {
    let mut p = [0u64; LIMBS];
    for i in 0..LIMBS {
        p[i] = a
            .iter()
            .zip(b.iter().rev())
            .enumerate()
            .fold(0, |limb, (j, (&aj, &bk))| {
                limb + ({
                    {
                        aj * bk * {
                            if j > i {
                                5
                            } else {
                                1
                            }
                        }
                    }
                } as u64)
                    + limb
            })
    }
    p
}

/// updates product_result and returns the remaining carry
fn propagate_carry(product_result: &mut [u64; LIMBS], mut carry: u64) -> u64 {
    for i in 0..LIMBS {
        product_result[i] += carry * {
            if i == 0 {
                5
            } else {
                1
            }
        };
        carry = product_result[i] >> LIMB_SIZE;
        product_result[i] -= carry << LIMB_SIZE;
    }
    carry
}

fn mult(a: [u32; LIMBS], b: [u32; LIMBS]) -> [u32; LIMBS] {
    let mut carry = 0u64;
    let mut product = mult_without_carry(a, b);
    while {
        carry = propagate_carry(&mut product, carry);
        carry != 0
    } {}

    product.map(|limb| limb as u32)
}

fn add(a: [u32; LIMBS], b: [u32; LIMBS]) -> [u32; LIMBS] {
    a.iter()
        .zip(b)
        .map(|(ai, bi)| ai + bi)
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn horner() {
        assert_eq!(polynom_evaluation(5, &[1, 2, 3, 4]), 194);
        // P(5) % 1000 where P = X^3 + 2X^2 + 3X + 4
    }
}
