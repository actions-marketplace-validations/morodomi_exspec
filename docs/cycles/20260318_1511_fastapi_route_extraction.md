---
feature: "Phase 10 — FastAPI route extraction for Python observe"
cycle: "20260318_1511"
phase: REVIEW-DONE
complexity: medium
test_count: 11
risk_level: low
codex_mode: "no"
codex_session_id: ""
created: 2026-03-18 15:11
updated: 2026-03-18 15:11
---

# Cycle: Phase 10 — FastAPI route extraction for Python observe

## Scope Definition

### In Scope
- [ ] `crates/lang-python/queries/route_decorator.scm` — FastAPI デコレータキャプチャ (新規)
- [ ] `crates/lang-python/src/observe.rs` — `extract_routes()` 関数追加, `Route` struct
- [ ] `crates/cli/src/main.rs` — Python の route_fn を `|_| Vec::new()` から実装に変更

### Out of Scope
- `app.include_router(router, prefix="/api")` のクロスファイル prefix 合成 (Phase 10.2)
- クラスベースビュー (CBV)
- Django / Laravel の route extraction
- 他言語 (TypeScript / PHP / Rust) の route extraction 変更

### Files to Change
- `crates/lang-python/queries/route_decorator.scm` (新規)
- `crates/lang-python/src/observe.rs` (edit)
- `crates/cli/src/main.rs` (edit)

## Environment

### Scope
- Layer: `crates/lang-python`, `crates/cli`
- Plugin: dev-crew:python-quality / dev-crew:ts-quality (cargo test / clippy / fmt)
- Risk: 20/100 (PASS)
- Runtime: Rust (cargo test)
- Dependencies: tree-sitter (既存、追加依存なし)

## Risk Interview

(BLOCK なし — リスク 20/100)

## Context & Dependencies

### 背景
v0.3.0 で observe が 4 言語対応完了。次の差別化は route extraction — 「どのエンドポイントにテストがあるか？」を静的解析で可視化する。TypeScript/NestJS の route extraction は既に実装済み。同パターンを FastAPI に展開する。

Python decorator の tree-sitter 解析のみが新規要素。NestJS 実装との差異は `@app.get(...)` / `@router.get(...)` という属性アクセス形式の decorator と、`APIRouter(prefix=...)` による同一ファイル内 prefix 解決。

### tree-sitter AST 構造

```
decorated_definition
  ├─ decorator
  │  └─ call
  │     ├─ function: attribute (app.get / router.post)
  │     │  ├─ object: identifier (app / router)
  │     │  └─ attribute: identifier (get / post)
  │     └─ arguments
  │        └─ string: "/items/{item_id}"
  └─ function_definition
     └─ name: identifier (read_item)
```

Router prefix 解決 (同一ファイル内):

```
assignment
  ├─ left: identifier (router)
  └─ right: call
     ├─ function: identifier (APIRouter)
     └─ arguments
        └─ keyword_argument
           ├─ name: identifier (prefix)
           └─ value: string ("/items")
```

### 設計方針 (extract_routes アルゴリズム)
1. ファイル全体を parse
2. `APIRouter(prefix=...)` の assignment を収集 → `router_prefixes: HashMap<String, String>`
3. `decorated_definition` を走査
4. decorator が `{var}.{http_method}(path, ...)` 形式か判定
5. HTTP_METHODS (`get`, `post`, `put`, `patch`, `delete`, `head`, `options`) にマッチするか
6. path (第1引数の string literal) を抽出。非リテラルなら `<dynamic>`
7. var が router_prefixes にあれば prefix を結合
8. `Route { http_method, path, handler_name, file }` を返す

### 参照ドキュメント
- `crates/lang-typescript/src/observe.rs` — NestJS route extraction 実装 (参照元)
- `crates/lang-typescript/queries/decorator.scm` — TypeScript decorator クエリ (参照元)
- `crates/lang-python/src/observe.rs` — Python observe 現行実装
- `crates/cli/src/main.rs` — CLI dispatch (route_fn 登録箇所)
- ROADMAP.md (Phase 10: FastAPI route extraction)

## Test List

