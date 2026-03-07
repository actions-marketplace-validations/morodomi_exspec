def test_user_creation_returns_user():
    service = UserService()
    user = service.create_user("alice")
    assert user.name == "alice"
    assert user.active is True
