---
feature: rust-same-file-helper-tracing
cycle: 20260323_0922
phase: DONE
complexity: standard
test_count: 6
risk_level: low
codex_session_id: ""
created: 2026-03-23 09:22
updated: 2026-03-23 09:22
---

# Phase 23a: Same-file helper delegation (Rust + core)

## Scope Definition

### In Scope
- [ ] `crates/lang-rust/queries/helper_trace.scm` — 新規 tree-sitter クエリ（call_name / def_name / def_body）
- [ ] `crates/core/src/query_utils.rs` — `apply_same_file_helper_tracing` 関数追加
- [ ] `crates/lang-rust/src/lib.rs` — `extract_file_analysis` 末尾で helper tracing 呼び出し + 統合テスト
- [ ] `tests/fixtures/rust/t001_pass_helper_tracing.rs` — 新規 fixture

### Out of Scope
- Python / TypeScript / PHP の helper_trace.scm / extractor 修正 (Phase 23b-d)
- 別ファイル（parent class 等）のヘルパー追跡 (Phase 24 以降)
- 2-hop 以上の追跡
- 動的ディスパッチ（trait object 等）

### Files to Change (target: 10 or less)
- `crates/lang-rust/queries/helper_trace.scm` (new)
- `crates/core/src/query_utils.rs` (edit)
- `crates/lang-rust/src/lib.rs` (edit)
- `tests/fixtures/rust/t001_pass_helper_tracing.rs` (new)

## Environment

### Scope
- Layer: Backend (Rust static analysis)
- Plugin: dev-crew:python-quality (Rust crate)
- Risk: 15 (PASS)

### Runtime
- Language: Rust (stable, workspace edition 2021)

### Dependencies (key packages)
- tree-sitter: workspace version
- tree-sitter-rust: workspace version
- streaming-iterator: workspace version
- exspec-core: workspace member

### Risk Interview (BLOCK only)
N/A — PASS判定

## Context & Dependencies

### Reference Documents
- `crates/core/src/query_utils.rs` — `apply_custom_assertion_fallback` が設計パターンの参照元
- `crates/lang-rust/queries/assertion.scm` — assertion detection クエリ（helper body への適用に使用）
- `tests/fixtures/rust/t001_pass_helper_delegation.rs` — 既存 helper delegation fixture（assert_* 関数名マッチ）

### Dependent Features
- Phase 22 (Rust assertion.scm prefix match): assertion.scm がヘルパー body 内の `assert!` / `assert_eq!` 等を正しく検出できることが前提

### Related Issues/PRs
- Phase 23 全体計画: plan file `/Users/morodomi/.claude/plans/kind-inventing-crab.md`

## Test List

### TODO
- [ ] TC-01: assertion ありヘルパー関数を呼ぶテスト → assertion_count が 1 以上になること（正常系）
- [ ] TC-02: assertion なしヘルパー関数を呼ぶテスト → assertion_count が増加しないこと（FN防止）
- [ ] TC-03: assertion_count > 0 のテスト関数 → helper tracing でさらに加算されないこと（早期リターン確認）
- [ ] TC-04: ヘルパーが存在しない（定義なし）の呼び出し → クラッシュせず 0 を返すこと（防御的テスト）
- [ ] TC-05: 2-hop ヘルパー（ヘルパーがさらに別ヘルパーを呼ぶ）→ 1-hop のみ追跡し、2-hop 先は未検出であること（境界値）
- [ ] TC-06: assertion_count == 0 の関数がない場合 → early return で追加クエリ実行コストゼロ（パフォーマンス保護の確認）

### WIP
(none)

### DISCOVERED
(none)

### DONE
(none)

## Implementation Notes

### Goal

テスト関数が assertion をヘルパー関数に委譲するパターン（T001 BLOCK FP の最大原因）を、同一ファイル内 1-hop 追跡により自動検出する。`custom_patterns` に頼らず、AST ベースで正確に assertion を計上する。

### Background

Helper delegation は全言語で T001 BLOCK FP の最大原因。テスト関数がアサーションを含むヘルパー関数を呼ぶが、exspec はヘルパー body 内の assertion を見ない。現在の対策は `custom_patterns`（テキストベースの手動設定）のみ。

Phase 23 では**同一ファイル内**のヘルパー関数を 1-hop 追跡し、assertion_count に自動加算する。

### Design Approach

**アルゴリズム**:
1. `def_query` でファイル全体から関数定義を収集 → `HashMap<String, Node>` (name → body node)
2. assertion_count == 0 の関数のみ処理（early return でパフォーマンス保護）
3. テスト関数 node 上で `call_query` を実行 → 呼び出し名リスト
4. 各呼び出し名が定義マップにあれば、その body node 上で `count_captures(assertion_query, "assertion", ...)` を実行
5. assertion_count に加算

**helper_trace.scm (Rust)**:
```scm
; Function calls (free function)
(call_expression function: (identifier) @call_name)

; Function definitions
(function_item name: (identifier) @def_name body: (block) @def_body)
```

**関数シグネチャ**:
```rust
pub fn apply_same_file_helper_tracing(
    analysis: &mut FileAnalysis,
    tree: &Tree,
    source: &[u8],
    call_query: &Query,       // helper_trace.scm (call patterns)
    def_query: &Query,        // helper_trace.scm (def patterns)
    assertion_query: &Query,  // assertion.scm
)
```

**既存パターンとの整合性**:
- `apply_custom_assertion_fallback` と同じく assertion_count == 0 の関数のみを対象とする
- `OnceLock` キャッシュパターンで `HELPER_TRACE_QUERY_CACHE` を追加（既存パターンに従う）
- `extract_file_analysis()` 末尾で呼び出し（`apply_custom_assertion_fallback` の前に実行）

**always-on**: FP 削減のみ（BLOCK→PASS 方向）なので設定不要。

## Progress Log

### 2026-03-23 09:22 - INIT
- Cycle doc created from plan file (kind-inventing-crab.md)
- Design Review Gate: PASS (score: 15/100)
- Scope definition ready (Phase 23a: Rust + core only)

### 2026-03-23 09:30 - RED
- Fixture: `tests/fixtures/rust/t001_pass_helper_tracing.rs` (5 test functions + 3 helpers)
- TC-01 FAIL (helper tracing stub), TC-02〜06 PASS. RED confirmed

### 2026-03-23 09:40 - GREEN
- `helper_trace.scm`: call_name / def_name / def_body captures
- `apply_same_file_helper_tracing`: HashMap-based definition lookup + byte-range scoped call extraction
- `RustExtractor::extract_file_analysis`: OnceLock cached query + helper tracing call
- All 1098 tests passed, clippy 0, self-dogfooding BLOCK 0

### 2026-03-23 09:45 - REFACTOR
- Checklist 7/7 reviewed, no changes needed. Verification Gate PASS. Phase completed

---

## Next Steps

1. [Done] INIT
2. [Done] PLAN
3. [Done] RED
4. [Done] GREEN
5. [Done] REFACTOR
6. [Done] REVIEW — PASS (correctness: dedup bug fix + TC-07, security: PASS)
7. [Done] COMMIT
