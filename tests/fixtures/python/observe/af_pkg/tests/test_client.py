"""Fixture: PY-AF-08 E2E test.

Imports:
  - Client (from pkg.client) -- asserted: assert client.is_ok
  - MockTransport (from pkg.transport) -- NOT asserted (setup only)

Expected after assertion filter:
  - client.py -> kept (Client is asserted)
  - transport.py -> filtered (MockTransport not in assertion)
"""
from pkg.client import Client
from pkg.transport import MockTransport


def test_client_ok():
    # Given: a client with mock transport (incidental import, setup only)
    transport = MockTransport()
    client = Client(transport=transport)
    # When: checking client status
    # Then: assert references Client's attribute (not MockTransport)
    assert client.is_ok
