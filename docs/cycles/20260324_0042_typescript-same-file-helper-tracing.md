---
feature: typescript-same-file-helper-tracing
cycle: 20260324_0042
phase: DONE
complexity: standard
test_count: 8
risk_level: low
codex_session_id: ""
created: 2026-03-24 00:42
updated: 2026-03-24 01:00
---

# #151 Same-file helper tracing for TypeScript (Phase 23b port)

## Scope Definition

### In Scope
- [ ] `crates/lang-typescript/queries/helper_trace.scm` 新規作成
- [ ] `crates/lang-typescript/src/lib.rs` に OnceLock cache + `apply_same_file_helper_tracing()` 呼び出し追加
- [ ] `tests/fixtures/typescript/t001_pass_helper_tracing.test.ts` 新規作成 (TC-01 ~ TC-08)
- [ ] 統合テスト追加 (lang-typescript lib.rs の #[cfg(test)] 内)

### Out of Scope
- `this.helper()` メソッド呼び出しトレース (Reason: `member_expression` は `(identifier)` にマッチしない。Python と同じスコープ制限。#153 で対応)
- arrow function の expression body (`() => expr`) (Reason: `statement_block` にマッチしない。helper としては稀、許容)
- 2-hop 以上のトレース (Reason: 1-hop only の設計制約)

### Files to Change (target: 10 or less)
- `crates/lang-typescript/queries/helper_trace.scm` (new)
- `crates/lang-typescript/src/lib.rs` (edit)
- `tests/fixtures/typescript/t001_pass_helper_tracing.test.ts` (new)

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
- `crates/lang-python/queries/helper_trace.scm` — Python版テンプレート
- `crates/lang-python/src/lib.rs` — 統合パターンのテンプレート
- `tests/fixtures/python/t001_pass_helper_tracing.py` — TC設計のテンプレート
- `ROADMAP.md` v0.4.3: #151 Same-file helper tracing: TypeScript

### Dependent Features
- Phase 23a (Rust): `crates/core/src/query_utils.rs` — `apply_same_file_helper_tracing()` 提供元
- #150 (Python): 同一パターンの確立済み実装

### Related Issues/PRs
- Issue #151: Same-file helper tracing: TypeScript
- #150 (Python port): 確立済み
- #153 (メソッド呼び出し): 将来対応

## Test List

### TODO
(none)

### WIP
(none)

### DONE (RED)
- [x] TC-01: helper with expect → test calls helper → assertion_count >= 1 — FAIL (RED confirmed)
- [x] TC-02: helper without assertion → test calls helper → assertion_count == 0 — PASS (trivial)
- [x] TC-03: test has own expect + calls helper → assertion_count >= 1 (no extra tracing) — PASS (trivial)
- [x] TC-04: test calls undefined function → assertion_count == 0 (no crash) — PASS (trivial)
- [x] TC-05: 2-hop: test → intermediate → checkResult → assertion_count == 0 (1-hop only) — PASS (trivial)
- [x] TC-06: test with own assertion → early return → assertion_count unchanged — PASS (trivial)
- [x] TC-07: multiple calls to same helper → dedup → assertion_count == 1 — FAIL (RED confirmed)
- [x] TC-08: arrow function helper with expect → assertion_count >= 1 — FAIL (RED confirmed)

### WIP
(none)

### DISCOVERED
(none)

### DONE
(none)

## Implementation Notes

### Goal
Phase 23a (Rust) → #150 (Python) と同じ same-file helper tracing を TypeScript にポートする。nestjs dogfooding で 13 BLOCK のうち 8 が helper delegation によるもので、これを解消する。

### Background
`core` の `apply_same_file_helper_tracing()` は言語非依存。Python ポートで確立したパターンをそのまま TypeScript に適用する。TypeScript では `function_declaration` + arrow function in `variable_declarator` の2パターンが必要。

### Design Approach

#### `helper_trace.scm` (新規)
```scm
; Function calls (free function — helper call in test body)
(call_expression function: (identifier) @call_name)

; Function declarations
(function_declaration
  name: (identifier) @def_name
  body: (statement_block) @def_body)

; Arrow function helpers assigned to const/let/var
; e.g. const assertValid = (x) => { expect(x)... }
(lexical_declaration
  (variable_declarator
    name: (identifier) @def_name
    value: (arrow_function
      body: (statement_block) @def_body)))
```

#### `lib.rs` 変更 (3点)
1. import: `use exspec_core::query_utils::apply_same_file_helper_tracing;`
2. 定数 + OnceLock: `HELPER_TRACE_QUERY` / `HELPER_TRACE_QUERY_CACHE`
3. `extract_file_analysis()` にて FileAnalysis 構築後・return 前に `apply_same_file_helper_tracing()` 呼び出し

## Progress Log

### 2026-03-24 00:42 - INIT
- Cycle doc created
- Scope definition ready

### 2026-03-24 01:00 - RED
- Fixture created: tests/fixtures/typescript/t001_pass_helper_tracing.test.ts (TC-01 ~ TC-08)
- Integration tests added: crates/lang-typescript/src/lib.rs (8 tests)
- RED state verified: TC-01, TC-07, TC-08 FAIL (assertion_count 0, helper tracing not yet implemented)
- TC-02~06 pass trivially (no tracing needed for zero/own-assertion cases)
- Self-dogfooding: BLOCK 0

---

## Next Steps

1. [Done] INIT
2. [Done] PLAN (plan approved)
3. [Done] RED
4. [Next] GREEN
5. [ ] REFACTOR
6. [ ] REVIEW
7. [ ] COMMIT
