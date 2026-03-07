def test_mock_and_hardcoded():
    mock.assert_called()
    assert add(1, 2) == 3
