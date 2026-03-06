from unittest.mock import MagicMock


def test_with_one_mock():
    mock_db = MagicMock()
    result = query(mock_db)
    assert result == []