### TODO
- [x] FA-RT-01: basic @app.get route — `@app.get("/users") def read_users(): ...` から `Route { method: "GET", path: "/users", handler: "read_users" }` を抽出 (RED: FAIL)
- [x] FA-RT-02: multiple HTTP methods — @app.get / @app.post / @app.put / @app.delete を持つソースから 4 route を抽出 (RED: FAIL)
- [x] FA-RT-03: path parameter — `@app.get("/items/{item_id}")` から path = "/items/{item_id}" を抽出 (RED: FAIL)
- [x] FA-RT-04: @router.get with APIRouter prefix — `router = APIRouter(prefix="/items")` + `@router.get("/{item_id}")` から path = "/items/{item_id}" を返す (RED: FAIL)
- [x] FA-RT-05: @router.get without prefix — `router = APIRouter()` + `@router.get("/health")` から path = "/health" を返す (RED: FAIL)
- [x] FA-RT-06: non-route decorator ignored — `@pytest.fixture` / `@staticmethod` で empty Vec を返す (RED: PASS — stub 空Vec が正解)
- [x] FA-RT-07: dynamic path (non-literal) — `@app.get(some_variable)` から path = "<dynamic>" を返す (RED: FAIL)
- [x] FA-RT-08: empty source — "" から empty Vec を返す (RED: PASS — stub 空Vec が正解)
- [x] FA-RT-09: async def handler — `@app.get("/") async def root(): ...` から handler = "root" を返す (async は無視) (RED: FAIL)
- [x] FA-RT-10: multiple decorators on same function — `@app.get("/") @require_auth def root(): ...` から 1 route のみ返す (非route decoratorは無視) (RED: FAIL)
- [x] FA-RT-E2E-01: observe with routes shows route coverage — tempdir に FastAPI app (main.py: 2 routes, test_main.py: main を import) を配置し routes_total = 2, routes_covered >= 1 を確認 (RED: FAIL)

### WIP
(none)

### DISCOVERED
(none)

### DONE
(none)

## Implementation Notes

### Goal
Python observe に FastAPI route extraction を追加し、「どのエンドポイントにテストがあるか？」を静的解析で可視化する。

### Background
NestJS route extraction は `crates/lang-typescript/src/observe.rs` に実装済み。Python の `@app.get(...)` / `@router.get(...)` は NestJS の `@Get(...)` / `@Controller(...)` と同パターン。tree-sitter の Python grammar で `decorated_definition` → `decorator` → `call` → `attribute` として AST が表現される。

### Design Approach
- `route_decorator.scm`: `decorated_definition` を走査し decorator の attribute call + 第1引数 string + function name をキャプチャ
- `Route` struct: `http_method: String`, `path: String`, `handler_name: String`, `file: String`
- `extract_routes(source: &str, file: &str) -> Vec<Route>`:
  1. Pass 1: `APIRouter(prefix=...)` を収集して `router_prefixes` HashMap を構築
  2. Pass 2: decorator を走査、HTTP method マッチ、prefix 結合
- CLI dispatch: `main.rs` の Python route_fn クロージャを `extract_routes` に差し替え

## Progress Log

### 2026-03-18 15:11 - INIT
- Cycle doc created
- Plan content transferred from Phase 10 plan

### 2026-03-18 15:11 - SYNC-PLAN - Phase completed
- Cycle doc generated from plan
- Test List: 11 items (FA-RT-01 ~ FA-RT-10, FA-RT-E2E-01)

### 2026-03-18 15:12 - Plan Review - Phase completed
- Design review verdict: WARN (score: 42)
- Issues addressed: Route struct line field (not needed in ObserveRouteEntry), class_name empty for Python, E2E test pattern (fixture-based)
- All WARN items resolved pre-implementation

### 2026-03-18 - RED Phase completed
- Tests created: FA-RT-01~FA-RT-10 in `crates/lang-python/src/observe.rs` (route_tests module)
- E2E test: FA-RT-E2E-01 in `crates/cli/src/main.rs` (tests module)
- Route stub added: `Route` struct + `extract_routes(_source, _file_path) -> Vec<Route>` returning `Vec::new()`
- RED state verified: 8/10 unit tests FAIL, 1/1 E2E test FAIL (FA-RT-06, FA-RT-08 PASS by coincidence — empty Vec is the correct answer for non-route/empty source)
- self-dogfooding: BLOCK 0件
- `crates/cli/Cargo.toml`: tempfile dev-dependency added

### 2026-03-18 - GREEN Phase completed
- `extract_routes()` fully implemented in `crates/lang-python/src/observe.rs`
- `route_decorator.scm` query created (2 patterns: string literal + dynamic path)
- CLI dispatch updated: Python route_fn closure calls `extract_routes()`
- All 11 tests PASS (10 unit + 1 E2E)

### 2026-03-18 - REFACTOR Phase completed
- Removed unused `_source: &str` parameter from `collect_router_prefixes()`
- Verification Gate: all tests PASS, clippy 0, fmt clean, self-dogfooding BLOCK 0

### 2026-03-18 - REVIEW - Phase completed
- Security review: PASS (score 12) — no blocking issues
- Correctness review: PASS (score 28) — 1 important (LIFO stack order fix applied), 4 optional (deferred)
- Fix applied: collect_router_prefixes DFS now processes children in source order (last-write-wins for same variable)
- DISCOVERED: (none)

## Next Steps

1. [Current] INIT
2. [ ] RED
3. [ ] GREEN
4. [ ] REFACTOR
5. [ ] REVIEW
6. [ ] COMMIT
