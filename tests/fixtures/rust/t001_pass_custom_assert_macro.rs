#[cfg(test)]
mod tests {
    #[test]
    fn test_with_assert_pending() {
        let val = some_fn();
        assert_pending!(val);
    }

    #[test]
    fn test_with_assert_ready_ok() {
        let future = create_future();
        assert_ready_ok!(future);
    }

    #[test]
    fn test_with_assert_data_eq() {
        let actual = get_data();
        assert_data_eq!(actual, expected);
    }

    #[test]
    fn test_with_multiple_custom_asserts() {
        assert_pending!(val);
        assert_ready_ok!(future);
    }
}

// Separate test module to verify false positive with "assertion!" macro
// The regex ^(assert|debug_assert|prop_assert) will match "assertion"
// because it starts with "assert" — this is a SECURITY/CORRECTNESS ISSUE
#[test]
fn test_with_assertion_not_assert() {
    // Using "assertion!" (not "assert!")
    // Current regex WILL incorrectly match this as an assertion
    assertion!(true);
}
