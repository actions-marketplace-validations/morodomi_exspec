#[test]
fn test_add() {
    assert_eq!(add(2, 2), 4);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(2, 2), 0);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(2, 3), 6);
}

#[test]
fn test_divide() {
    assert_eq!(divide(4, 2), 2);
}

#[test]
fn test_modulo() {
    assert_eq!(modulo(5, 2), 1);
}
