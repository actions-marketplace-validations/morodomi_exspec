---
feature: "Phase 15 — Python observe bare import statement resolution"
cycle: "20260319_1200"
phase: RED
complexity: standard
test_count: 5
risk_level: low
codex_session_id: ""
created: 2026-03-19 12:00
updated: 2026-03-19 12:00
---

# Cycle: Phase 15 — Python observe bare import statement resolution

## Scope Definition

### In Scope
- `extract_all_import_specifiers()` で `@import_name` capture を処理 (bare `import X`)
- bare `import` → specifier = package name, symbols = [] (全エクスポートにマッチ)
- テスト追加 (unit: bare import / dotted bare import / regression, e2e: barrel経由マッピング x2)

### Out of Scope
- `import os.path as p` (alias 付き bare import) — 別 Issue
- `from . import views` (relative bare import) — 既存処理で対応済み
- L3 semantic duplication detection (CONSTITUTION Non-Goals)

### Files to Change
- `crates/lang-python/src/observe.rs`

## Environment

### Scope
- Layer: `crates/lang-python/src/observe.rs`
- Plugin: dev-crew:python-quality は不使用 (Rust crate)
- Risk: 25/100 (PASS)
- Runtime: Rust (cargo test)
- Dependencies: tree-sitter (既存、追加依存なし)

### Risk Interview

(PASS — リスク 25/100)
- `@import_name` capture は `import_mapping.scm` に既に存在するが Rust 側で無視されていた。scm 変更不要。
- 既存ペアチェック (L303-306) を迂回する分岐を追加するだけで影響範囲が限定的。
- symbols = [] とすることで `file_exports_any_symbol()` が全シンボルにマッチ。既存インフラを活用。
- regression リスク: PY-IMPORT-03 で `from X import Y` 処理が壊れないことを明示的に確認。

## Context & Dependencies

### Background

Phase 14 で wildcard barrel re-export を修正したが、httpx の dogfooding 改善はほぼなし (2→3 mapped)。
調査の結果、httpx テストの93%が `import httpx` (bare import) を使用していることが判明。

根本原因:
- `import_mapping.scm` の `@import_name` capture (bare `import X`) が Rust 側で完全に無視されている
- `extract_all_import_specifiers()` は `@module_name` + `@symbol_name` のペアのみ処理
- bare import は symbols なしで全エクスポートにマッチすべきだが、そのロジックが未実装

Issue: #116

### Real-world Patterns

- **httpx テスト**: `import httpx` → `httpx.Client(...)` のように bare import 後にドット記法でアクセス
- **標準ライブラリ**: `import os.path` → dotted bare import。`os/path` に変換が必要

### Design Approach

**1. import_mapping.scm 変更不要**

`@import_name` capture は既に存在する。コメント "captured for completeness but skipped in Rust" を解消するだけ。

**2. extract_all_import_specifiers() 変更**

```rust
// 既存
let module_name_idx = query.capture_index_for_name("module_name");
let symbol_name_idx = query.capture_index_for_name("symbol_name");
// 追加
let import_name_idx = query.capture_index_for_name("import_name");

// match loop 内:
// @module_name + @symbol_name → 既存処理 (from X import Y)
// @import_name のみ → bare import 処理 (import X)
//   specifier = python_module_to_absolute_specifier(&import_name_text)
//   symbols = [] (全エクスポートにマッチ)
```

**3. 既存インフラとの整合**

`file_exports_any_symbol()` は symbols が空 Vec の場合に全エクスポートにマッチする実装。
`python_module_to_absolute_specifier()` を再利用して `os.path` → `os/path` 変換。

## Test List

### TODO
- [ ] PY-IMPORT-01: `import httpx` が extract_all_import_specifiers で specifier="httpx", symbols=[] として抽出される
- [ ] PY-IMPORT-02: `import os.path` が specifier="os/path", symbols=[] として抽出される
- [ ] PY-IMPORT-03: `from httpx import Client` が specifier="httpx", symbols=["Client"] として抽出される (regression)
- [ ] PY-IMPORT-04: e2e: `import pkg`, `pkg/__init__.py` has `from .module import *`, `pkg/module.py` has Foo → module.py mapped
- [ ] PY-IMPORT-05: e2e: `import pkg`, `pkg/__init__.py` has `from .module import Foo` (named) → module.py mapped

### WIP
(none)

### DISCOVERED
(none)

### DONE
(none)

## Progress Log

### 2026-03-19 12:00 — Cycle doc 作成 (sync-plan)

planファイルから Cycle doc を生成。
Phase 14 wildcard barrel 修正後も httpx dogfooding が改善しない原因として bare import 未処理を特定。
`import_mapping.scm` の `@import_name` capture が Rust 側で無視されていた根本原因を修正するサイクルを開始。
test_count: 5 (unit: 3, e2e: 2)
Design Review Gate: PASS (スコア 20/100)
