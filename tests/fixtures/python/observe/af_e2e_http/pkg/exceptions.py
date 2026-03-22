"""Fixture: PY-AF-10 — incidental module (exceptions).

Imported in test but NOT the primary subject of assertions.
The assertion filter should suppress this mapping.
"""


class RequestError(Exception):
    """Raised when a request fails."""


class TimeoutError(RequestError):
    """Raised on timeout."""
