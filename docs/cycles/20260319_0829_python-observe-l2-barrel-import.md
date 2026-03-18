---
feature: "Phase 14 — Python observe L2 barrel import resolution"
cycle: "20260319_0829"
phase: RED
complexity: standard
test_count: 5
risk_level: medium
codex_session_id: ""
created: 2026-03-19 08:29
updated: 2026-03-19 08:29
---

# Cycle: Phase 14 — Python observe L2 barrel import resolution

## Scope Definition

### In Scope
- `re_export.scm` に wildcard import pattern 追加 (`from .module import *`)
- `extract_barrel_re_exports()` で wildcard フラグを正しく設定
- テスト追加 (barrel wildcard / barrel named multi-symbol / e2e)
- httpx + Requests 再 dogfooding で改善確認

### Out of Scope
- L1 cross-directory matching (別 Issue)
- Multi-path CLI (別 Issue)
- `namespace_wildcard` (Python に該当する構文なし)

### Files to Change
- `crates/lang-python/queries/re_export.scm`
- `crates/lang-python/src/observe.rs`

## Environment

### Scope
- Layer: `crates/lang-python/queries/re_export.scm`, `crates/lang-python/src/observe.rs`
- Plugin: dev-crew:python-quality は不使用 (Rust crate)
- Risk: 35/100 (WARN)
- Runtime: Rust (cargo test)
- Dependencies: tree-sitter (既存、追加依存なし)

### Risk Interview

(WARN — リスク 35/100)
- wildcard pattern 追加による FP: `re_export.scm` に新規 capture `@wildcard` を追加するため、既存 named import との干渉を確認が必要
- `extract_barrel_re_exports()` の wildcard 検出: capture index が正しく取得されることを unit test で確認
- core インフラは既に統合済みのため、Python 側の query + extraction 修正のみで完結する想定

## Context & Dependencies

### Background

Phase 12 dogfooding で httpx の 28 FN の原因が `__init__.py` barrel import 未解決と判明。`import httpx` や `from httpx import Client` が `httpx/__init__.py` の re-export を追跡できていない。

Phase 13 で P0 (L1 `_` prefix, src/ layout) は修正済み。本フェーズは P1 の最大影響タスク。

根本原因:
1. wildcard import (`from ._api import *`) を検出できない (re_export.scm に pattern なし)
2. `wildcard: false` をハードコード

core の `resolve_barrel_exports`, `collect_import_matches`, `file_exports_any_symbol` は既に統合済みで動作する。Python 側の query + extraction が不完全なだけ。

### Real-world Patterns

- **httpx `__init__.py`**: `from ._api import *` (wildcard 9件)
- **requests `__init__.py`**: `from .api import delete, get, head, ...` (named multi-symbol)

### Design Approach

**1. re_export.scm 変更**

```scheme
;; Wildcard re-export: from .module import *
(import_from_statement
  module_name: (_) @from_specifier
  (wildcard_import) @wildcard)
```

**2. extract_barrel_re_exports() 変更**

- `@wildcard` capture index を取得
- match 内で wildcard capture を検出したら `is_wildcard = true`
- `BarrelReExport` の `wildcard` フィールドを正しく設定
- wildcard の場合は symbols は空 Vec

**3. 既存インフラの動作確認**

core の `resolve_barrel_exports_inner()` は wildcard: true なら全 re-export を追跡。Python 側の `file_exports_any_symbol()` は既に実装済み。

## Test List

### TODO
(none)

### WIP
(none)

### DISCOVERED
(none)

### DONE
- [x] PY-BARREL-05: `from .module import *` が wildcard=true で抽出される
- [x] PY-BARREL-06: `from .module import Foo, Bar` が named (wildcard=false) で抽出される
- [x] PY-BARREL-07: e2e: test imports `from pkg import Foo`, `pkg/__init__.py` has `from .module import *`, `pkg/module.py` has Foo → mapped
- [x] PY-BARREL-08: e2e: test imports `from pkg import Foo`, `pkg/__init__.py` has `from .module import Foo` (named) → mapped
- [x] PY-BARREL-09: e2e: `from pkg import NonExistent`, `pkg/__init__.py` has `from .module import *`, module has `__all__=["Foo"]` → NOT mapped
(none)

## Progress Log

### 2026-03-19 08:29 — Cycle doc 作成 (sync-plan)

planファイルから Cycle doc を生成。
Phase 12 dogfooding で判明した httpx の 28 FN の根本原因 (barrel import 未解決) を修正するサイクルを開始。
test_count: 5 (unit: 2, e2e: 3)
