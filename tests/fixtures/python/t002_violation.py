from unittest.mock import MagicMock, patch


@patch("module.ServiceA")
@patch("module.ServiceB")
@patch("module.ServiceC")
def test_overuse_mocks(mock_service_c, mock_service_b, mock_service_a):
    mock_db = MagicMock()
    mock_cache = MagicMock()
    mock_logger = MagicMock()
    result = do_something(mock_db, mock_cache, mock_logger)
    assert result is not None
