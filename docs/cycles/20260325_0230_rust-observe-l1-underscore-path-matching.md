---
feature: rust-observe-l1-underscore-path-matching
cycle: 20260325_0230
phase: RED
complexity: standard
test_count: 10
risk_level: low
codex_session_id: ""
created: 2026-03-25 02:30
updated: 2026-03-25 02:30
---

# Rust observe: L1 underscore-to-path stem matching for recall improvement

## Scope Definition

### In Scope
- [ ] `map_test_files_with_imports()` に L1.5 マッチングロジックを追加
- [ ] underscore 区切りで prefix をディレクトリ、suffix を stem として比較する処理を実装
- [ ] 短い suffix (2文字以下) の FP ガードを実装
- [ ] ユニットテスト 8件の作成
- [ ] インテグレーションテスト 2件の作成

### Out of Scope
- L2 import tracing の変更 (今回は L1 のみ対象)
- 複数アンダースコアの再帰的マッチング (first split のみ)

### Files to Change (target: 10 or less)
- `crates/lang-rust/src/observe.rs` (edit)

## Environment

### Scope
- Layer: Backend (Rust)
- Plugin: rust
- Risk: 30/100 (WARN)

### Runtime
- Language: Rust (edition 2021)

### Dependencies (key packages)
- tree-sitter: workspace version
- tree-sitter-rust: workspace version

### Risk Interview (BLOCK only)
(N/A - WARN level)

## Context & Dependencies

### Reference Documents
- [CONSTITUTION.md] - Ship criteria: Precision >= 98%, Recall >= 90%
- [docs/STATUS.md] - P=100%, R=71.0%。Next P1: Rust GT re-audit
- [docs/observe-ground-truth-rust-tokio.md] - GT データ (評価基準)

### Dependent Features
- Rust observe L1 filename convention: `crates/lang-rust/src/observe.rs`
- fan-out filter: `crates/lang-rust/src/observe.rs`

### Related Issues/PRs
- Rust observe recall improvement (R=71.0% → 向上目標)

## Test List

### TODO
- [x] US-01: Given test "tests/sync_broadcast.rs" and prod "src/sync/broadcast.rs", When L1.5 matching, Then test maps to prod [RED: FAIL as expected]
- [x] US-02: Given test "tests/sync_oneshot.rs" and prod "src/sync/oneshot.rs", When L1.5 matching, Then test maps to prod [RED: FAIL as expected]
- [x] US-03: Given test "tests/task_blocking.rs" and prod "src/task/blocking.rs", When L1.5 matching, Then test maps to prod [RED: FAIL as expected]
- [x] US-04: Given test "tests/macros_select.rs" and prod "src/macros/select.rs", When L1.5 matching, Then test maps to prod [RED: FAIL as expected]
- [x] US-05: Given test "tests/abc.rs" (no underscore) and prod "src/abc.rs", When L1.5, Then falls through to normal L1 (no change) [RED: PASS as expected]
- [x] US-06: Given test "tests/sync_broadcast.rs" and prod "src/runtime/broadcast.rs" (wrong dir), When L1.5, Then NO match (prefix "sync" not in "runtime") [RED: PASS as expected]
- [x] US-07: Given test "tests/a_b.rs" (suffix "b" is 1 char), When L1.5, Then NO match (short suffix guard) [RED: PASS as expected]
- [x] US-08: Given test already matched by L1, When L1.5, Then skipped (no double-match) [RED: FAIL as expected]
- [ ] US-INT-01: Run observe on tokio, verify sync_broadcast.rs maps to sync/broadcast.rs
- [ ] US-INT-02: Run observe on tokio, verify FP count does not increase (evaluate against GT)

### WIP
(none)

### DISCOVERED
(none)

### DONE
(none)

## Implementation Notes

### Goal
Rust observe のリコール向上。現在 R=71.0% (ship criteria: R >= 90%)。FN の最大カテゴリである underscore→path 変換で4件解決を目指す。

### Background
Rust observe P=100% (GT), R=33.9% (当初) → R=71.0% (改善後)。FN root cause の最大カテゴリは barrel import (10件)。うち4件は L1 filename convention の underscore→path 変換で解決可能:
- `sync_broadcast.rs` → `sync/broadcast.rs` (stem "sync_broadcast" ≠ "broadcast")
- `sync_oneshot.rs` → `sync/oneshot.rs`
- `task_blocking.rs` → `task/blocking.rs`
- `macros_select.rs` → `macros/select.rs`

現在の L1 は `test_stem == production_stem` の完全一致のみ。

### Design Approach
`map_test_files_with_imports()` の L1 処理の後、L2 の前に L1.5 マッチングを挿入する。

アルゴリズム:
```
For each unmatched test file (not in layer1_matched):
  1. test_stem = rust_test_stem(test_path)  // e.g., "sync_broadcast"
  2. If test_stem contains '_':
     a. Split on first '_': prefix="sync", suffix="broadcast"
     b. For each production file:
        - If prod_stem == suffix AND prod_path contains "/{prefix}/"
        - → Add test to this production file's mapping (strategy: FileNameConvention)
        - → Add to layer1_matched
```

name-match の条件:
- `prod_stem.to_lowercase() == suffix.to_lowercase()` (完全一致)
- `prod_path` にディレクトリセグメント `prefix` が含まれる (e.g., `/sync/` in `src/sync/broadcast.rs`)

FP リスク軽減:
- suffix が 2文字以下の場合はスキップ
- prefix がプロダクションファイルのパスに含まれない場合はスキップ

## Progress Log

### 2026-03-25 02:30 - INIT
- Cycle doc created
- Scope definition ready

### 2026-03-25 - RED
- `apply_l1_5_underscore_path_matching` stub added to `crates/lang-rust/src/observe.rs` (no-op)
- Stub called from `map_test_files_with_imports` after L1, before L2
- 8 unit tests added (US-01 to US-08)
- RED state verified: 5 FAIL (US-01,02,03,04,08), 3 PASS (US-05,06,07)
- 172 existing tests passing, 0 regressions

---

## Next Steps

1. [Done] INIT <- Current
2. [Done] PLAN
3. [Next] RED
4. [ ] GREEN
5. [ ] REFACTOR
6. [ ] REVIEW
7. [ ] COMMIT
