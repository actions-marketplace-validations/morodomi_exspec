import unittest


class TestMath(unittest.TestCase):
    def test_equality_one(self):
        self.assertEqual(add(1, 2), 3)

    def test_equality_two(self):
        self.assertEqual(subtract(5, 3), 2)

    def test_equality_three(self):
        self.assertEqual(multiply(2, 3), 6)

    def test_equality_four(self):
        self.assertEqual(divide(10, 2), 5)

    def test_greater(self):
        self.assertGreater(compute(10), 0)
