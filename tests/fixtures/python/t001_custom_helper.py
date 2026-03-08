def test_with_custom_helper():
    result = compute(42)
    util.assertEqual(result, 42)


def test_with_standard_assert():
    result = compute(1)
    assert result == 1


def test_no_assertion_at_all():
    result = compute(0)
    print(result)
