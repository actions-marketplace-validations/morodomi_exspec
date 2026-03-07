# Cycle: Issue #13 - T101 Private Attribute Access Detection

**Date**: 2026-03-07
**Issue**: #13
**Type**: Feature enhancement

## Goal

T101 how-not-what ルールを拡張し、assertion内の private attribute アクセス (`obj._private`) を検出する。

## Scope

- Python: `assert obj._private == x` を検出
- TypeScript: `expect(obj._private)` / `expect(obj['_private'])` を検出
- core: `count_captures_within_context()` ユーティリティ追加
- メッセージ汎化: "mock verification" -> "implementation-testing"
- PHP/Rust: スコープ外

## Test List

### core/query_utils.rs
- [ ] `count_captures_within_context_basic` - assertion内のprivateアクセス検出
- [ ] `count_captures_within_context_outside` - assertion外は除外
- [ ] `count_captures_within_context_no_outer` - assertionなし→0
- [ ] `count_captures_within_context_missing_capture` - キャプチャ名なし→0

### lang-python
- [ ] `private_in_assertion_detected` - `assert obj._x` を検出
- [ ] `private_outside_assertion_not_counted` - assertion外は除外
- [ ] `dunder_not_counted` - `__class__` は除外
- [ ] `private_adds_to_how_not_what` - mock + private 合算
- [ ] `query_capture_names_private_in_assertion`

### lang-typescript
- [ ] `private_dot_notation_detected` - `expect(obj._x)`
- [ ] `private_bracket_notation_detected` - `expect(obj['_x'])`
- [ ] `private_outside_expect_not_counted`
- [ ] `private_adds_to_how_not_what`
- [ ] `query_capture_names_private_in_assertion`

### rules.rs
- [ ] 既存 `t101_how_not_what_produces_warn` のメッセージ assertion 更新

## Phase: DONE

### RED
- Created fixtures: t101_private_violation.py, t101_private_violation.test.ts
- 8 failing tests confirmed (4 core + 5 Python + 5 TypeScript - 6 trivially passing stubs)

### GREEN
- Implemented `count_captures_within_context()` in core (2-pass byte range matching)
- Created private_in_assertion.scm for Python and TypeScript
- Wired up in both extractors (adds to how_not_what_count)
- Updated T101 message: "mock verification" -> "implementation-testing"
- 371 tests all passing

### REFACTOR
- cargo fmt applied

### REVIEW
- Security: PASS (score 5)
- Correctness: PASS (score 8)
- Overall: PASS
