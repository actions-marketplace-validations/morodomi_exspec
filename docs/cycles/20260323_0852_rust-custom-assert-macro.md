---
feature: rust-custom-assert-macro
cycle: 20260323_0852
phase: DONE
complexity: trivial
test_count: 4
risk_level: low
codex_session_id: ""
created: 2026-03-23 08:52
updated: 2026-03-23 09:20
---

# Phase 22: Rust custom assert macro auto-detection

## Scope Definition

### In Scope
- [ ] `crates/lang-rust/queries/assertion.scm` Pattern 1 の正規表現を prefix matching に変更
- [ ] `tests/fixtures/rust/t001_pass_custom_assert_macro.rs` テストfixture追加
- [ ] 既存テストファイルに統合テスト追加（カスタムアサートマクロが assertion としてカウントされることを検証）

### Out of Scope
- T107 message counting (`count_assertion_messages_rust`): ジェネリック実装済みのため変更不要
- `custom_patterns` (`query_utils.rs`): non-`assert_*` マクロ用に維持、変更不要
- Pattern 2, 3 (function call `assert_*()`): 既に `^assert_` でジェネリック、変更不要

### Files to Change (target: 10 or less)
- `crates/lang-rust/queries/assertion.scm` (edit)
- `tests/fixtures/rust/t001_pass_custom_assert_macro.rs` (new)
- `crates/lang-rust/src/lib.rs` (edit — 統合テスト追加)

## Environment

### Scope
- Layer: Backend
- Plugin: python-quality (n/a — Rust project)
- Risk: 15 (PASS)

### Runtime
- Language: Rust (cargo)

### Dependencies (key packages)
- tree-sitter: workspace
- tree-sitter-rust: workspace

### Risk Interview (BLOCK only)
N/A (PASS判定)

## Context & Dependencies

### Reference Documents
- `crates/lang-rust/queries/assertion.scm` - 変更対象のtree-sitterクエリ
- `CONSTITUTION.md` - BLOCK near-zero FP の検出哲学
- `docs/dogfooding-results.md` - tokio/clap FP データ

### Dependent Features
- T001 (No Assertion): `crates/core/src/rules.rs` で assertion_count を使用

### Related Issues/PRs
- FP削減: tokio -124 FP、clap -115 FP (dogfooding観測値)

## Test List

### TODO
- [ ] TC-01: `assert_pending!(val)` が assertion としてカウントされる（正常系・カスタムマクロ）
- [ ] TC-02: `assert_ready_ok!(future)` が assertion としてカウントされる（正常系・カスタムマクロ）
- [ ] TC-03: `assert_data_eq!(actual, expected)` が assertion としてカウントされる（正常系・カスタムマクロ）
- [ ] TC-04: 既存の `assert!`, `assert_eq!`, `assert_ne!`, `prop_assert_eq!`, `debug_assert!` が引き続き assertion としてカウントされる（回帰確認）

### WIP
(none)

### DISCOVERED
(none)

### DONE
(none)

## Implementation Notes

### Goal

Rust dogfooding (tokio, clap) で T001 BLOCK の主要 FP 原因となっている `assert_pending!`, `assert_ready!`, `assert_data_eq!` 等のカスタム assertion マクロを自動検出する。`assert_*` プレフィックスマッチングにより、ユーザーが `custom_patterns` で手動設定せずとも FP を解消する。

### Background

現在の `assertion.scm` は標準マクロ9種のみをハードコード:
```
^(assert|assert_eq|assert_ne|debug_assert|debug_assert_eq|debug_assert_ne|prop_assert|prop_assert_eq|prop_assert_ne)$
```
末尾 `$` アンカーにより、`assert_pending!` 等のカスタムマクロが検出されない。

### Design Approach

**Pattern 1 修正**: `assertion.scm` の1行変更

Before:
```scm
(macro_invocation
  macro: (identifier) @_name
  (#match? @_name "^(assert|assert_eq|assert_ne|debug_assert|debug_assert_eq|debug_assert_ne|prop_assert|prop_assert_eq|prop_assert_ne)$")) @assertion
```

After:
```scm
(macro_invocation
  macro: (identifier) @_name
  (#match? @_name "^(assert|debug_assert|prop_assert)")) @assertion
```

- `$` アンカー削除でプレフィックスマッチングに変更
- 明示的バリアントリスト（`assert_eq`, `assert_ne` 等）を除去（共通プレフィックスで包含済み）
- `debug_assert` と `prop_assert` は `assert` で始まらないため明示的に残す

**FP リスク**: `assert_impl!` 等の型制約マクロが assertion として誤カウントされる可能性。ただし T001 が BLOCK→PASS に変わるだけ（false negative 方向）で害は小さい。dogfooding データでは観測されていない。

## Progress Log

### 2026-03-23 08:52 - INIT
- Cycle doc created from plan file `kind-inventing-crab.md`
- Design Review Gate: PASS (score 15/100)
- Scope definition ready

### 2026-03-23 09:00 - RED
- Fixture: `tests/fixtures/rust/t001_pass_custom_assert_macro.rs` (4 test functions)
- Tests: TC-01〜03 FAIL (assertion_count == 0), TC-04 PASS (regression guard)
- RED state verified

### 2026-03-23 09:05 - GREEN
- `assertion.scm` Pattern 1: `$` anchor removed, prefix match enabled
- All 1091 tests passed, clippy 0, self-dogfooding BLOCK 0

### 2026-03-23 09:10 - REFACTOR
- Checklist: 7/7 reviewed, no changes needed
- Verification Gate: PASS (725 tests, clippy 0, fmt OK). Phase completed

### 2026-03-23 09:20 - REVIEW
- Correctness reviewer: `^assert` matches `assertion!` → regex tightened to `assert(_|$)`
- TC-05 追加: `assertion!()` が assertion としてカウントされないことを検証
- 全726テスト通過、clippy 0。PASS (score ~20). Phase completed

---

## Next Steps

1. [Done] INIT
2. [Done] PLAN (plan file approved)
3. [Done] RED
4. [Done] GREEN
5. [Done] REFACTOR
6. [Done] REVIEW
7. [Done] COMMIT
