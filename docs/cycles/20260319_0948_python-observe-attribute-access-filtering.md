---
feature: "Phase 16 — Python observe attribute-access filtering for bare import precision"
cycle: "20260319_0948"
phase: DONE
complexity: trivial
test_count: 6
risk_level: low
codex_session_id: ""
created: 2026-03-19 09:48
updated: 2026-03-19 09:48
---

# Cycle: Phase 16 — Python observe attribute-access filtering for bare import precision

## Scope Definition

### In Scope
- `crates/lang-python/queries/bare_import_attribute.scm` 新規作成: 属性アクセス query
- `extract_bare_import_attributes()` 追加: テストファイル内の `pkg.Symbol` アクセスを抽出
- `extract_all_import_specifiers()` で呼び出し: bare import 時に symbols を属性アクセスから設定
- テスト追加 (unit: PY-ATTR-01〜PY-ATTR-05, e2e: PY-ATTR-06)

### Out of Scope
- `import os.path as p` (alias 付き bare import) — 別 Issue
- `from . import views` (relative bare import) — 既存処理で対応済み
- L3 semantic duplication detection (CONSTITUTION Non-Goals)

### Files to Change
- `crates/lang-python/queries/bare_import_attribute.scm` (新規)
- `crates/lang-python/src/observe.rs`

## Environment

### Scope
- Layer: `crates/lang-python/queries/` + `crates/lang-python/src/observe.rs`
- Plugin: dev-crew:python-quality は不使用 (Rust crate)
- Risk: 30/100 (WARN)
- Runtime: Rust (cargo test)
- Dependencies: tree-sitter (既存、追加依存なし)

### Risk Interview

(WARN — リスク 30/100)
- 新規 .scm ファイル追加: 属性アクセスパターン `pkg.Symbol` を取得するクエリが必要。tree-sitter Python grammar のノード名を確認すること。
- `extract_bare_import_attributes()` は bare import 時のみ呼び出される。from-import パスへの影響なし。
- 属性アクセスが存在しない場合 (PY-ATTR-04) は symbols=[] を維持する必要がある (fallback)。
- regression リスク: PY-ATTR-05 で `from httpx import Client` が既存処理のまま symbols=["Client"] となることを明示的に確認。

## Context & Dependencies

### Background

Phase 15 で bare `import` 文を解決し、httpx の mapped rate を 10.3% → 69.0% に改善した。
しかし `symbols=[]` (空) で全 re-export にマッチするため、30テストファイルが全20 production files に一律マッピングされる。
精度 (Precision) が低い状態。

根本原因:
- bare import 後にテストファイル内で `pkg.Symbol` としてアクセスされている属性名が未抽出
- symbols=[] のままでは絞り込みが不可能で全ファイルにマッチ

Issue: Phase 15 フォローアップ

### Real-world Patterns

- **httpx テスト**: `import httpx; httpx.Client(...)` → `Client` を symbols に設定することで `_client.py` のみにマッピング
- **複数属性**: `httpx.Client(); httpx.get()` → symbols=["Client", "get"] (重複排除)
- **属性なし**: `import httpx` のみで属性アクセスが存在しない場合 → symbols=[] (fallback: 全マッチ維持)

### Design Approach

**1. bare_import_attribute.scm 新規作成**

テストファイル内の `<package>.<attribute>` パターンを抽出するクエリ。
キャプチャ名: `@pkg_name`, `@attr_name`

**2. extract_bare_import_attributes() 追加**

```rust
fn extract_bare_import_attributes(
    source: &str,
    pkg_name: &str,
) -> Vec<String>
```

- pkg_name に一致する属性アクセスのみ抽出
- 重複排除してソートした Vec<String> を返す

**3. extract_all_import_specifiers() 変更**

bare import 分岐内で `extract_bare_import_attributes()` を呼び出し:

```rust
// bare import 処理
let attrs = extract_bare_import_attributes(source, &import_name_text);
// attrs が空なら symbols=[] (fallback)、非空なら symbols=attrs
let symbols = attrs;
```

## Test List

### TODO
(none)

### WIP
(none)

### DISCOVERED
- [x] dotted bare import (`import os.path; os.path.join()`) の属性アクセス絞り込み未対応 (fallback=safe) → issue #121
- [x] shadow 変数の known limitation を `docs/known-constraints.md` に記録 → issue #122

### DONE
- [x] PY-ATTR-01: `import httpx; httpx.Client()` → symbols=["Client"] (単一属性)
- [x] PY-ATTR-02: `import httpx; httpx.Client(); httpx.get()` → symbols=["Client", "get"] (複数属性)
- [x] PY-ATTR-03: `import httpx; httpx.Client(); httpx.Client()` → symbols=["Client"] (重複排除)
- [x] PY-ATTR-04: `import httpx` (属性アクセスなし) → symbols=[] (fallback: 全マッチ)
- [x] PY-ATTR-05: `from httpx import Client` → symbols=["Client"] (regression: from-import は変更なし)
- [x] PY-ATTR-06: e2e: `import pkg; pkg.Foo()`, barrel `from .mod import Foo` → mod.py のみ mapped (bar.py は mapped されない)

## Progress Log

### 2026-03-19 09:48 — Cycle doc 作成 (sync-plan)

planファイルから Cycle doc を生成。
Phase 15 で bare import 解決後も symbols=[] による精度低下 (全ファイル一律マッピング) が残存。
テストファイル内の属性アクセス `pkg.Symbol` を抽出して symbols に設定することで精度を改善するサイクルを開始。
test_count: 6 (unit: 5, e2e: 1)
Risk: 30/100 (WARN)

### 2026-03-19 — REFACTOR

- チェックリスト7項目を確認。リファクタリング対象なし（関数サイズ適切、命名一貫、重複なし）
- Verification Gate: tests 197 PASS, clippy 0 errors, fmt OK
- Phase completed

### 2026-03-19 — REVIEW

- Security: PASS (15/100), Correctness: WARN (52/100), Aggregate: WARN (52/100)
- .scm ファイルに known limitation コメント追加 (shadow variable, dotted import)
- DISCOVERED: dotted bare import 属性解析テスト、shadow variable ドキュメント
- Phase completed
