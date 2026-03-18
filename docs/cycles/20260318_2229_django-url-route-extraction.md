---
feature: "Phase 10.2b — Django URL conf route extraction for Python observe"
cycle: "20260318_2229"
phase: REFACTOR
complexity: standard
test_count: 22
risk_level: medium
codex_session_id: ""
created: 2026-03-18 22:29
updated: 2026-03-18 23:30
---

# Cycle: Phase 10.2b — Django URL conf route extraction for Python observe

## Scope Definition

### In Scope
- [ ] `crates/lang-python/queries/django_url_pattern.scm` — `path()` / `re_path()` キャプチャ (新規)
- [ ] `crates/lang-python/src/observe.rs` — `extract_django_routes()`, `normalize_django_path()`, `normalize_re_path()` 追加 + テスト
- [ ] `crates/cli/src/main.rs` — Python route_fn で FastAPI + Django の結果をマージ

### Out of Scope
- `include()` クロスファイル URL 合成
- CBV `.as_view()` dict からの HTTP method 推論
- `admin.site.urls`
- Django REST Framework の `Router`

### Files to Change
- `crates/lang-python/queries/django_url_pattern.scm` (新規)
- `crates/lang-python/src/observe.rs` (edit)
- `crates/cli/src/main.rs` (edit)

## Environment

### Scope
- Layer: `crates/lang-python`, `crates/cli`
- Plugin: dev-crew:python-quality (cargo test / clippy / fmt)
- Risk: 30/100 (WARN) — FastAPI 実装パターンが確立済みだが、Django は HTTP method が暗黙的（"ANY"）、かつ re_path の正規表現パースが新規要素
- Runtime: Rust (cargo test)
- Dependencies: tree-sitter (既存、追加依存なし)

### Risk Interview

(BLOCK なし — リスク 30/100)

## Context & Dependencies

### Background

Phase 10 で NestJS (デコレータ)、FastAPI (デコレータ)、Next.js (ファイルベース) のroute extraction を完了。同じ observe route extraction を Django URL conf に展開する。Django は**設定リストベースルーティング** — `urlpatterns` リスト内の `path()` / `re_path()` 関数呼び出しがルートを定義する。

FastAPI実装パターンが確立済み。新規要素は HTTP method が暗黙的（"ANY"固定）である点と、re_path の正規表現からのパラメータ抽出ロジック。

### Design Approach

Django URL conf パターン: `urlpatterns` リスト内の `path()` / `re_path()` 呼び出しが対象。

HTTP method は `"ANY"` 固定（Django URL conf は HTTP method を指定しない）。

**パスパラメータ正規化規則:**

| Django | Normalized | 例 |
|--------|-----------|-----|
| `<int:pk>` | `:pk` | 型付きパラメータ → `:name` |
| `<pk>` | `:pk` | 型なしパラメータ → `:name` |
| `<slug:slug>` | `:slug` | 同上 |
| `(?P<year>[0-9]{4})` | `:year` | re_path named group → `:name` |
| `^` / `$` | 除去 | re_path のアンカー |

**tree-sitter query 設計:**

`django_url_pattern.scm`:
```scheme
;; Pattern 1: path/re_path with attribute handler (views.func)
(call
  function: (identifier) @django.func
  arguments: (argument_list
    (string) @django.path
    (attribute
      attribute: (identifier) @django.handler))
  (#match? @django.func "^(path|re_path)$"))

;; Pattern 2: path/re_path with identifier handler (direct import)
(call
  function: (identifier) @django.func
  arguments: (argument_list
    (string) @django.path
    (identifier) @django.handler)
  (#match? @django.func "^(path|re_path)$"))
```

**extract_django_routes アルゴリズム:**
1. Empty source → 空 Vec
2. tree-sitter でパース
3. `django_url_pattern.scm` クエリ実行
4. 各マッチ: `django.func`, `django.path`, `django.handler` を取得
5. path: `strip_string_quotes()` → `func` が `path` なら `normalize_django_path()`, `re_path` なら `normalize_re_path()`
6. 重複排除: `(path, handler)` キーで HashSet
7. Route 生成: `{ http_method: "ANY", path, handler_name, file }`

**CLI dispatch 変更:**
```rust
// L474-492 の route_fn クロージャ内
let mut routes = exspec_lang_python::observe::extract_routes(&source, prod_file);
routes.extend(exspec_lang_python::observe::extract_django_routes(&source, prod_file));
```

### Reference Documents
- `crates/lang-python/src/observe.rs` — FastAPI route extraction 実装 (参照元)
- `crates/lang-python/queries/` — Python クエリ群 (参照元)
- `crates/lang-typescript/src/observe.rs` — NestJS/Next.js route extraction 実装 (参照元)
- `crates/cli/src/main.rs` — CLI dispatch (route_fn 登録箇所)
- ROADMAP.md (Phase 10.2: Django route extraction)

