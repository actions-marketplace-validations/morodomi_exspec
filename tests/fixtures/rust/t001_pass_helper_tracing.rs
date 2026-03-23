// Helper with assertion — should be traced
fn check_result(val: i32) {
    assert_eq!(val, 42);
}

// Helper WITHOUT assertion — should NOT add to assertion_count
fn setup_data() -> i32 {
    42
}

// 2-hop: calls another helper that has assertion
fn intermediate_helper(val: i32) {
    check_result(val);  // 2nd hop — should NOT be traced
}

fn compute() -> i32 {
    42
}

fn nonexistent_function() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // TC-01: calls helper with assertion → assertion_count >= 1
    #[test]
    fn test_delegates_to_helper_with_assertion() {
        let result = compute();
        check_result(result);
    }

    // TC-02: calls helper WITHOUT assertion → assertion_count == 0
    #[test]
    fn test_delegates_to_helper_without_assertion() {
        let _data = setup_data();
        // no assertion, helper has no assertion either
    }

    // TC-03: has direct assertion + calls helper → already assertion_count > 0, no extra tracing
    #[test]
    fn test_has_own_assertion_and_calls_helper() {
        assert_eq!(1, 1);
        check_result(42);
    }

    // TC-04: calls undefined function → no crash, assertion_count stays 0
    #[test]
    fn test_calls_undefined_function() {
        let _result = nonexistent_function();
    }

    // TC-05: calls intermediate helper (2-hop) → only 1-hop traced
    // intermediate_helper calls check_result, but we only trace 1 hop
    // intermediate_helper itself has NO assertion → assertion_count stays 0
    #[test]
    fn test_two_hop_not_traced() {
        intermediate_helper(42);
    }

    // TC-07: calls same helper multiple times
    // Should count helper assertions once (deduplicated), not once per call
    #[test]
    fn test_calls_helper_twice() {
        check_result(1);
        check_result(2);
    }
}
