def helper():
    return 42


def test_first():
    assert helper() == 42


def test_second():
    result = helper()
    assert result > 0


def test_third():
    x = 1
    y = 2
    assert x + y == 3
