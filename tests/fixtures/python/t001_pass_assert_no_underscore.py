def test_assert_no_underscore():
    """obj.assertX() without underscore should count as assertion."""
    reprec = get_reprec()
    reprec.assertoutcome(passed=1)


def test_assert_status():
    """obj.assertStatus() should count as assertion."""
    response = get_response()
    response.assertStatus(200)
