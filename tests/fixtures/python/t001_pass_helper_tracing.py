# Helper WITH assertion (exactly 1 assert)
def check_result(value):
    assert value > 0


# Helper WITHOUT assertion
def no_assert_helper(value):
    print(value)


# Calls check_result (2-hop chain) but has NO assertion itself
def intermediate(value):
    check_result(value)


# TC-01: calls helper with assertion → assertion_count >= 1
def test_calls_helper_with_assert():
    check_result(42)


# TC-02: calls helper without assertion → assertion_count == 0
def test_calls_helper_without_assert():
    no_assert_helper(42)


# TC-03: has own assert AND calls helper → assertion_count >= 1 (no extra tracing needed)
def test_has_own_assert_plus_helper():
    assert 1 == 1
    check_result(42)


# TC-04: calls undefined function → no crash, assertion_count == 0
def test_calls_undefined_function():
    undefined_function(42)


# TC-05: calls intermediate() which calls check_result() — 2-hop, only 1-hop traced.
# intermediate() has NO assertion → assertion_count stays 0
def test_two_hop_tracing():
    intermediate(42)


# TC-06: test with own assertion — early return path, assertion_count unchanged
def test_with_assertion_early_return():
    assert True


# TC-07: calls check_result() twice → dedup, assertion_count == 1 (not 2)
def test_multiple_calls_same_helper():
    check_result(1)
    check_result(2)
