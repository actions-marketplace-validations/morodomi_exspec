---
feature: "B1 — TypeScript observe: namespace re-export support"
cycle: "20260318_0844"
phase: DONE
complexity: trivial
test_count: 4
risk_level: low
codex_session_id: ""
created: 2026-03-18 08:44
updated: 2026-03-18 08:44
---

# Cycle: B1 — TypeScript observe: namespace re-export support

## Scope Definition

### In Scope
- [ ] `crates/lang-typescript/queries/re_export.scm` — namespace_export パターン追加
- [ ] `crates/lang-typescript/src/observe.rs` — `extract_barrel_re_exports_impl` で `ns_wildcard` capture を処理
- [ ] 境界テスト期待値の反転

### Out of Scope
- 他言語 observe への影響
- named re-export の挙動変更
- TypeScript 以外の barrel resolution 変更
- Ns.Foo -> Foo の namespace 解決 (opaque wildcard として扱う)

### Files to Change (target: 10 or less)
- `crates/lang-typescript/queries/re_export.scm` (edit)
- `crates/lang-typescript/src/observe.rs` (edit)

## Environment

### Scope
- Layer: Backend (`crates/lang-typescript` のみ)
- Plugin: dev-crew:rust-quality (cargo test / clippy / fmt)
- Risk: 20/100 (PASS)

### Runtime
- Language: Rust (cargo test)

### Dependencies (key packages)
- tree-sitter: 既存
- tree-sitter-typescript: 既存

### Risk Interview (BLOCK only)

(BLOCK なし — リスク 20/100)

## Context & Dependencies

### Reference Documents
- `crates/lang-typescript/queries/re_export.scm` — 既存 re-export クエリ
- `crates/lang-typescript/src/observe.rs` L764-834 — `extract_barrel_re_exports_impl` 実装
- `crates/lang-typescript/src/observe.rs` L2934-3018 — 境界テスト

### Dependent Features
- TypeScript observe barrel resolution: `crates/lang-typescript/src/observe.rs`
- Wildcard re-export processing: 既存 wildcard パスを再利用

### Related Issues/PRs
- (none)

## Test List

### TODO
(none)

### WIP
(none)

### DISCOVERED
(none)

### DONE
- [x] TS-B1-NS-01: namespace re-export が wildcard として抽出される
- [x] TS-B1-NS-02: namespace re-export 経由の mapping が TP になる
- [x] TS-B1-NS-03: 通常の wildcard と namespace re-export の混在 (namespace_wildcard フラグ差異も検証)
- [x] TS-B1-NS-04: named re-export との混在
- [x] TC-01: boundary_b1_ns_reexport_captured_as_wildcard (反転)
- [x] TC-02: boundary_b1_ns_reexport_mapping_miss (反転)

## Implementation Notes

### Goal
TypeScript observe の Recall 93.4% (NestJS) をさらに改善する。`export * as Ns from './module'` が `re_export.scm` に未対応で、barrel resolution で見落とされる boundary (B1) を修正する。

### Background
NestJS eval では B1 単体の FN 件数は少ないが、namespace re-export パターンは utility package で頻出するため汎用性が高い。

`export * as Validators from './validators'` のAST:

```
(export_statement
  (namespace_export
    (identifier))          ; "Validators"
  source: (string
    (string_fragment)))    ; "./validators"
```

### Design Approach
`export * as Ns from './module'` を `wildcard: true` の BarrelReExport として扱う。

**Step 1**: `re_export.scm` に namespace_export パターンを追加:

```scheme
;; Namespace re-export: export * as Ns from './module'
(export_statement
  (namespace_export) @ns_wildcard
  source: (string
    (string_fragment) @from_specifier))
```

**Step 2**: `observe.rs` の `extract_barrel_re_exports_impl` に `ns_wildcard` capture index を追加。既存の `wildcard_idx` チェック (L801) に `|| ns_wildcard_idx == Some(cap.index)` で合算する。barrel resolution は既存コードが `wildcard=true` の BarrelReExport を透過的に処理するため変更不要。

**設計判断**: namespace は opaque wildcard として扱う (Ns.Foo -> Foo の静的解決は行わない)。wildcard=true により `./module` 内の全 public シンボルが解決対象になり FN を回避する。FP リスクは低い (barrel 経由の precision 99.4%)。

**Step 3**: 境界テスト反転 (L2934-3018)。

## Progress Log

### 2026-03-18 08:44 - INIT
- Cycle doc created
- Scope definition ready

### 2026-03-18 - RED
- 境界テスト2件反転 (boundary_b1_ns_reexport_captured_as_wildcard, boundary_b1_ns_reexport_mapping_miss)
- 新規テスト4件追加 (TS-B1-NS-01〜04)
- 6テスト全て失敗確認 (RED state)

### 2026-03-18 - GREEN
- re_export.scm に namespace_export パターン追加
- observe.rs に ns_wildcard capture 処理追加
- core BarrelReExport に namespace_wildcard フィールド追加
- barrel resolution で namespace_wildcard 時に symbols を空にして再帰
- 946テスト全通過

### 2026-03-18 - REVIEW
- correctness-reviewer: score 42 (PASS)
- 指摘3件修正: テスト名改名、namespace_wildcard assert追加、非バレルパスのコメント追加
- self-dogfooding: BLOCK 0 | WARN 0 | INFO 7 | PASS 9
- Phase completed

### 2026-03-18 - COMMIT
- PR #102 作成・マージ (b110511)
- feat: support namespace re-export in TypeScript observe (#85)
- Phase completed

---

## Next Steps

1. [Done] INIT
2. [Done] PLAN
3. [Done] RED
4. [Done] GREEN
5. [Done] REFACTOR (不要と判断)
6. [Done] REVIEW
7. [Done] COMMIT
