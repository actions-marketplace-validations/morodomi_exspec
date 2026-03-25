---
feature: rust-observe-cross-crate-import-resolution
cycle: 20260325_1116
phase: RED
complexity: complex
test_count: 9
risk_level: medium
codex_session_id: ""
created: 2026-03-25 11:16
updated: 2026-03-25 11:16
---

# Rust observe: cross-crate import resolution for integration tests

## Scope Definition

### In Scope
- [ ] `extract_use_declaration()` を複数 crate_name を受け付けるよう拡張
- [ ] `apply_l2_imports()` を crate_name → member_root mapping 受け取るよう拡張
- [ ] `map_test_files_with_imports()`: root integration tests に全 member names を渡す cross-crate fallback 追加

### Out of Scope
- member-owned tests (clap_builder/tests/ 等) への cross-crate fallback 適用 (理由: 不要。per-member L2 で解決済み)
- L1 マッチングへの影響 (理由: L2 のみの変更)

### Files to Change (target: 10 or less)
- `crates/lang-rust/src/observe.rs` (edit)

## Environment

### Scope
- Layer: Backend
- Plugin: rust
- Risk: 45/100 (WARN) - L2 import tracing の core logic 変更。既存マッピングに影響する可能性

### Runtime
- Language: Rust

### Dependencies (key packages)
- tree-sitter: workspace member
- lang-rust: workspace crate

### Risk Interview (BLOCK only)
(N/A - WARN level)

## Context & Dependencies

### Reference Documents
- [docs/dogfooding-results.md] - Rust observe 現状の P/R 数値 (clap R=14.2%, tokio R=50.8%)
- [docs/languages/] - Rust observe 設計詳細
- [ROADMAP.md] - observe ship criteria (P>=98%, R>=90%)

### Dependent Features
- Rust observe L2 import tracing: `crates/lang-rust/src/observe.rs`
- Workspace member resolution: `map_test_files_with_imports()`

### Related Issues/PRs
(none)

## Test List

### TODO
- [ ] XC-01: Given `use clap::builder::Arg` with crate_names=["clap", "clap_builder"], When extract, Then returns ("clap", "builder", ["Arg"])
- [ ] XC-02: Given `use clap_builder::error::ErrorKind` with crate_names=["clap", "clap_builder"], When extract, Then returns ("clap_builder", "error", ["ErrorKind"])
- [ ] XC-03: Given `use std::collections::HashMap` with crate_names=["clap"], When extract, Then skipped (not in crate_names)
- [ ] XC-04: Given `use crate::utils` with crate_names=["clap"], When extract, Then returns ("crate", "utils", [])
- [ ] XC-05: Given root integration test `tests/builder/action.rs` with `use clap::Arg` and member clap at scan_root, When L2 with cross-crate resolution, Then test maps to production files in clap/src/
- [ ] XC-06: Given root integration test with `use clap_builder::builder::Arg` and member clap_builder, When L2, Then test maps to clap_builder/src/builder.rs
- [ ] XC-07: Given member-owned test (clap_builder/tests/), When cross-crate fallback, Then NOT applied (only root tests)
- [ ] XC-INT-01: Run observe on clap, verify R improves significantly from 14.2%
- [ ] XC-INT-02: Run observe on tokio, verify no regression (P=100% maintained)

### WIP
(none)

### DISCOVERED
(none)

### DONE
(none)

## Implementation Notes

### Goal
Rust の workspace 構成において、root crate の integration tests (`tests/`) が workspace member の実装にマッピングされるよう L2 import resolution を拡張する。clap の R=14.2% → 50%+ への改善を目標とする。

### Background
Rust integration tests は `use crate_name::` (external crate import) を使う。これはRust言語の構造的制約:
- `tests/` ディレクトリは external crate として扱われる
- `use crate::` は `src/` 内の unit test でのみ使用可能

Root crate (e.g., `clap`) の `src/lib.rs` は多くの場合 thin barrel で、`pub use clap_builder::*` のように workspace member に re-export する。L2 は `clap/src/` で解決しようとするが、実体は `clap_builder/src/` にある。

### Design Approach
`extract_use_declaration()` に複数の crate name を渡せるよう拡張する:

```rust
fn extract_import_specifiers_with_crate_names(
    source: &str,
    crate_names: &[&str],  // ["clap", "clap_builder", "clap_derive", ...]
) -> Vec<(String, String, Vec<String>)>  // (matched_crate_name, specifier, symbols)
```

各 `use X::module::Symbol` に対し:
1. X が crate_names のいずれかにマッチ → (X, "module", ["Symbol"]) を返す
2. `use crate::` → ("crate", "module", ["Symbol"]) (既存動作)
3. どれにもマッチしない → スキップ (std:: 等)

map_test_files_with_imports() での処理追加:
```
// Cross-crate fallback for root integration tests
root_integration_tests = test_sources not owned by any member
for member in workspace_members:
    apply_l2_imports(
        root_integration_tests,
        member.crate_name,      // e.g., "clap_builder"
        member.member_root,     // e.g., "clap/clap_builder"
        ...
    )
```

FP リスク: Root integration test が member A と member B の両方からimportする場合、両方にマッピングされる → これは正しい動作 (secondary targets)。`use std::` 等の外部crateはスキップ (crate_namesに含まれないため)。

## Verification

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
cargo run -- --lang rust .

# clap dogfooding
cargo run -- observe --lang rust --format json /tmp/exspec-dogfood/clap > /tmp/clap-post.json
# 期待: R が 14.2% → 50%+ に改善

# tokio regression check
cargo run -- observe --lang rust --format json /tmp/exspec-dogfood/tokio > /tmp/tokio-post.json
python3 scripts/evaluate_observe.py \
  --observe-json /tmp/tokio-post.json \
  --ground-truth docs/observe-ground-truth-rust-tokio.md \
  --scan-root /tmp/exspec-dogfood/tokio
# 期待: P=100% 維持
```

Evidence: (orchestrate が自動記入)

## Progress Log

### 2026-03-25 11:16 - INIT
- Cycle doc created
- Scope definition ready

---

## Next Steps

1. [Done] INIT <- Current
2. [Done] PLAN
3. [Next] RED
4. [ ] GREEN
5. [ ] REFACTOR
6. [ ] REVIEW
7. [ ] COMMIT
