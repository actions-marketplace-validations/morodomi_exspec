# Rust (cargo test)

Since v0.1.0 (Phase 5A).

## Test Detection

- Functions with `#[test]` attribute
- Functions with `#[tokio::test]` attribute
- **Macro-generated tests are NOT detected** (fundamental tree-sitter limitation)

## Assertions

- `assert!`, `assert_eq!`, `assert_ne!` (standard)
- `#[should_panic]` attribute (counted as assertion, detected via sibling walk)
- `.unwrap_err()` (error path indicator for T103)

## Macro Limitation (token_tree)

tree-sitter parses macro bodies as opaque `token_tree` nodes. This affects:

1. **Test functions inside macros**: Not detected (e.g. `rgtest!` in ripgrep)
2. **Custom assertion macros**: Not recognized (e.g. `assert_pending!`, `assert_data_eq!`)
3. **T101 (how-not-what)**: Private field access in macros not detectable
4. **T105 (deterministic-no-metamorphic)**: Relational operators in `assert!()` not detectable

**Workaround** for custom assertion macros:

```toml
[assertions]
custom_patterns = ["assert_pending!", "assert_ready!", "assert_data_eq!"]
```

## T102 (fixture-sprawl)

Smart fixture detection: constructor/struct/macro counted, method calls on locals excluded.

## T103 (missing-error-test)

`.is_err()` removed as weak proxy. Only `#[should_panic]` and `.unwrap_err()` count.

## Dogfooding Results

| Project | Tests | BLOCK | Notes |
|---------|-------|-------|-------|
| exspec (self) | 51 | 0 | 0 FP |

Broader Rust dogfooding (tokio, clap) showed significant macro-related false positives, mitigable with `custom_patterns`.
