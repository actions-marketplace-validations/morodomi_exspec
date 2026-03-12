// T1: Simple assert_* function call (helper delegation)
#[test]
fn test_simple_helper() {
    let result = compute();
    assert_matches(result, Expected::Value(42));
}

// T2: Scoped assert_* function call (module::helper delegation)
#[test]
fn test_scoped_helper() {
    let result = compute();
    common::assert_matches(result, Expected::Value(42));
}

// T3: Mixed macro + function call
#[test]
fn test_mixed_macro_and_fn() {
    let result = compute();
    assert_eq!(result.len(), 1);
    assert_matches(result, Expected::Value(42));
}
