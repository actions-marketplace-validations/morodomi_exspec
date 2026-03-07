import unittest
from unittest.mock import MagicMock

class TestUser(unittest.TestCase):
    def test_mock_obj_assert_raises(self):
        mock_obj = MagicMock()
        mock_obj.assertRaises(ValueError)
        mock_obj.assertRaisesRegex(ValueError, "msg")
        mock_obj.assertWarns(DeprecationWarning)
        mock_obj.assertWarnsRegex(DeprecationWarning, "msg")
