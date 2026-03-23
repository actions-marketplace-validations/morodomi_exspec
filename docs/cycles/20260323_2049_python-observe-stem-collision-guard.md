---
feature: python-observe-stem-collision-guard
cycle: 20260323_2049
phase: DONE
complexity: standard
test_count: 4
risk_level: low
codex_session_id: ""
created: 2026-03-23 20:49
updated: 2026-03-23 21:05
---

# Python observe stem collision guard (#126)

## Scope Definition

### In Scope
- [ ] `crates/lang-python/src/observe.rs` の stem-only fallback ブロックに `prod_indices.len() == 1` ガードを追加
- [ ] L874-876 のコメント ("recall takes priority") を precision guard に修正

### Out of Scope
- L2 import tracing 自体の変更 (理由: 今回はガードのみ。L2 は既存実装に委ねる)
- 他言語の observe への変更 (理由: Python 固有の問題)

### Files to Change (target: 10 or less)
- `crates/lang-python/src/observe.rs` (edit)
- `crates/lang-python/src/observe.rs` (edit) — PY-L1X-04 期待値変更 + PY-L1X-06/07 新規追加 (inline tests)

## Environment

### Scope
- Layer: Backend
- Plugin: rust
- Risk: 20/100 (PASS)

### Runtime
- Language: Rust (stable)

### Dependencies (key packages)
- tree-sitter: workspace
- exspec-core: workspace

### Risk Interview (BLOCK only)
(該当なし — Risk 20/100 PASS)

## Context & Dependencies

### Reference Documents
- [ROADMAP.md] - v0.4.2 scope に #126 が明記済み
- [CONSTITUTION.md] - observe ship criteria: Precision >= 98%, Recall >= 90%

### Dependent Features
- Python observe L1 core (ディレクトリ+stem ペア): `crates/lang-python/src/observe.rs`
- Python observe L2 import tracing: `crates/lang-python/src/observe.rs`

### Related Issues/PRs
- Issue #126: Python observe stem collision guard

## Test List

### TODO
(none)

### WIP → DONE (RED phase)
- [x] TC-01: PY-L1X-04 -- stem collision は L1 でマップしない (期待値変更) -- FAIL (expected)
- [x] TC-02: PY-L1X-06 -- stem collision + L2 import で正しくマップ + strategy==ImportTracing (新規) -- FAIL (expected)
- [x] TC-03: PY-L1X-07 -- stem collision + barrel import で fan-out 正常動作 (新規) -- FAIL (expected)
- [x] TC-04: PY-L1X-01 -- 1対1 stem match は変わらない (回帰確認) -- PASS (expected)

### WIP
(none)

### DISCOVERED
- [ ] relative direct import (`from ._sub import X`) が `direct_import_indices` に追加されない (assertion filter bypass が relative import に効かない) -- #119 の追加ギャップ
- [ ] `direct_import_indices.intersection(&all_matched)` は構築上 `direct_import_indices` と等価 -- 可読性改善

### DONE
- [x] RED phase: 4テスト作成完了。TC-01/02/03 FAIL、TC-04 PASS を確認

## Implementation Notes

### Goal
Python observe の stem-only fallback で同一 stem を持つ複数の production file が存在する場合、全てにマップする代わりに L2 import tracing に委ねることで precision を向上させる。

### Background
Python observe の stem-only fallback は、L1 core（ディレクトリ+stem ペア）で未マッチのテストに対し stem のみで cross-directory マッチを行う。現状は複数の production file が同一 stem を持つ場合、全てにマップする（recall 優先）。`models.py`, `utils.py` 等の common name が複数ディレクトリに存在する大規模プロジェクトで precision が低下するリスクがある。

### Design Approach
`crates/lang-python/src/observe.rs` L901 付近の stem fallback ブロックに `prod_indices.len() > 1` の早期 continue を追加する。

```rust
// 変更後:
if let Some(prod_indices) = stem_to_prod_indices.get(tstem) {
    if prod_indices.len() > 1 {
        continue; // stem collision: defer to L2 import tracing
    }
    for &idx in prod_indices {
```

stem collision の場合は L2 import tracing に委ねる。L2 でもインポートなしなら未マッチとなる（acceptable — precision 優先）。

## Progress Log

### 2026-03-23 20:49 - INIT
- Cycle doc created from plan file `/Users/morodomi/.claude/plans/buzzing-launching-wand.md`
- Scope definition ready

### 2026-03-23 21:05 - RED
- TC-01: `py_l1x_04_stem_ambiguity_maps_to_all` を `py_l1x_04_stem_collision_defers_to_l2` に変更。アサーションを反転（!client_mapped, !aio_mapped）
- TC-02: `py_l1x_06_stem_collision_with_l2_import_resolves_correctly` を新規追加。strategy==ImportTracing の検証を含む
- TC-03: `py_l1x_07_stem_collision_with_barrel_import_resolves_correctly` を新規追加。barrel __init__.py 経由での L2 解決
- TC-04: `py_l1x_01_stem_only_fallback_cross_directory` (1対1 stem) は変更なし、PASS 確認
- `cargo test -p exspec-lang-python -- py_l1x` 結果: 4 passed, 3 failed (TC-01/02/03 FAIL, TC-04+既存 PASS)

### 2026-03-23 21:10 - GREEN
- L901 に `if prod_indices.len() > 1 { continue; }` ガード追加
- L874-876 コメント更新 (precision guard の意図を明記)
- 全テスト PASS (1,117 tests)

### 2026-03-23 21:12 - REFACTOR
- チェックリスト7項目を確認、改善不要
- cargo fmt の差分を修正 (TC-07 の write 呼び出しチェーン)
- Verification Gate: PASS (tests 1,117, clippy 0, fmt OK, self-dogfooding BLOCK 0)
- Phase completed

### 2026-03-23 21:15 - REVIEW
- Security review: PASS (score 5) -- optional 2件
- Correctness review: PASS/WARN (score 25) -- important 1件 (relative import gap, スコープ外)
- Aggregate: PASS (score 15)
- DISCOVERED: relative direct import の assertion filter bypass 未対応、intersection 可読性改善
- Phase completed

---

## Next Steps

1. [Done] INIT <- Current
2. [Done] PLAN
3. [Next] RED
4. [ ] GREEN
5. [ ] REFACTOR
6. [ ] REVIEW
7. [ ] COMMIT
