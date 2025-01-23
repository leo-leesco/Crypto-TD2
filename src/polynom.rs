// credit to https://docs.rs/fast_polynomial/latest/fast_polynomial/#motivation
/// coefficients should be ordered the highest degree to lowest
pub fn polynom_evaluation(x: u32, coefficients: &[u32], p: u32) -> u32 {
    coefficients
        .iter()
        .fold(0, |evaluation, coef| (x * evaluation + coef) % p)
}

#[cfg(test)]
mod test {

    #[test]
    fn horner() {
        assert_eq!(polynom_evaluation(5, &[1, 2, 3, 4], 1000), 194); // P(5) % 1000 where P = X^3 + 2X^2 + 3X + 4
    }
}
