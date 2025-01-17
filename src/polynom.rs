/// coefficients should be ordered the highest degree to lowest
pub fn polynom_evaluation(x: u32, coefficients: &[u32], p: u32) -> u32 {
    coefficients
        .iter()
        .fold(0, |evaluation, coef| (x * evaluation + coef) % p)
}
