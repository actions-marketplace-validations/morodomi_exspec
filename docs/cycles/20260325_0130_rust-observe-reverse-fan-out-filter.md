---
feature: rust-observe-reverse-fan-out-filter
cycle: 20260325_0130
phase: RED_DONE
complexity: standard
test_count: 10
risk_level: medium
codex_session_id: ""
created: 2026-03-25 01:30
updated: 2026-03-25 01:30
---

# Rust observe: barrel import fan-out precision fix (reverse fan-out filter)

## Scope Definition

### In Scope
- [ ] `apply_reverse_fan_out_filter()` 関数の実装 (crates/cli/src/main.rs)
- [ ] `ObserveConfig` に `max_reverse_fan_out: Option<usize>` を追加 (crates/core/src/config.rs)
- [ ] デフォルト値 `max_reverse_fan_out: 5` の設定 (crates/core/src/rules.rs)
- [ ] `run_observe_common()` 内での呼び出し追加
- [ ] Unit tests x7 + Integration tests x3 の作成

### Out of Scope
- 既存 `apply_fan_out_filter()` (プロダクション→テスト軸) の変更 (今回は逆軸のみ)
- Rust 以外の言語への適用 (今回は Rust observe のみ)
- `.exspec.toml` での per-language 設定分離 (将来課題)

### Files to Change (target: 10 or less)
- `crates/cli/src/main.rs` (edit) — `apply_reverse_fan_out_filter()` 追加 + `run_observe_common()` 呼出追加
- `crates/core/src/config.rs` (edit) — `ObserveConfig` に `max_reverse_fan_out` フィールド追加
- `crates/core/src/rules.rs` (edit) — デフォルト値 `max_reverse_fan_out: 5`

## Environment

### Scope
- Layer: Backend (Rust, CLI)
- Plugin: rust
- Risk: 45/100 (WARN) — 既存 fan-out filter に新軸を追加。全言語の observe 出力に間接影響

### Runtime
- Language: Rust (edition 2021)

### Dependencies (key packages)
- tree-sitter: workspace
- clap: workspace

### Risk Interview (BLOCK only)
- (WARN レベルのため省略)

## Context & Dependencies

### Reference Documents
- [CONSTITUTION.md](../../CONSTITUTION.md) — "Ship criteria: Precision >= 98%"
- [docs/STATUS.md](../STATUS.md) — "barrel import fan-out is blocking issue"
- [docs/dogfooding-results.md](../dogfooding-results.md) — "Previous P=100% was misleading", GT re-audit で P=23.3% が判明

### Dependent Features
- `apply_fan_out_filter()`: crates/cli/src/main.rs — 既存の順方向 fan-out filter。本機能は直後に適用
- `extract_class_name()`: crates/cli/src/main.rs — name-match ロジックで再利用

### Related Issues/PRs
- (なし)

## Test List

### TODO
- [ ] RF-INT-01: Run `cargo run -- observe --lang rust --format json /tmp/exspec-dogfood/tokio`, verify io_driver.rs maps to <= 5 prods
- [ ] RF-INT-02: Run same, verify fs_write.rs maps to fs/write.rs (possibly + secondary)
- [ ] RF-INT-03: Run same, verify overall precision improves vs GT (目標: P=23.3% → 80%+)

### WIP
(none)

### DONE
- [x] RF-01: Given test mapped to 10 prods (> threshold 5), When reverse filter applied, Then only name-matched prods remain — FAILED (RED state verified)
- [x] RF-02: Given test mapped to 3 prods (< threshold 5), When reverse filter applied, Then all prods retained — PASSED (no-op stub correct)
- [x] RF-03: Given test with L1 filename match + L2 barrel fan-out, When reverse filter, Then L1 match preserved + L2 fan-out trimmed (fs_write → fs/write.rs のみ保持) — FAILED (RED state verified)
- [x] RF-04: Given test mapped to exactly threshold prods (=5), When reverse filter, Then all kept (strictly greater than で判定) — PASSED (no-op stub correct)
- [x] RF-05: Given empty mappings, When reverse filter, Then no panic — PASSED (no-op stub correct)
- [x] RF-06: Given reverse filter with custom threshold=10, When test maps to 8 prods, Then all kept (閾値未満) — PASSED (no-op stub correct)
- [x] RF-07: Given test where prod_stem contains test_stem (reverse name-match), When filter, Then kept (broadcast.rs → sync/broadcast.rs 保持) — FAILED (RED state verified)
- [ ] RF-INT-01: Run `cargo run -- observe --lang rust --format json /tmp/exspec-dogfood/tokio`, verify io_driver.rs maps to <= 5 prods
- [ ] RF-INT-02: Run same, verify fs_write.rs maps to fs/write.rs (possibly + secondary)
- [ ] RF-INT-03: Run same, verify overall precision improves vs GT (目標: P=23.3% → 80%+)

