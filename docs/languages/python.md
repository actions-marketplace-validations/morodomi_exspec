# Python (pytest)

Since v0.1.0. Best coverage among supported languages.

## Test Detection

- Functions starting with `test_`
- Functions decorated with `@pytest.mark.*`
- Functions decorated with `@pytest.fixture` are **excluded** (not test functions)

## Assertions

- `assert` statements
- `pytest.raises` context manager
- `mock.assert_called*`, `mock.assert_any_call`, etc. (`assert_` prefix pattern)

## Known Gaps

- **Nested test functions**: Assertions in functions defined inside test functions are not counted toward the outer test (#41)
- **Helper delegation**: Project-local assertion helpers (e.g. `self.assertValid()`) need `[assertions] custom_patterns`

## Dogfooding Results

| Project | Tests | BLOCK | Notes |
|---------|-------|-------|-------|
| fastapi | 2,121 | 19 | 4 FP (21%): mock.assert_* 3, nested fn 1 |
| pydantic | ~2,500 | 105 | ~47 TP, 43 benchmark(), 15 nested/helper |
| requests | 339 | 14 | Mostly TP |
