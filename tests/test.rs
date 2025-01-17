use TD2::polynom::polynom_evaluation;

#[test]
fn horner() {
    assert_eq!(polynom_evaluation(5, &[1, 2, 3, 4], 1000), 194); // P(5) % 1000 where P = X^3 + 2X^2 + 3X + 4
}
