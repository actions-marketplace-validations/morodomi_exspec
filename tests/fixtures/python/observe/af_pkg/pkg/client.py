"""Fixture: primary production file (Client class)."""


class Client:
    def __init__(self, transport=None):
        self.transport = transport

    def get(self, url):
        return Response(200)

    @property
    def is_ok(self):
        return True


class Response:
    def __init__(self, status_code):
        self.status_code = status_code

    @property
    def ok(self):
        return self.status_code < 400
