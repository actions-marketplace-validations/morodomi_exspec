---
feature: "#162 Rust L2 re-export chain validation + L0 detect_inline_tests improvement"
cycle: 20260324_1119
phase: COMMIT
complexity: standard
test_count: 6
risk_level: low
codex_session_id: ""
created: 2026-03-24 11:19
updated: 2026-03-24 11:19
---

# #162 Rust L2 re-export chain validation + L0 detect_inline_tests improvement

## Summary

Issue #163 の中間 re-audit で P=92.0% (46/50)。残存 FP 4件を排除し P=100% (50/50) を目指す。FP の内訳は L0 cfg(test) false detection 2件 (source.rs, open_options.rs — `#[cfg(test)]` がテストモジュール以外の用途) と L2 re-export chain confusion 2件 (driver.rs ← shutdown.rs/yield_now.rs — `pub(crate) mod` 経由の over-mapping)。

## Scope Definition

### In Scope

- `crates/lang-rust/src/observe.rs:135-170`: `detect_inline_tests()` に `mod_item` 兄弟ノード検証追加
- `crates/lang-rust/src/observe.rs:918-962`: `apply_l2_imports()` に `file_exports_any_symbol()` フィルタ追加
- `crates/lang-rust/src/observe.rs` (tests): 新規テスト6件追加

### Out of Scope

- L0 FN: `#[cfg(test)] mod helper_utils;` のような非 `tests` 名モジュール (現行と同じ挙動を維持)
- L2 FN: symbols が空の import はフィルタリングしない (短絡評価、現行維持)
- `core/observe.rs` の `collect_import_matches()` は直接変更しない

### Files to Change

| File | Change |
|------|--------|
| `crates/lang-rust/src/observe.rs:135-170` | `detect_inline_tests` に `mod_item` 検証追加 |
| `crates/lang-rust/src/observe.rs:918-962` | `apply_l2_imports` に `file_exports_any_symbol` フィルタ追加 |
| `crates/lang-rust/src/observe.rs` (tests) | 新規テスト6件追加 |

## Environment

- Layer: lang-rust/observe
- Plugin: Rust
- Risk: low
- Runtime: Rust (cargo test)
- Dependencies: tree-sitter, lang-rust crate

## Risk Interview

- **L0 FN risk**: `#[cfg(test)] mod helper_utils;` のような非 `tests` 名モジュールも inline test として検出される (現行と同じ)。mod_item 検証のみで名前制限はしない。
- **L2 FN risk**: `file_exports_any_symbol()` は symbols が空のとき true を返す (短絡評価)。symbols 付きの import のみフィルタリング。

## Context & Dependencies

### Upstream References

- CONSTITUTION.md Section 7: quiet 原則 (FP を避ける方向)
- ROADMAP.md: Phase 2 — L2 re-export validation (GO 判定済み)
- Re-audit data: dogfooding-results.md の 50-pair 結果

## Implementation Notes

### Goal

残存 FP 4件を全排除。P=92.0% (46/50) → P=100% (50/50)。

### Background

Issue #163 中間 re-audit 結果:

- L0 cfg(test) false detection (2件): source.rs, open_options.rs — `#[cfg(test)]` がテストモジュール以外の用途で使われているケース
- L2 re-export chain confusion (2件): driver.rs ← shutdown.rs/yield_now.rs — `pub(crate) mod` 経由の over-mapping

### Design Approach

#### Fix A: L0 detect_inline_tests improvement

`detect_inline_tests()` (lang-rust/observe.rs:135-170) は `#[cfg(test)]` 属性を検出するだけで、次の兄弟ノードが `mod_item` (テストモジュール) かどうかを確認しない。

`find_cfg_test_ranges()` (同ファイル:398-453) にある兄弟ノード走査パターンを再利用:

```rust
pub fn detect_inline_tests(source: &str) -> bool {
    // ... existing tree-sitter parse + query ...
    if is_cfg && is_test {
        // NEW: verify next sibling is a mod_item
        if let Some(attr) = attr_node {
            let mut sibling = attr.next_sibling();
            while let Some(s) = sibling {
                if s.kind() == "mod_item" {
                    return true;  // Real inline test module
                }
                if s.kind() != "attribute_item" {
                    break;  // Next non-attribute item is not a mod
                }
                sibling = s.next_sibling();
            }
        }
    }
    // ... continue loop, return false at end
}
```

#### Fix B: L2 file_exports_any_symbol validation

`apply_l2_imports()` → `collect_import_matches()` が barrel re-export chain を辿り、`pub(crate) mod driver` 経由で driver.rs をマッピングするが、driver.rs は Builder/Handle をexportしていない。

`apply_l2_imports()` (lang-rust/observe.rs:918-962) の中で、`collect_import_matches()` の結果に対して `file_exports_any_symbol()` でフィルタリング。具体的には: `collect_import_matches()` が返す各ファイルに対し、symbols が空でなければ `self.file_exports_any_symbol(&path, &symbols)` を確認。false なら mapping に追加しない。

## Test List

### TODO

- [x] TC-01: **Given** file with `#[cfg(test)]` followed by `mod tests {}`, **When** detect_inline_tests, **Then** true (regression) — PASS ✓
- [x] TC-02: **Given** file with `#[cfg(test)]` for helper method (no mod), **When** detect_inline_tests, **Then** false — FAIL (RED ✓)
- [x] TC-03: **Given** file with `#[cfg(test)]` for mock substitution (no mod), **When** detect_inline_tests, **Then** false — FAIL (RED ✓)
- [x] TC-04: **Given** file with `#[cfg(test)] mod tests;` (external module), **When** detect_inline_tests, **Then** true (regression) — PASS ✓
- [x] TC-05: **Given** driver.rs w/o Builder, test imports `use myapp::runtime::driver::{Builder}` (direct non-barrel), **When** map_test_files_with_imports, **Then** driver.rs is NOT mapped — FAIL (RED ✓)
- [x] TC-06: **Given** barrel mod.rs with `pub mod service`, test imports `crate::app::{service_fn}`, service.rs exports `pub fn service_fn`, **When** map_test_files_with_imports, **Then** service.rs IS mapped (regression) — PASS ✓

### WIP

(none)

### DISCOVERED

(none)

### DONE

(none)

## Progress Log

- 2026-03-24 11:19: Cycle doc 作成 (sync-plan)
- 2026-03-24 11:25: REVIEW (plan) PASS score:15. No blocking issues. Optional: extract sibling-walk helper. Phase completed.
- 2026-03-24: RED phase completed. 6 tests added to crates/lang-rust/src/observe.rs. TC-01/04/06 PASS (regression), TC-02/03/05 FAIL (RED state confirmed). clippy clean.
- 2026-03-24: GREEN phase completed. Fix A: detect_inline_tests mod_item sibling check. Fix B: apply_l2_imports file_exports_any_symbol filter. All 147 tests PASS.
- 2026-03-24: REFACTOR phase completed. Sibling-walk duplication noted (deferred). Verification Gate PASS (1152 tests, clippy 0, fmt clean, BLOCK 0). Phase completed.
- 2026-03-24: REVIEW (code) PASS score:12. Security PASS(8), Correctness PASS(12). No blocking issues. Phase completed.
