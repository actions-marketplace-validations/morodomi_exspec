---
feature: python-same-file-helper-tracing
cycle: 20260323_2357
phase: REVIEW
complexity: standard
test_count: 7
risk_level: low
codex_session_id: ""
created: 2026-03-23 23:57
updated: 2026-03-24 00:00
---

# Phase 23b: Same-file helper tracing (Python)

## Scope Definition

### In Scope
- [ ] `crates/lang-python/queries/helper_trace.scm` — 新規 tree-sitter クエリ（call_name / def_name / def_body）
- [ ] `crates/lang-python/src/lib.rs` — OnceLock cache + `apply_same_file_helper_tracing()` 呼び出し追加
- [ ] `tests/fixtures/python/t001_pass_helper_tracing.py` — 新規 fixture（TC-01 〜 TC-07）
- [ ] 統合テスト追加（lang-python lib.rs の `#[cfg(test)]` 内）

### Out of Scope
- TypeScript / PHP の helper_trace.scm / extractor 修正 (Phase 23c-d)
- 別ファイル（parent class 等）のヘルパー追跡 (Phase 24 以降、#153)
- 2-hop 以上の追跡
- `self.helper()` メソッド呼び出し（`call function: (attribute)` は対象外、cross-file #153 で対応）
- 動的ディスパッチ

### Files to Change (target: 10 or less)
- `crates/lang-python/queries/helper_trace.scm` (new)
- `crates/lang-python/src/lib.rs` (edit)
- `tests/fixtures/python/t001_pass_helper_tracing.py` (new)

## Environment

### Scope
- Layer: Backend (Rust static analysis)
- Plugin: dev-crew:python-quality (Rust crate)
- Risk: LOW

### Runtime
- Language: Rust (stable, workspace edition 2021)

### Dependencies (key packages)
- tree-sitter: workspace version
- tree-sitter-python: workspace version
- exspec-core: workspace member (`apply_same_file_helper_tracing`)

### Risk Interview (BLOCK only)
N/A — LOW判定

## Context & Dependencies

### Reference Documents
- `crates/lang-rust/queries/helper_trace.scm` — Rust版クエリ（設計テンプレート）
- `crates/lang-rust/src/lib.rs` — OnceLock cache + helper tracing 呼び出しの実装パターン
- `crates/core/src/query_utils.rs` — `apply_same_file_helper_tracing()` 関数（言語非依存）
- `crates/lang-python/queries/assertion.scm` — assertion detection クエリ（helper body への適用に使用）
- `tests/fixtures/rust/t001_pass_helper_tracing.rs` — TC設計のテンプレート

### Dependent Features
- Phase 23a (Rust + core): `apply_same_file_helper_tracing()` 実装済み。core 関数はそのまま使用

### Related Issues/PRs
- GitHub Issue #150: Same-file helper tracing: Python
- ROADMAP.md v0.4.3: #150 Same-file helper tracing: Python
- Phase 23a (Rust): 同一アプローチの検証済み実装

## Test List

### TODO
(none — all moved to DONE)

### DONE
- [x] TC-01: assertion ありヘルパー関数を呼ぶテスト → assertion_count が 1 以上になること（正常系）
- [x] TC-02: assertion なしヘルパー関数を呼ぶテスト → assertion_count が増加しないこと（FN防止）
- [x] TC-03: assertion_count > 0 のテスト関数 → helper tracing でさらに加算されないこと（早期リターン確認）
- [x] TC-04: ヘルパーが存在しない（定義なし）の呼び出し → クラッシュせず 0 を返すこと（防御的テスト）
- [x] TC-05: 2-hop ヘルパー（テスト → 中間 → check_result）→ 1-hop のみ追跡し、2-hop 先は未検出であること（境界値）
- [x] TC-06: assertion_count == 0 の関数がない場合 → early return で追加クエリ実行コストゼロ（パフォーマンス保護の確認）
- [x] TC-07: 同じヘルパーを複数回呼ぶ → dedup により assertion_count が重複加算されないこと

### WIP
(none)

### DISCOVERED
- D-01: `self.method()` (attribute call) は helper_trace.scm の `(identifier)` パターンにマッチしない → unittest.TestCase内メソッドヘルパーは未トレース。Out of Scopeとして明記済み、cross-file #153で対応予定
- D-02: 同名の module-level 関数とクラスメソッドが共存する場合、HashMap後勝ちで誤カウントのリスクあり。実害は低い (テストファイルで同名関数が複数存在するケースは稀)

### DONE (original)

## Implementation Notes

### Goal

Python テスト関数が assertion をヘルパー関数に委譲するパターン（T001 BLOCK FP の最大原因: django 32件, requests 10件）を、同一ファイル内 1-hop 追跡により自動検出する。`custom_patterns` に頼らず、AST ベースで正確に assertion を計上する。

### Background

Helper delegation は全言語で T001 BLOCK FP の最大原因。Phase 23a で Rust + core に実装済みの `apply_same_file_helper_tracing()` は言語非依存のため、Python 固有の tree-sitter クエリと統合コードのみ追加すればよい。

### Design Approach

**helper_trace.scm (Python)**:
```scm
; Function calls (free function — helper call in test body)
(call function: (identifier) @call_name)

; Function definitions (helper function with body)
(function_definition
  name: (identifier) @def_name
  body: (block) @def_body)
```

Rust版と同じ2パターン。Python の `call` ノードと `function_definition` ノードを使用。

**lib.rs 変更点**:
1. `use exspec_core::query_utils::apply_same_file_helper_tracing;` 追加
2. `const HELPER_TRACE_QUERY: &str = include_str!("../queries/helper_trace.scm");`
3. `static HELPER_TRACE_QUERY_CACHE: OnceLock<Query> = OnceLock::new();`
4. `extract_file_analysis()` 内 FileAnalysis構築後・return前に呼び出し追加

**メソッド呼び出し制限**:
`self.helper()` は `call function: (attribute)` であり `(identifier)` にマッチしない → Phase 23a と同じスコープ制限。cross-file (#153) で対応予定。

## Progress Log

### 2026-03-23 23:57 - INIT
- Cycle doc created from plan file (nifty-jumping-kurzweil.md)
- Scope: Python port of Phase 23a same-file helper tracing

### 2026-03-24 - RED
- Created `tests/fixtures/python/t001_pass_helper_tracing.py` (TC-01 ~ TC-07)
- Added 7 integration tests in `crates/lang-python/src/lib.rs` (#[cfg(test)] section)
- Verified RED state: TC-01 and TC-07 FAIL (assertion_count == 0, expected >= 1 / == 1)
- TC-02 ~ TC-06 trivially pass (expect 0 or own assert already counted)
- Self-dogfooding: BLOCK 0 (Rust), fixture BLOCK 0 (Python)

### 2026-03-24 - GREEN
- Created `crates/lang-python/queries/helper_trace.scm` (2 patterns: call + def)
- Modified `crates/lang-python/src/lib.rs`: import, const, OnceLock cache, apply_same_file_helper_tracing() call
- All 250 Python tests pass (248 existing + 2 new TC-01/TC-07 now GREEN)
- Verification: cargo test OK, clippy 0, fmt OK, self-dogfooding BLOCK 0

### 2026-03-24 - REFACTOR
- Removed stale RED phase comments from test code
- Checklist: no duplicates, no magic numbers, no unused imports, naming consistent
- Verification Gate PASS (1126 tests, clippy 0, fmt OK)
- Phase completed

### 2026-03-24 - REVIEW
- Security reviewer: PASS (45). IMPORTANT 1: call_query/def_query同一オブジェクト → コメント追記で対応
- Correctness reviewer: PASS (35). IMPORTANT 2: self.method()非トレース (Out of Scope明記済み), 同名関数後勝ち (DISCOVERED記録)
- Aggregate: PASS (40)
- DISCOVERED: D-01 (self.method未対応), D-02 (同名関数HashMap後勝ち)
- Phase completed

---

## Next Steps

1. [Done] INIT
2. [ ] PLAN
3. [ ] RED
4. [ ] GREEN
5. [ ] REFACTOR
6. [ ] REVIEW
7. [ ] COMMIT
