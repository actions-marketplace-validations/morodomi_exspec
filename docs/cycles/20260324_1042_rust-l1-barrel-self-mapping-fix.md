---
feature: "#161 Rust L1 safeguards: mod.rs exclusion + test file existence check"
cycle: 20260324_1042
phase: COMMIT
complexity: standard
test_count: 4
risk_level: low
codex_session_id: ""
created: 2026-03-24 10:42
updated: 2026-03-24
---

# #161 Rust L1 safeguards: mod.rs exclusion + test file existence check

## Summary

GT audit (#149) で Rust observe P=76.7% (7/30 FP)。Issue #161 は L1 filename matching の FP 排除を目的とするが、コード調査の結果、Issue の記述と実装に矛盾がある。実際の FP は L0 (inline test detection) の self-mapping から発生している。mod.rs/lib.rs/main.rs 等の barrel ファイルが `#[cfg(test)]` を持つ場合に自身をマッピングするのを抑制する。

## Scope Definition

### In Scope

- `crates/lang-rust/src/observe.rs`: L0 self-mapping に `production_stem()` チェック追加 (L802-812)
- 新規テスト: mod.rs/lib.rs/main.rs の self-mapping が除外されることを検証
- Issue #161 へのコメント: 根本原因の修正報告

### Out of Scope

- L2 incidental import FP (Issue #163 で別途対応)
- tokio 50-pair re-audit (Issue #163)
- テストファイル存在チェック (`map_test_files()` は構造的に不要と確認)

### Files to Change

| File | Change |
|------|--------|
| `crates/lang-rust/src/observe.rs` | L0 self-mapping に barrel filter 追加 (L802-812 付近) |
| `crates/lang-rust/src/observe.rs` (tests) | 新規テスト追加 |

## Environment

- Layer: lang-rust/observe
- Plugin: Rust
- Risk: low
- Runtime: Rust (cargo test)
- Dependencies: tree-sitter, lang-rust crate

## Context & Dependencies

### Root Cause Analysis

| Issue の主張 | 実コード |
|-------------|---------|
| `production_stem()` が "mod" を返す | `stem == "mod"` → `None` (既に除外済み, lang-rust/observe.rs:82) |
| L1 がテスト不在ファイルをマッチ | `map_test_files()` は test_files リスト内のみマッチ (core/observe.rs:103-117) |

実際の FP は `map_test_files_with_imports()` の L0 セクション (lang-rust/observe.rs:802-812) が、`detect_inline_tests()` で `#[cfg(test)]` を検出した全 production file を self-map することで発生。mod.rs/lib.rs でも `#[cfg(test)]` があれば self-mapping される。

### FP Examples (tokio dogfooding)

```
tokio-util/src/sync/mod.rs → 自身  (strategy: filename)
tokio/src/fs/mod.rs → 自身
tokio/src/runtime/mod.rs → 自身
tokio/src/runtime/task/mod.rs → 自身
tokio/src/runtime/time/mod.rs → 自身
tokio/src/runtime/time_alt/mod.rs → 自身
tokio/src/sync/mod.rs → 自身
tokio/src/lib.rs → 自身
```

### References

- CONSTITUTION.md Section 7: "exspec errs on the side of being quiet"
- ROADMAP.md: Phase 1 (L1 safeguards) の目的と一致
- observe-gt-guideline.md: barrel file (mod.rs) は non_target

## Implementation Notes

### Goal

tokio の mod.rs/lib.rs self-mapping FP (~6-8件) を排除。30-pair サンプルの L1 FP (5件中 4-5件) 解消。P 76.7% → ~87-93% (推定)。

### Background

GT audit (#149) で Rust observe の Precision が 76.7% (7/30 FP)。Issue #161 が作成されたが、Issue の Proposed Solution は実コードと矛盾していた。コード調査により真の FP メカニズムが L0 self-mapping にあることが判明。

### Design Approach

`map_test_files_with_imports()` の L0 セクションで、`production_stem()` が `None` を返すファイル (mod.rs, lib.rs, main.rs, build.rs) の self-mapping をスキップ。

```rust
// Layer 0: Inline test self-mapping
for (idx, prod_file) in production_files.iter().enumerate() {
    // Skip barrel/entry point files (mod.rs, lib.rs, main.rs, build.rs)
    if self.production_stem(prod_file).is_none() {
        continue;
    }
    if let Ok(source) = std::fs::read_to_string(prod_file) {
        if detect_inline_tests(&source) {
            if !mappings[idx].test_files.contains(prod_file) {
                mappings[idx].test_files.push(prod_file.clone());
            }
        }
    }
}
```

**Note**: 一部の mod.rs は barrel ではなく実ロジック + inline tests を持つ可能性があるが (e.g., `tokio/src/runtime/time_alt/mod.rs`)、CONSTITUTION の quiet 原則 (FP を避ける方向にエラー) に沿い、mod.rs self-mapping は除外する。

## Test List

### TODO

(none)

### WIP

(none)

### DONE

- [x] TC-01: **Given** mod.rs with `#[cfg(test)]` in production_files, **When** map_test_files_with_imports, **Then** mod.rs は self-mapping されない → `rs_l0_barrel_01_mod_rs_excluded` (RED: FAIL)
- [x] TC-02: **Given** lib.rs with `#[cfg(test)]` in production_files, **When** map_test_files_with_imports, **Then** lib.rs は self-mapping されない → `rs_l0_barrel_02_lib_rs_excluded` (RED: FAIL)
- [x] TC-03: **Given** regular .rs file with `#[cfg(test)]`, **When** map_test_files_with_imports, **Then** self-mapping される (regression) → `rs_l0_barrel_03_regular_file_self_mapped` (RED: PASS as expected)
- [x] TC-04: **Given** main.rs with `#[cfg(test)]`, **When** map_test_files_with_imports, **Then** main.rs は self-mapping されない → `rs_l0_barrel_04_main_rs_excluded` (RED: FAIL)

### DISCOVERED

(none)

## Progress Log

- 2026-03-24 10:42: Cycle doc 作成 (sync-plan)
- 2026-03-24 10:50: REVIEW (plan) WARN score:35. FN risk for logic-carrying mod.rs noted (runtime/task/mod.rs 31fn, runtime/time/mod.rs 11fn). CONSTITUTION quiet原則で FP 優先、FN は #163 re-audit で定量評価。TC-04 main.rs は production_files に含まれることを確認。Phase completed.
- 2026-03-24: RED phase completed. 4 tests added to crates/lang-rust/src/observe.rs. TC-01/TC-02/TC-04 FAIL (expected RED), TC-03 PASS (regression). clippy clean.
- 2026-03-24: GREEN phase completed. L0 self-mapping に production_stem() チェック追加 (2行)。全4テスト PASS。
- 2026-03-24: REFACTOR phase completed. チェックリスト走査、改善不要。Verification Gate PASS (1146 tests, clippy 0, fmt clean, self-dogfooding BLOCK 0). Phase completed.
- 2026-03-24: REVIEW (code) PASS score:8. Security PASS(5), Correctness PASS(8), Lint PASS(0). No blocking issues. Phase completed.
