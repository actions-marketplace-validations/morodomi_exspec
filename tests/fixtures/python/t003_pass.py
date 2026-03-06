def test_short_function():
    user = create_user("Bob")
    assert user.name == "Bob"
    assert user.active is True
