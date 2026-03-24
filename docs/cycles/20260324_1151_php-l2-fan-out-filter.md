---
feature: "#129 PHP L2 fan-out filter (高頻度 utility class 抑制)"
cycle: 20260324_1151
phase: RED
complexity: standard
test_count: 5
risk_level: low
codex_session_id: ""
created: 2026-03-24 11:51
updated: 2026-03-24 11:51
---

# #129 PHP L2 fan-out filter (高頻度 utility class 抑制)

## Summary

PHP observe GT audit: P=90.0% (27/30)。全3 FP が Str.php (高頻度 utility class)。多くのテストが `use Illuminate\Support\Str` を import するが、Str を直接テストしていない。fan-out 閾値で除外する。

## Scope Definition

### In Scope

- `crates/core/src/config.rs`: ObserveConfig struct 追加、ExspecConfig に `pub observe: ObserveConfig` フィールド追加
- `crates/cli/src/main.rs:71-87`: ObserveArgs に `--no-fan-out-filter` フラグ追加
- `crates/cli/src/main.rs:344-407`: `run_observe_common()` にファンアウトフィルタ追加
- `crates/cli/src/main.rs:409+`: 各言語呼び出しに `no_fan_out_filter` 渡し

### Out of Scope

- 言語固有のレイヤー変更なし (post-processing で言語非依存)
- PHP 以外の observe ロジックへの直接変更なし

### Files to Change

| File | Change |
|------|--------|
| `crates/core/src/config.rs` | ObserveConfig struct 追加 |
| `crates/cli/src/main.rs:71-87` | ObserveArgs に --no-fan-out-filter 追加 |
| `crates/cli/src/main.rs:344-407` | run_observe_common にファンアウトフィルタ追加 |
| `crates/cli/src/main.rs:409+` | run_observe の各言語呼び出しに no_fan_out_filter 渡し |

## Environment

- Layer: core/config + cli/observe
- Plugin: 言語非依存 (全言語 post-processing)
- Risk: low
- Runtime: Rust (cargo test)
- Dependencies: crates/core, crates/cli

## Risk Interview

(low risk — no BLOCK interview required)

## Context & Dependencies

### Upstream References

- ROADMAP.md: "Fan-out filter は default ON — opt-out。閾値は configurable"
- CONSTITUTION.md Section 7: quiet 原則 (FP を避ける方向)

### Related Issues/PRs

- Issue #129: PHP L2 fan-out filter (高頻度 utility class 抑制)

## Implementation Notes

### Goal

PHP observe の P=90.0% (27/30) → P=100% (30/30)。全3 FP (Str.php) を fan-out フィルタで除外する。

### Background

PHP observe GT audit で全3 FP が `Illuminate\Support\Str` (utility class)。多くのテストが `use Illuminate\Support\Str` を import するが、Str を直接テストしていない。fan-out 閾値 (テストファイル数に対する割合) で高頻度 utility class を除外する。言語非依存の post-processing として実装し、将来的に全言語に有用。

### Design Approach

#### Fan-out filter (言語非依存、post-processing)

`run_observe_common()` の `map_fn` 呼び出し後に適用:

```rust
// Post-processing: fan-out filter
if !no_fan_out_filter {
    let total_test_files = test_files.len();
    let threshold = config.observe.max_fan_out_percent.unwrap_or(20.0) / 100.0;
    for mapping in &mut file_mappings {
        let fan_out = mapping.test_files.len() as f64 / total_test_files as f64;
        if fan_out > threshold {
            mapping.test_files.clear();
        }
    }
}
```

#### Config: `[observe]` section

`crates/core/src/config.rs` に `ObserveConfig` 追加:

```rust
#[derive(Debug, Deserialize, Default)]
pub struct ObserveConfig {
    pub max_fan_out_percent: Option<f64>,
}
```

ExspecConfig に `pub observe: ObserveConfig` フィールド追加。

#### CLI: `--no-fan-out-filter` フラグ

`ObserveArgs` に追加。`run_observe_common()` のシグネチャに `no_fan_out_filter: bool` パラメータ追加。

## Verification

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
cargo run -- --lang rust .
# PHP dogfooding は laravel/symfony で別途実施 (手動)
```

Evidence: (orchestrate が自動記入)

## Test List

### TODO

(none)

### WIP

- [x] TC-01: **Given** 10 test files, prod A mapped to 3 tests (30%), threshold 20%, **When** fan-out filter, **Then** A の test_files が空になる — RED (FAIL: stub は no-op のため未クリア)
- [x] TC-02: **Given** 10 test files, prod B mapped to 1 test (10%), threshold 20%, **When** fan-out filter, **Then** B の test_files は維持される — RED (PASS: stub no-op = 維持)
- [x] TC-03: **Given** no_fan_out_filter=true, prod A mapped to 5 tests (50%), **When** filter skipped, **Then** A の test_files は維持される — RED (PASS: filter 非呼び出し)
- [x] TC-04: **Given** config max_fan_out_percent=50, prod mapped to 4/10 (40%), **When** filter, **Then** 維持 (40% < 50%) — RED (PASS: stub no-op = 維持)
- [x] TC-05: **Given** 0 test files (edge case), **When** fan-out filter, **Then** パニックしない — RED (PASS: stub no-op)

### DISCOVERED

(none)

### DONE

(none)

## Progress Log

- 2026-03-24 11:51: Cycle doc 作成 (sync-plan)
- 2026-03-24: RED phase 完了。apply_fan_out_filter スタブ追加 + TC-01〜TC-05 作成。TC-01 FAIL (expected RED), TC-02/03/04/05 PASS (no-op stub)。clippy 0 errors。
