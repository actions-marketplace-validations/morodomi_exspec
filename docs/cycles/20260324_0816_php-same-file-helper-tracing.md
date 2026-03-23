---
feature: php-same-file-helper-tracing
cycle: 20260324_0816
phase: DONE
complexity: trivial
test_count: 7
risk_level: low
codex_session_id: ""
created: 2026-03-24 08:16
updated: 2026-03-24 RED-complete
---

# #152 Same-file helper tracing for PHP (Phase 23b port)

## Scope Definition

### In Scope
- [ ] `crates/lang-php/queries/helper_trace.scm` 新規作成
- [ ] `crates/lang-php/src/lib.rs` に OnceLock cache + `apply_same_file_helper_tracing()` 呼び出し追加
- [ ] `tests/fixtures/php/t001_pass_helper_tracing.php` 新規作成 (TC-01 ~ TC-07)
- [ ] 統合テスト追加 (lang-php lib.rs の #[cfg(test)] 内)

### Out of Scope
- `$this->helper()` メソッド呼び出しトレース (Reason: `member_call_expression` は `(name)` にマッチしない。same-file では free function のみ対象。#153 で対応)
- 2-hop 以上のトレース (Reason: 1-hop only の設計制約)
- laravel/symfony の BLOCK FP 大幅削減 (Reason: 大半が `$this->` パターンのため same-file の直接削減効果は限定的。cross-file #153 のベースライン確立が主目的)

### Files to Change (target: 10 or less)
- `crates/lang-php/queries/helper_trace.scm` (new)
- `crates/lang-php/src/lib.rs` (edit)
- `tests/fixtures/php/t001_pass_helper_tracing.php` (new)

## Environment

### Scope
- Layer: Backend (Rust CLI tool)
- Plugin: rust
- Risk: 10 (PASS)

### Runtime
- Language: Rust (cargo)

### Dependencies (key packages)
- tree-sitter: workspace
- exspec-core: workspace (apply_same_file_helper_tracing)

### Risk Interview (BLOCK only)
N/A — LOW risk, no blocking issues.

## Context & Dependencies

### Reference Documents
- `crates/core/src/query_utils.rs` — `apply_same_file_helper_tracing()` 実装 (そのまま使用)
- `crates/lang-python/queries/helper_trace.scm` — テンプレート
- `crates/lang-python/src/lib.rs` — 統合パターンのテンプレート
- `crates/lang-typescript/queries/helper_trace.scm` — TypeScript版 (#151)
- `crates/lang-typescript/src/lib.rs` — TypeScript統合パターン (#151)
- `ROADMAP.md` v0.4.3: #152 Same-file helper tracing: PHP

### Dependent Features
- Phase 23a (Rust): `crates/core/src/query_utils.rs` — `apply_same_file_helper_tracing()` 提供元
- #150 (Python): 確立済みパターン
- #151 (TypeScript): 確立済みパターン

### Related Issues/PRs
- Issue #152: Same-file helper tracing: PHP
- #150 (Python port): 確立済み
- #151 (TypeScript port): 確立済み
- #153 (cross-file / メソッド呼び出し): 将来対応

## Test List

### TODO
- [ ] TC-01: helper with assertion → test calls helper → assertion_count >= 1
- [ ] TC-02: helper without assertion → test calls helper → assertion_count == 0
- [ ] TC-03: test has own assertion + calls helper → assertion_count >= 1 (no extra tracing)
- [ ] TC-04: test calls undefined function → assertion_count == 0 (no crash)
- [ ] TC-05: 2-hop: test → intermediate → check_result → assertion_count == 0 (1-hop only)
- [ ] TC-06: test with own assertion → early return → assertion_count unchanged
- [ ] TC-07: multiple calls to same helper → dedup → assertion_count == 1

### WIP
(none)

### DISCOVERED
(none)

### DONE
(none)

## Implementation Notes

### Goal
Phase 23a (Rust) → #150 (Python) → #151 (TypeScript) と同じ same-file helper tracing を PHP にポートする。Phase 23b の最後の言語ポート。cross-file (#153) の Go/No-Go 判定用ベースライン確立が主目的。

### Background
`core` の `apply_same_file_helper_tracing()` は言語非依存。Python/TypeScript ポートで確立したパターンを PHP に適用する。PHP は helper delegation が最大の BLOCK FP 源 (laravel 222, symfony 616) だが、大半が `$this->method()` (cross-file) のため same-file での削減幅は限定的。

PHP 固有ノード名: `function_call_expression` + `(name)` (Python の `identifier` / TS の `identifier` とは異なる)。body は `compound_statement` (Python の `block` / TS の `statement_block` に相当)。

### Design Approach

#### `helper_trace.scm` (新規)
```scm
; Function calls (free function — helper call in test body)
(function_call_expression function: (name) @call_name)

; Function definitions (helper function with body)
(function_definition
  name: (name) @def_name
  body: (compound_statement) @def_body)
```

#### `lib.rs` 変更 (3点)
1. import: `use exspec_core::query_utils::apply_same_file_helper_tracing;`
2. 定数 + OnceLock: `HELPER_TRACE_QUERY` / `HELPER_TRACE_QUERY_CACHE`
3. `extract_file_analysis()` にて FileAnalysis 構築後・return 前に `apply_same_file_helper_tracing()` 呼び出し

Note: helper_trace.scm は `@call_name` と `@def_name/@def_body` を単一クエリで持つ。同一オブジェクトを call_query / def_query 両方に渡す設計 (Python/TS と同じパターン)。

#### Fixture スタイル
PHP テストは通常クラスメソッドだが、helper tracing は free function のみ対象。fixture は `function test_*()` スタイル (Pest スタイル) で作成。

## Progress Log

### 2026-03-24 08:16 - INIT
- Cycle doc created
- Scope definition ready

### 2026-03-24 RED - COMPLETE
- Created `tests/fixtures/php/t001_pass_helper_tracing.php` (TC-01 ~ TC-07, free function style)
- Added 7 integration tests in `crates/lang-php/src/lib.rs` (`helper_tracing_tc01` ~ `tc07`)
- All 7 tests FAIL as expected (RED state verified)
- Failure cause: `test_function.scm` does not match free-function `function test_*()` style;
  functions are absent from `extract_file_analysis` result. GREENフェーズで解決:
  (1) test_function.scm に free-function パターン追加
  (2) `crates/lang-php/queries/helper_trace.scm` 新規作成
  (3) `lib.rs` に `apply_same_file_helper_tracing()` 呼び出し追加
- self-dogfooding: BLOCK 0 (unchanged)

---

## Next Steps

1. [Done] INIT <- Current
2. [Done] PLAN (plan approved)
3. [Next] RED
4. [ ] GREEN
5. [ ] REFACTOR
6. [ ] REVIEW
7. [ ] COMMIT