### DISCOVERED
(none)

## Implementation Notes

### Goal
Rust observe の Precision を GT re-audit 結果 P=23.3% から Ship criteria P>=98% に引き上げる。
barrel import fan-out (1テストが多数 prod にマッピング) を逆軸 fan-out filter で抑制する。

### Background
GT re-audit で Rust observe P=23.3% が判明。主原因は barrel import fan-out:
- `io_driver.rs`: `use tokio::runtime::Builder` → runtime/配下40ファイルにマッピング (39 FP)
- `fs_write.rs`: `use tokio::fs` → fs/配下25ファイルにマッピング (23 FP)

既存 `apply_fan_out_filter()` は「プロダクション→テスト」軸 (1プロダクションが多数テストにマッピング) を扱うが、
今回は逆軸「テスト→プロダクション」(1テストが多数 prod にマッピング) が問題。

### Design Approach
`apply_reverse_fan_out_filter()` を CLI post-processing に追加する。

**アルゴリズム**:
1. 逆インデックス構築: test_file → [production_file indices]
2. 各 test_file について、マッピング先 prod 数 > `max_reverse_fan_out` の場合:
   - test_stem と prod_stem の name-match を検査
   - `test_stem.contains(&prod_stem) || prod_stem.contains(&test_stem)`
   - name-match する prod のみ保持、それ以外からこの test を除去
3. 既存 `apply_fan_out_filter()` の後に適用

**閾値**: `max_reverse_fan_out = 5` (デフォルト)
- io_driver.rs (40 prods) → 閾値超過 → name-match filter → 0 prod (name-match なし)
- fs_write.rs (25 prods) → 閾値超過 → name-match filter → fs/write.rs のみ保持
- udp.rs (4 prods) → 閾値以下 → そのまま保持

**呼び出し箇所** (`run_observe_common()` main.rs:379-385 付近):
```rust
if !no_fan_out_filter {
    apply_fan_out_filter(&mut file_mappings, test_files.len(), config.max_fan_out_percent);
    apply_reverse_fan_out_filter(&mut file_mappings, config.max_reverse_fan_out);
}
```

## Verification

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
cargo run -- --lang rust .

# 精度検証
cargo run -- observe --lang rust --format json /tmp/exspec-dogfood/tokio > /tmp/rust-observe-post.json
python3 scripts/evaluate_observe.py \
  --observe-json /tmp/rust-observe-post.json \
  --ground-truth docs/observe-ground-truth-rust-tokio.md \
  --scan-root /tmp/exspec-dogfood/tokio
# 期待: P が 23.3% → 80%+ に改善
```

Evidence: (orchestrate が自動記入)

## Progress Log

### 2026-03-25 01:30 - INIT
- Cycle doc created from plan file abstract-honking-liskov.md
- Scope definition ready

### 2026-03-25 - RED
- `apply_reverse_fan_out_filter()` no-op stub added at line 727 (after `apply_fan_out_filter`)
- RF-01〜RF-07 unit tests added in `#[cfg(test)] mod tests` (after fan-out filter tests)
- RED state verified: RF-01, RF-03, RF-07 FAIL / RF-02, RF-04, RF-05, RF-06 PASS (no-op stub correct behavior)

---

## Next Steps

1. [Done] INIT <- Current
2. [Done] PLAN
3. [Done] RED
4. [Done] GREEN
5. [Done] REFACTOR
6. [Done] REVIEW
7. [Next] COMMIT
4. [ ] GREEN
5. [ ] REFACTOR
6. [ ] REVIEW
7. [ ] COMMIT
