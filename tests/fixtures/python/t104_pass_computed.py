def test_roundtrip():
    data = "hello"
    assert decode(encode(data)) == data
