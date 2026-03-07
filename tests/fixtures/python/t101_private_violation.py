def test_checks_internal_count(service):
    service.process()
    assert service._count == 1
    assert service._processed is True


def test_mixed_private_and_mock(mock_repo):
    service = UserService(mock_repo)
    service.create_user("alice")
    mock_repo.save.assert_called_with("alice")
    assert service._last_created == "alice"


def test_dunder_not_private():
    obj = MyClass()
    assert obj.__class__.__name__ == "MyClass"
    assert obj.__dict__ is not None


def test_private_outside_assertion():
    obj = MyClass()
    value = obj._internal
    assert value == 42
