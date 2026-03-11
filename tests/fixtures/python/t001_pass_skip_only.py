import pytest
import unittest


def test_skipped_feature():
    pytest.skip("Not supported on this platform")


class TestSkipOnly(unittest.TestCase):
    def test_incomplete(self):
        self.skipTest("Not implemented yet")
