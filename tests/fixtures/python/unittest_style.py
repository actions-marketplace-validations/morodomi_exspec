import unittest


class TestUser(unittest.TestCase):
    def test_create_user(self):
        user = create_user("Alice")
        self.assertEqual(user.name, "Alice")
        self.assertTrue(user.active)
