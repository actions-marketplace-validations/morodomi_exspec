"""Fixture: incidental production file (MockTransport class).

This file is imported in the test but not referenced in assertions.
It should be filtered out by the assertion-referenced import filter.
"""


class MockTransport:
    """A mock transport used only for setup, not directly asserted."""

    def handle_request(self, request):
        pass
