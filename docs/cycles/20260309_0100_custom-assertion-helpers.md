# Cycle: #33 Custom Assertion Helpers via .exspec.toml

**Issue**: #33
**Date**: 2026-03-09
**Status**: DONE

## Goal

Add `.exspec.toml` config-based T001 escape hatch for custom assertion helpers (e.g., `util.assertEqual()`, `myAssert()`). Projects with custom helpers get T001 false positives because these helpers aren't in the built-in tree-sitter .scm queries.

## Approach

Post-extraction text supplement: augment `assertion_count` in the CLI layer after extraction, before rule evaluation. Only functions with `assertion_count == 0` are processed. Substring matching (not regex) by design.

## Changes

| File | Change |
|------|--------|
| `crates/core/src/config.rs` | `AssertionsConfig` struct, wired through `From<ExspecConfig>` |
| `crates/core/src/rules.rs` | `custom_assertion_patterns: Vec<String>` on `Config` |
| `crates/core/src/query_utils.rs` | `count_custom_assertion_lines()` + `apply_custom_assertion_fallback()` |
| `crates/cli/src/main.rs` | Call fallback after extraction, before rule evaluation |
| Fixtures | `custom_assertions.toml`, `t001_custom_helper.py`, `t001_custom_helper.test.ts` |

## Test Cases (18 new, 537 -> 555)

- TC-01~03: Config parsing + conversion
- TC-04~08, TC-16: `count_custom_assertion_lines` unit tests
- TC-09~10: `apply_custom_assertion_fallback` unit tests
- TC-11~15: Integration tests (Python + TypeScript + edge cases)

## Review

- Security: 8/100 (PASS)
- Correctness: 8/100 (PASS)
- Lint: PASS (clippy 0, fmt clean)
- DISCOVERED: #36 (empty string filter, test coverage gap, docs, T006 interaction)

## Design Decisions

- Text match, not AST re-parse: custom patterns are user-configured function names, substring matching is sufficient
- No comment/string/import filtering: intentionally simple escape hatch, documented behavior
- No LanguageExtractor trait change: trait doesn't take Config, changing it is a large breaking change
- Line count, not occurrence count: consistent with escape hatch philosophy
