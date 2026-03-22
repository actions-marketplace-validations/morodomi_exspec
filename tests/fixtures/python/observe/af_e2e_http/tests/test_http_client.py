"""Fixture: PY-AF-10 E2E — third-party-http-client-like chain tracking.

Imports:
  - HttpClient (from pkg.http_client) -- asserted via chain: response -> client -> HttpClient
  - RequestError (from pkg.exceptions) -- appears in pytest.raises(), which IS an
    assertion node per assertion.scm, so RequestError is in asserted_imports too.

Expected after assertion filter:
  - http_client.py -> kept (HttpClient is traced through 2-hop chain)
  - exceptions.py -> also kept (RequestError is in assertion context via pytest.raises)

This fixture primarily tests the chain tracking path (response -> client -> HttpClient).
"""
import pytest
from pkg.http_client import HttpClient
from pkg.exceptions import RequestError


def test_get_returns_ok_response():
    # Given: an HttpClient instance
    client = HttpClient()
    # When: making a GET request
    response = client.get("http://example.com/")
    # Then: response is ok (chain: response -> client.get() -> HttpClient)
    assert response.ok


def test_post_returns_created():
    # Given: an HttpClient instance
    client = HttpClient()
    # When: making a POST request
    response = client.post("http://example.com/", b"data")
    # Then: status code is 201
    assert response.status_code == 201


def test_invalid_url_raises():
    # Given: an HttpClient instance
    client = HttpClient()
    # When/Then: invalid URL raises RequestError
    with pytest.raises(RequestError):
        client.get("not-a-url")
