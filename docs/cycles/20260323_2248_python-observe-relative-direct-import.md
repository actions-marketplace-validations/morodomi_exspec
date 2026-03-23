---
feature: python-observe-relative-direct-import
cycle: 20260323_2248
phase: RED
complexity: standard
test_count: 3
risk_level: low
codex_session_id: ""
created: 2026-03-23 22:48
updated: 2026-03-23 22:48
---

# Issue #146: relative direct import の direct_import_indices 追加

## Scope Definition

### In Scope
- [ ] non-bare relative ブランチ (L991-1013) で non-barrel resolved file を `direct_import_indices` に記録
- [ ] bare relative ブランチ (L951-986) で non-barrel resolved file を `direct_import_indices` に記録

### Out of Scope
- absolute direct import ロジックの変更 (#119/#145 で実装済み)
- L1 マッチング・barrel suppression ロジックの変更
- 他言語 (TypeScript / PHP / Rust) の observe への波及

### Files to Change (target: 10 or less)
- `crates/lang-python/src/observe.rs` (edit)

## Environment

### Scope
- Layer: Backend
- Plugin: rust (cargo test)
- Risk: 15 (PASS)

### Runtime
- Language: Rust

### Dependencies (key packages)
- tree-sitter: workspace
- exspec-core: workspace (observe trait, collect_import_matches)

### Risk Interview (BLOCK only)
(not applicable — PASS)

## Context & Dependencies

### Reference Documents
- CONSTITUTION.md - static AST analysis only。assertion filter bypass は static 解析の範囲内
- ROADMAP.md v0.4.2 - #119 の追加ギャップとして v0.4.2 スコープ内
- Issue #145 (#119 PR) - absolute direct import bypass の実装 (横展開元)

### Dependent Features
- absolute direct import bypass (#119/#145): 既存実装。同パターンを relative ブランチに横展開
- assertion filter (PY-AF-06a/06b/09): T3 回帰確認で動作保護

### Related Issues/PRs
- Issue #146: relative direct import の direct_import_indices 追加
- PR #145: bypass assertion filter for direct sub-module imports (upstream reference)

## Test List

### TODO
- [x] TC-01 (PY-SUBMOD-05): non-bare relative direct import bypass
  - Given: `pkg/_config.py` (non-barrel), `pkg/_client.py`, `pkg/__init__.py` (barrel: re-exports Client), `pkg/test_app.py` に `import pkg` + `from ._config import Config` 相対 import、assertion は `pkg.Client()` (Config 未使用)
  - When: `map_test_files_with_imports` を実行
  - Then: `test_app.py` は `pkg/_config.py` にマップされる (direct_import_indices 経由で assertion filter bypass)
  - Status: RED confirmed (FAIL — relative non-bare branch に direct_import_indices ロジック未追加)
- [x] TC-02 (PY-SUBMOD-06): bare relative direct import bypass
  - Given: `pkg/utils.py` (non-barrel), `pkg/_client.py`, `pkg/__init__.py` (barrel: re-exports Client), `pkg/test_app.py` に `import pkg` + `from . import utils` bare relative import、assertion は `pkg.Client()` (utils 未使用)
  - When: `map_test_files_with_imports` を実行
  - Then: `test_app.py` は `pkg/utils.py` にマップされる (direct_import_indices 経由で assertion filter bypass)
  - Status: RED confirmed (FAIL — relative bare branch に direct_import_indices ロジック未追加)
- [x] TC-03 (PY-SUBMOD-01-04 回帰): 既存 absolute direct import テストが引き続き PASS すること
  - Status: PASS (241 passed, 2 failed)

### WIP
(none)

### DONE
- TC-01, TC-02, TC-03 テスト作成完了 (RED phase)

### DISCOVERED
(none)

### DONE
(none)

## Implementation Notes

### Goal

#119/#145 で実装した absolute direct import bypass を relative import ブランチ (`from ._sub import X`, `from . import sub`) にも適用し、relative direct import でも assertion filter を bypass できるようにする。

### Background

#119/#145 で L2 absolute ループに `direct_import_indices` を追加したが、relative import の2ブランチ (non-bare / bare) には同ロジックが未適用のまま。correctness reviewer (#145 REVIEW phase) が asymmetry を指摘し、Issue #146 として filed された。

#### Root Cause

```
tests/test_app.py:
  import pkg              → barrel → _config.py, etc.
  from ._config import Config  → relative direct → _config.py

assertions: assert pkg.something()
  → asserted_imports = {"something", "pkg", ...}
  → _config.py idx_to_symbols = {"Config"} → NOT in asserted_imports
  → asserted_matched non-empty (barrel indices pass)
  → _config.py excluded (relative branch に bypass ロジックなし)
```

### Design Approach

**relative direct import (non-barrel) も absolute と同様に assertion filter を bypass する。**

根拠: `from ._submodule import X` / `from . import sub` は「そのモジュールをテストする意図」の強いシグナル。absolute 同様、明示的に特定の production file を指定している。

#### 実装

修正箇所: `crates/lang-python/src/observe.rs`

**変更1: non-bare relative ブランチ (L991-1013)**

```rust
// L1003 の before 定義は既存
let before = all_matched.clone();
// ... collect_import_matches + track_new_matches (既存) ...

// 追加: direct (non-barrel) relative import → assertion filter bypass
let is_direct = !self.is_barrel_file(&resolved);
if is_direct {
    for &idx in all_matched.difference(&before) {
        direct_import_indices.insert(idx);
    }
}
```

**変更2: bare relative ブランチ (L951-986)**

```rust
// L969 の before 定義は既存
let before = all_matched.clone();
// ... collect_import_matches + track_new_matches (既存) ...

// 追加: direct (non-barrel) bare relative import → assertion filter bypass
if !self.is_barrel_file(&resolved) {
    for &idx in all_matched.difference(&before) {
        direct_import_indices.insert(idx);
    }
}
```

注意: bare relative は barrel suppression で既に `is_barrel_file` チェック済み (L963)。barrel の場合は L966 で `continue` されるため、ここに到達する resolved は必ず non-barrel。`if !self.is_barrel_file(&resolved)` は常に true だが、absolute ブランチとの一貫性のために明示的に書く。

#### Verification

```bash
cargo test -p exspec-lang-python -- py_submod   # SUBMOD テスト全体
cargo test                                       # 全テスト
cargo clippy -- -D warnings                     # 静的解析
cargo fmt --check                               # フォーマット
cargo run -- --lang rust .                      # self-dogfooding
```

## Progress Log

### 2026-03-23 22:48 - INIT
- Cycle doc created from plan file buzzing-launching-wand.md
- Design Review Gate: PASS (score 15/100)
- Scope definition ready

### 2026-03-23 RED - テスト作成完了
- PY-SUBMOD-05, PY-SUBMOD-06 を `crates/lang-python/src/observe.rs` に追加
- テストシナリオ: L1 stem match を避けるため `test_app.py` (stem "app") を使用。barrel + relative direct import の組み合わせで asserted_matched が non-empty になるシナリオを構成
- RED 確認: py_submod_05, py_submod_06 が FAIL (期待通り)
- 既存テスト: 241 PASS (回帰なし)
