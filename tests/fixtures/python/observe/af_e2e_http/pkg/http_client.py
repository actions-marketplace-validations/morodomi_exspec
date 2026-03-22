"""Fixture: PY-AF-10 — primary SUT (http_client module).

Simulates a thin wrapper around a third-party HTTP library.
"""


class HttpClient:
    """SUT: the class under test."""

    def __init__(self):
        self._session = None

    def get(self, url: str) -> "HttpResponse":
        return HttpResponse(200, b"ok")

    def post(self, url: str, data: bytes) -> "HttpResponse":
        return HttpResponse(201, b"created")


class HttpResponse:
    def __init__(self, status_code: int, content: bytes):
        self.status_code = status_code
        self.content = content

    @property
    def ok(self) -> bool:
        return self.status_code < 400
