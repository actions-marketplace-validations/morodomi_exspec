---
feature: rust-observe-l1-cross-dir-stem-matching
cycle: 20260325_1200
phase: RED
complexity: standard
test_count: 8
risk_level: medium
codex_session_id: ""
created: 2026-03-25 12:00
updated: 2026-03-25 12:00
---

# Rust observe: L1 cross-directory stem matching for integration test subdirs

## Scope Definition

### In Scope
- [ ] `apply_l1_5_underscore_path_matching()` にサブディレクトリマッチングロジックを追加
- [ ] テストファイルのサブディレクトリ (`tests/builder/action.rs` → subdir="builder") を production path の dir segment としてマッチ

### Out of Scope
- L2 (import tracing) の変更 (理由: 今回はL1.5拡張のみ)
- Python/TypeScript/PHP observe への影響 (理由: Rust固有の実装)

### Files to Change (target: 10 or less)
- `crates/lang-rust/src/observe.rs` (edit)

## Environment

### Scope
- Layer: Backend (Rust)
- Plugin: rust
- Risk: 35/100 (WARN)

### Runtime
- Language: Rust (stable)

### Dependencies (key packages)
- tree-sitter: existing
- tree-sitter-rust: existing

### Risk Interview (BLOCK only)
(N/A - WARN level)

## Context & Dependencies

### Reference Documents
- `docs/observe-ground-truth-rust-tokio.md` - tokio regression チェック用 ground truth
- `ROADMAP.md` - observe ship criteria (P >= 98%, R >= 90%)

### Dependent Features
- L1.5 underscore path matching: `crates/lang-rust/src/observe.rs` (`apply_l1_5_underscore_path_matching`)

### Related Issues/PRs
- clap R=20.9% 改善 (tests/builder/ 47 files の多くが未マッチ)

## Test List

### TODO
- [ ] SD-INT-01: Run observe on clap, verify tests/builder/*.rs recall improves significantly (target: R 20.9% → 50%+)
- [ ] SD-INT-02: Run observe on tokio, verify no regression

### WIP
(none)

### DONE
- [x] SD-01: Given test "tests/builder/action.rs" and prod "src/builder/action.rs", When subdir matching, Then match (subdir="builder", stem="action") -- RED FAIL (stub)
- [x] SD-02: Given test "tests/builder/command.rs" and prod "member/src/builder/command.rs", When subdir matching, Then match (cross-crate) -- RED FAIL (stub)
- [x] SD-03: Given test "tests/builder/action.rs" and prod "src/parser/action.rs" (wrong dir), When subdir matching, Then NO match -- RED PASS
- [x] SD-04: Given test "tests/action.rs" (no subdir, directly in tests/), When subdir matching, Then skip (no subdir) -- RED PASS
- [x] SD-05: Given test "tests/builder/main.rs", When subdir matching, Then skip (main.rs excluded by test_stem) -- RED PASS
- [x] SD-06: Given test already L1.5-matched, When subdir matching, Then skip; unmatched subdir test processed by L1.6 -- RED FAIL (stub)
- [ ] SD-INT-01: Run observe on clap, verify tests/builder/*.rs recall improves significantly (target: R 20.9% → 50%+)
- [ ] SD-INT-02: Run observe on tokio, verify no regression

### DISCOVERED
(none)

## Implementation Notes

### Goal
clap の observe Recall を改善する。現在 R=20.9% の主因は `tests/builder/` (47 files) が `clap_builder/src/builder/` にマッチしないこと。L1 matching を cross-directory stem matching に拡張することで解決する。

### Background
L1 `map_test_files()` は `(directory, stem)` ペアで完全一致マッチ。テスト `tests/builder/action.rs` は key=`("tests/builder", "action")`、プロダクション `clap_builder/src/builder/action.rs` は key=`("clap_builder/src/builder", "action")` となりディレクトリが異なるためマッチしない。

### Design Approach
既存の `apply_l1_5_underscore_path_matching()` の後に新ロジックを追加:

1. 未マッチのテストファイルについて、`tests/` 以降のサブディレクトリを抽出 (e.g., `tests/builder/action.rs` → subdir="builder")
2. subdir が空でなく、3文字以上の場合のみ処理
3. 全プロダクションファイルを走査し、stem が一致かつ prod path に `/{subdir}/` を含む場合マッチ
4. crate boundary guard は既存 L1.5 と同様

FP guard:
- test subdir は 3 文字以上 (単一文字ディレクトリを除外)
- prod path は subdir をディレクトリセグメントとして含む (substring マッチ不可)
- crate boundary guard (既存と同様)

## Verification

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
cargo run -- --lang rust .

# clap
cargo run -- observe --lang rust --format json /tmp/exspec-dogfood/clap > /tmp/clap-post-sd.json
# 期待: R が 20.9% → 50%+ に改善 (tests/builder/ 47 files の多くがマッチ)

# tokio regression
cargo run -- observe --lang rust --format json /tmp/exspec-dogfood/tokio > /tmp/tokio-post-sd.json
python3 scripts/evaluate_observe.py --observe-json /tmp/tokio-post-sd.json --ground-truth docs/observe-ground-truth-rust-tokio.md --scan-root /tmp/exspec-dogfood/tokio
```

Evidence: (orchestrate が自動記入)

## Progress Log

### 2026-03-25 12:00 - INIT
- Cycle doc created
- Scope definition ready

### 2026-03-25 - RED
- Added stub `apply_l1_subdir_matching()` to RustExtractor (no-op)
- Called from `map_test_files_with_imports()` after L1.5, before L2
- Added 6 unit tests (SD-01 to SD-06)
- RED state verified: SD-01, SD-02, SD-06 FAIL (stub); SD-03, SD-04, SD-05 PASS
- 187 existing tests all pass

---

## Next Steps

1. [Done] INIT <- Current
2. [Done] PLAN
3. [Next] RED
4. [ ] GREEN
5. [ ] REFACTOR
6. [ ] REVIEW
7. [ ] COMMIT
