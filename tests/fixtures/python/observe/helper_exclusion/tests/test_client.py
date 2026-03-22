from pkg.client import Client
from tests.helpers import mock_client

def test_connect():
    client = Client()
    assert client.connect()

def test_with_mock():
    mc = mock_client()
    assert mc == "mock"
