use proptest::prelude::*;

#[test]
fn test_addition_commutative() {
    let a = 42;
    let b = 58;
    prop_assert_eq!(a + b, b + a);
}
