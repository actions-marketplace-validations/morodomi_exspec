def test_user_creation_calls_repository(mock_repo):
    service = UserService(mock_repo)
    service.create_user("alice")
    mock_repo.save.assert_called_with("alice")
    mock_repo.save.assert_called_once()
    assert service is not None


def test_notification_sent(mock_notifier):
    service = OrderService(mock_notifier)
    service.place_order(item="book")
    mock_notifier.send.assert_called_once_with(item="book")
    assert True
