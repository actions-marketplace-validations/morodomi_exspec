from unittest.mock import MagicMock


def test_assert_called_once():
    mock = MagicMock()
    mock("hello")
    mock.assert_called_once()


def test_assert_called_once_with():
    mock = MagicMock()
    mock("hello", key="value")
    mock.assert_called_once_with("hello", key="value")


def test_assert_not_called():
    mock = MagicMock()
    mock.assert_not_called()


def test_assert_has_calls():
    mock = MagicMock()
    mock(1)
    mock(2)
    from unittest.mock import call
    mock.assert_has_calls([call(1), call(2)])


def test_chained_mock_assert():
    mock = MagicMock()
    mock.return_value.process("data")
    mock.return_value.assert_called_once()