### Related Issues/PRs
- PR #107: Phase 10 FastAPI route extraction (完了済み、参照元)
- PR #108: Phase 10.2a Next.js App Router route extraction (完了済み、参照元)

## Test List

### TODO
(none — all moved to DONE)

### WIP
(none)

### DISCOVERED
(none)

### DONE

**Unit: normalize_django_path (4テスト)**
- [x] DJ-NP-01: typed parameter
- [x] DJ-NP-02: untyped parameter
- [x] DJ-NP-03: multiple parameters
- [x] DJ-NP-04: no parameters

**Unit: normalize_re_path (4テスト — 3 + DJ-NR-04 from review)**
- [x] DJ-NR-01: single named group
- [x] DJ-NR-02: multiple named groups
- [x] DJ-NR-03: no named groups
- [x] DJ-NR-04: ^ inside character class preserved

**Unit: extract_django_routes (13テスト)**
- [x] DJ-RT-01: basic path() with attribute handler
- [x] DJ-RT-02: path() with direct import handler
- [x] DJ-RT-03: path() with typed parameter
- [x] DJ-RT-04: path() with untyped parameter
- [x] DJ-RT-05: re_path() with named group
- [x] DJ-RT-06: multiple routes
- [x] DJ-RT-07: path() with name kwarg
- [x] DJ-RT-08: empty source
- [x] DJ-RT-09: no path/re_path calls
- [x] DJ-RT-10: deduplication
- [x] DJ-RT-11: include() is ignored
- [x] DJ-RT-12: multiple path parameters
- [x] DJ-RT-13: re_path with multiple named groups

**Integration: CLI (1テスト)**
- [x] DJ-RT-E2E-01: observe with Django routes

## Implementation Notes

### Goal

Python observe に Django URL conf route extraction を追加し、「どのエンドポイントにテストがあるか？」を静的解析で可視化する。

### Background

Phase 10 で NestJS、FastAPI、Next.js App Router の route extraction を完了。Django は設定リストベースルーティングを採用しており、`urlpatterns` リスト内の `path()` / `re_path()` 関数呼び出しがルートを定義する。FastAPI の実装パターン（tree-sitter クエリ + extract 関数）を参照しながら Django 固有の HTTP method 固定（"ANY"）と re_path 正規表現パースを追加実装する。

### Design Approach

- `django_url_pattern.scm`: attribute handler (views.func) と identifier handler (直接import) の 2 パターンで handler name をキャプチャ
- `normalize_django_path(path: &str) -> String`: `<type:name>` / `<name>` を `:name` に正規化
- `normalize_re_path(path: &str) -> String`: `^`/`$` アンカー除去 + `(?P<name>...)` named group を `:name` に正規化
- `extract_django_routes(source: &str, file_path: &str) -> Vec<Route>`:
  1. tree-sitter parse + `django_url_pattern.scm` クエリ
  2. `func` 種別で `path` → `normalize_django_path`, `re_path` → `normalize_re_path`
  3. `http_method = "ANY"` 固定
  4. `(path, handler)` キーで重複排除
- CLI dispatch: Python の route_fn で FastAPI routes + Django routes を concat

## Progress Log

### 2026-03-18 22:29 - INIT
- Cycle doc created
- Plan content transferred from Phase 10.2b plan

### 2026-03-18 22:29 - SYNC-PLAN - Phase completed
- Cycle doc generated from plan
- Test List: 21 items (DJ-NP-01~04, DJ-NR-01~03, DJ-RT-01~13, DJ-RT-E2E-01)

### 2026-03-18 - REFACTOR - Phase completed
- `normalize_django_path` / `normalize_re_path` の `Regex::new()` 呼び出し毎コンパイルを `OnceLock` キャッシュに変更 (`DJANGO_PATH_RE`, `DJANGO_RE_PATH_RE`)
- `"ANY"` リテラルを定数 `HTTP_METHOD_ANY` に抽出 (2箇所)
- `extract_django_routes` のパラメータ名 `file` → `file_path` に統一 (`extract_routes` と揃える)
- cargo test: 全PASS / cargo clippy: 0 errors / cargo fmt: diff なし / self-dogfooding: BLOCK 0

### 2026-03-18 - RED - Phase completed
- 22テスト作成 (DJ-NR-04を追加、合計22件)
- stub関数3件追加: `normalize_django_path`, `normalize_re_path`, `extract_django_routes`
- テスト結果: 18 FAILED / 4 passed (空入力・no-path系スタブでパスするもの) — RED確認
- cargo clippy: 0 errors
- cargo fmt: diff なし
- self-dogfooding (cargo run -- --lang rust .): BLOCK 0件

## Next Steps

1. [Done] INIT
2. [Done] SYNC-PLAN
3. [x] RED
4. [ ] GREEN
5. [x] REFACTOR
6. [ ] REVIEW
7. [ ] COMMIT
