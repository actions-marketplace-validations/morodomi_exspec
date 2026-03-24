---
feature: "PHP Str.php FP: fan-out filter with name-match exemption"
cycle: 20260324_1514
phase: RED
complexity: standard
test_count: 5
risk_level: low
codex_session_id: ""
created: 2026-03-24 15:14
updated: 2026-03-24 15:14
---

# PHP Str.php FP: fan-out filter with name-match exemption

## Summary

PHP observe P=96.0%。FP は Str.php incidental import (2/50)。fan-out 20% 閾値では Str.php (6.7%) を捕捉できない。fan-out filter に name-match 保護を追加し、閾値を 5% に下げることで、Str.php の FP 60件を除去しつつ SupportStrTest.php を保持する。

## Scope Definition

### In Scope

- `crates/cli/src/main.rs`: `apply_fan_out_filter()` に name-match 保護追加、`extract_class_name()` 関数追加
- `crates/core/src/config.rs`: `max_fan_out_percent` の default を 20.0 → 5.0 に変更
- `crates/core/src/rules.rs`: default 20.0 → 5.0 に変更

### Out of Scope

- 言語固有のレイヤー変更なし (post-processing で言語非依存)
- PHP 以外の observe ロジックへの直接変更なし

### Files to Change

| File | Change |
|------|--------|
| `crates/cli/src/main.rs` | `apply_fan_out_filter` に name-match 保護追加、`extract_class_name` 関数追加 |
| `crates/core/src/config.rs` | default 20.0 → 5.0 |
| `crates/core/src/rules.rs` | default 20.0 → 5.0 |

## Environment

- Layer: cli/observe (post-processing)
- Plugin: 言語非依存 (全言語 post-processing)
- Risk: low
- Runtime: Rust (cargo test)
- Dependencies: crates/core, crates/cli

## Risk Interview

(low risk — no BLOCK interview required)

## Context & Dependencies

### Upstream References

- ROADMAP.md: fan-out filter 設計方針 (default ON, opt-out, configurable threshold)
- CONSTITUTION.md Section 7: quiet 原則 (FP を避ける方向)
- `docs/cycles/20260324_1151_php-l2-fan-out-filter.md`: fan-out filter 初期実装

### Related Issues/PRs

- Laravel fan-out 分布: Model.php 20%, Blueprint 13.9%, Schema 13.2%, Carbon 12.1%, Container 10.7%, DB 7.0%, Str 6.7%
- 全て L2-only (L1 マッチなし)。全て test-name 部分一致あり

## Implementation Notes

### Goal

PHP observe P=96.0% → P=100%。Str.php incidental import FP を fan-out filter + name-match 保護で除去。SupportStrTest.php は保持。

### Background

Laravel の Str.php は fan-out 6.7%。現在の 20% 閾値では捕捉できない。しかし閾値を下げると Model.php (20%) や Blueprint.php (13.9%) まで除去してしまう。これらは name-match tests (EloquentModelTest, DatabaseSchemaBlueprint*) を持つため KEEP すべき。

Str.php の 61 テストのうち `SupportStrTest.php` だけが primary target。残り 60 は incidental import。

name-match 保護: fan-out > threshold のとき、テストファイル名に production class 名を含むものだけ KEEP する。

### Design Approach

#### apply_fan_out_filter() — name-match 保護追加

```rust
if fan_out > threshold {
    let prod_class = extract_class_name(&mapping.production_file);
    mapping.test_files.retain(|test_file| {
        let test_name = extract_file_stem(test_file);
        test_name.to_lowercase().contains(&prod_class.to_lowercase())
    });
}
```

`extract_class_name()`: パスからクラス名を抽出 (e.g., `src/Support/Str.php` → `Str`, `src/user.rs` → `user`)

#### 閾値変更: 20% → 5%

`crates/core/src/config.rs` と `crates/core/src/rules.rs` の `max_fan_out_percent` default を 5.0 に変更。

#### 効果 (Laravel)

| File | Fan-out | Name-match tests | 結果 |
|------|---------|-----------------|------|
| Model.php | 20% | EloquentModel*.php (多数) | KEEP (name-match) |
| Str.php | 6.7% | SupportStrTest.php (1件) | 1件KEEP, 60件REMOVE |
| Carbon.php | 12.1% | SupportCarbonTest.php (1件) | 1件KEEP, 109件REMOVE |
| Blueprint.php | 13.9% | DatabaseSchemaBlueprint*.php | KEEP (name-match) |
| Manager.php | 5.6% | なし | 全REMOVE |

## Verification

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
cargo run -- --lang rust .                        # BLOCK 0
# laravel dogfooding: Str.php reduced to ~1 test (SupportStrTest), Model.php preserved
```

Evidence: (orchestrate が自動記入)

## Test List

### TODO

- [ ] TC-01: **Given** fan-out > 5%, test name contains prod class, **When** filter, **Then** test KEPT
- [ ] TC-02: **Given** fan-out > 5%, test name does NOT contain prod class, **When** filter, **Then** test REMOVED
- [ ] TC-03: **Given** fan-out <= 5%, **When** filter, **Then** all tests KEPT (below threshold)
- [ ] TC-04: **Given** mixed tests (some match, some don't), **When** filter, **Then** only matching KEPT
- [ ] TC-05: **Given** --no-fan-out-filter, fan-out > 5%, **When** filter disabled, **Then** all KEPT (existing regression)

### WIP

(none)

### DISCOVERED

(none)

### DONE

(none)

## Progress Log

- 2026-03-24 15:14: Cycle doc 作成 (sync-plan)
