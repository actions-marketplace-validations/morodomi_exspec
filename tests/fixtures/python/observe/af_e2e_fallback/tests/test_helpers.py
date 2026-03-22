"""Fixture: PY-AF-09 E2E test — all imports incidental, fallback expected.

All imported symbols are used only in setup (assignment) but NOT in assertions.
The assertion-referenced import filter would produce an empty asserted_matched set.
The safe fallback must activate: all_matched is used, ensuring no regression (FN).
"""
from pkg.helpers import HelperA, HelperB


def test_setup_only():
    # Given: setup using both helpers (no assertion references either symbol)
    a = HelperA()
    b = HelperB()
    # When/Then: no assertion about a or b -- assertion is about a derived value
    result = a.do_something() or b.do_something_else()
    # Minimal assertion that does not reference HelperA or HelperB
    assert result is None
