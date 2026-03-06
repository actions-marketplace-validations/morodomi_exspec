import pytest


@pytest.mark.parametrize("a,b,expected", [
    (1, 1, 2),
    (2, 2, 4),
    (3, 3, 6),
])
def test_add(a, b, expected):
    assert a + b == expected


def test_subtract():
    assert 3 - 1 == 2
