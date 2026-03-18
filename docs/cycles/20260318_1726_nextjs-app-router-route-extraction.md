---
feature: "Phase 10.2a — Next.js App Router route extraction for TypeScript observe"
cycle: "20260318_1726"
phase: REFACTOR
complexity: standard
test_count: 18
risk_level: low
codex_session_id: ""
created: 2026-03-18 17:26
updated: 2026-03-18
---

# Cycle: Phase 10.2a — Next.js App Router route extraction for TypeScript observe

## Scope Definition

### In Scope
- [ ] `crates/lang-typescript/queries/nextjs_route_handler.scm` — exported HTTP handler キャプチャ (新規)
- [ ] `crates/lang-typescript/src/observe.rs` — `extract_nextjs_routes()`, `file_path_to_route_path()` 追加 + テスト
- [ ] `crates/cli/src/main.rs` — TypeScript route_fn で NestJS + Next.js の結果をマージ

### Out of Scope
- Pages Router (`pages/api/**/*.ts`)
- Server Actions (`"use server"`)
- `export { GET } from './handler'` (re-export)
- `export const { GET, POST } = handlers` (destructured export)
- `route.js` (JavaScript)
- middleware.ts

### Files to Change
- `crates/lang-typescript/queries/nextjs_route_handler.scm` (新規)
- `crates/lang-typescript/src/observe.rs` (edit)
- `crates/cli/src/main.rs` (edit)

## Environment

### Scope
- Layer: `crates/lang-typescript`, `crates/cli`
- Plugin: dev-crew:ts-quality (cargo test / clippy / fmt)
- Risk: 25/100 (PASS)
- Runtime: Rust (cargo test)
- Dependencies: tree-sitter (既存、追加依存なし)

### Risk Interview

(BLOCK なし — リスク 25/100)

## Context & Dependencies

### Background

Phase 10 で FastAPI route extraction を完了 (PR #107)。同じ observe route extraction を Next.js App Router に展開する。NestJS はデコレータベース、FastAPI もデコレータベースだったが、Next.js は**ファイルベースルーティング** — ファイルパスがルートパスを定義し、exported 関数名が HTTP method を決める。

NestJS/FastAPI の実装パターンが確立済み。新規要素はファイルパス変換ロジックのみ。

### Design Approach

Next.js App Router パターン: `route.ts` / `route.tsx` ファイルが対象。

ルートはファイルパスから算出:
- `app/api/users/route.ts` → `/api/users`
- `app/api/users/[id]/route.ts` → `/api/users/:id`
- `app/(auth)/api/login/route.ts` → `/api/login` (route group 除去)

HTTP method は exported 関数名 (GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS)。

**file_path_to_route_path 変換規則:**

| セグメント | 変換 |
|-----------|------|
| `[param]` | `:param` |
| `[...slug]` | `:slug*` |
| `[[...slug]]` | `:slug*?` |
| `(group)` | 除去 (route group) |
| `route.ts` / `route.tsx` | 除去 (対象ファイル判定) |

**extract_nextjs_routes アルゴリズム:**
1. `file_path_to_route_path(file_path)` → `None` なら空 Vec (route.ts/tsx でないファイル)
2. source を tree-sitter でパース
3. `nextjs_route_handler.scm` クエリ実行
4. 各マッチの `handler_name` が `NEXTJS_HTTP_METHODS` に含まれるかフィルタ
5. Route を生成: `{ http_method: name, path: route_path, handler_name: name, class_name: "", file, line }`

**CLI dispatch:** NestJS と Next.js は同一ファイルで両方マッチしない (排他的)。TypeScript route_fn で NestJS + Next.js の結果をマージ。

### tree-sitter query 設計

`nextjs_route_handler.scm`:
```scheme
;; Pattern 1: export [async] function GET() {}
(export_statement
  (function_declaration
    name: (identifier) @handler_name)) @exported_handler

;; Pattern 2: export const GET = [async] () => {}
(export_statement
  (lexical_declaration
    (variable_declarator
      name: (identifier) @handler_name
      value: [(arrow_function) (function_expression)]))) @exported_arrow_handler
```

### Reference Documents
- `crates/lang-typescript/src/observe.rs` — NestJS route extraction 実装 (参照元)
- `crates/lang-typescript/queries/decorator.scm` — TypeScript decorator クエリ (参照元)
- `crates/lang-python/src/observe.rs` — FastAPI route extraction 実装 (参照元)
- `crates/cli/src/main.rs` — CLI dispatch (route_fn 登録箇所)
- ROADMAP.md (Phase 10.2: Next.js route extraction)

### Related Issues/PRs
- PR #107: Phase 10 FastAPI route extraction (完了済み、参照元)

## Test List

### TODO

(none — all moved to WIP)

### WIP

**Unit: file_path_to_route_path**
- [x] NX-FP-01: basic app router path — `"app/api/users/route.ts"` → `Some("/api/users")`
- [x] NX-FP-02: src/app prefix — `"src/app/api/users/route.ts"` → `Some("/api/users")`
- [x] NX-FP-03: dynamic segment — `"app/api/users/[id]/route.ts"` → `Some("/api/users/:id")`
- [x] NX-FP-04: route group removed — `"app/(admin)/api/route.ts"` → `Some("/api")`
- [x] NX-FP-05: route.tsx extension — `"app/api/route.tsx"` → `Some("/api")`
- [x] NX-FP-06: non-route file rejected — `"app/api/users/page.ts"` → `None`
- [x] NX-FP-07: catch-all segment — `"app/api/[...slug]/route.ts"` → `Some("/api/:slug*")`
- [x] NX-FP-08: optional catch-all — `"app/api/[[...slug]]/route.ts"` → `Some("/api/:slug*?")`
- [x] NX-FP-09: root route — `"app/route.ts"` → `Some("/")`

**Unit: extract_nextjs_routes**
- [x] NX-RT-01: basic GET handler — `export async function GET() {}` + `"app/api/users/route.ts"` → `[Route { method: "GET", path: "/api/users", handler: "GET" }]`
- [x] NX-RT-02: multiple HTTP methods — export GET + export POST → 2 routes, same path, methods = ["GET", "POST"]
- [x] NX-RT-03: dynamic segment path — export GET + `"app/api/users/[id]/route.ts"` → `path = "/api/users/:id"`
- [x] NX-RT-04: non-route file returns empty — export GET + `"app/api/users/page.ts"` → empty Vec
- [x] NX-RT-05: no HTTP method exports returns empty — `export function helper() {}` + `"app/api/route.ts"` → empty Vec
- [x] NX-RT-06: arrow function export — `export const GET = async () => {}` + `"app/api/route.ts"` → `[Route { method: "GET", path: "/api" }]`
- [x] NX-RT-07: empty source — `""` + `"app/api/route.ts"` → empty Vec
- [x] NX-RT-08: route group in path — export GET + `"app/(auth)/api/login/route.ts"` → `path = "/api/login"`

**Integration: CLI**
- [x] NX-RT-E2E-01: observe with Next.js routes — tempdir with `app/api/users/route.ts` (GET+POST) + `app/api/users/route.test.ts` → `routes_total = 2, routes_covered >= 1`

### WIP
(none)

### DISCOVERED
(none)

### DONE
(none)

## Implementation Notes

### Goal

TypeScript observe に Next.js App Router route extraction を追加し、「どのエンドポイントにテストがあるか？」を静的解析で可視化する。

### Background

Phase 10 で FastAPI route extraction を完了 (PR #107)。NestJS route extraction は `crates/lang-typescript/src/observe.rs` に実装済み。Next.js はファイルベースルーティングで、デコレータベースの NestJS/FastAPI と異なり、ファイルパスがルートパスを定義し、exported 関数名が HTTP method を決める。同一 TypeScript observe モジュール内に NestJS と Next.js の両対応を追加する。

### Design Approach

- `nextjs_route_handler.scm`: export 関数宣言 + export const arrow 関数の 2 パターンで handler name をキャプチャ
- `file_path_to_route_path(path: &str) -> Option<String>`: ファイルパスからルートパスへの変換。`route.ts`/`route.tsx` でない場合は `None`
- `extract_nextjs_routes(source: &str, file_path: &str) -> Vec<Route>`:
  1. `file_path_to_route_path` で None なら早期 return
  2. tree-sitter parse + `nextjs_route_handler.scm` クエリ
  3. handler_name が HTTP_METHODS に含まれるものだけ Route 生成
- CLI dispatch: TypeScript の route_fn を NestJS routes + Next.js routes の concat に変更

## Progress Log

### 2026-03-18 17:26 - INIT
- Cycle doc created
- Plan content transferred from Phase 10.2a plan

### 2026-03-18 17:26 - SYNC-PLAN - Phase completed
- Cycle doc generated from plan
- Test List: 18 items (NX-FP-01~09, NX-RT-01~08, NX-RT-E2E-01)

### 2026-03-18 - RED - Phase completed
- `crates/lang-typescript/queries/nextjs_route_handler.scm` 作成（スタブクエリ）
- `crates/lang-typescript/src/observe.rs` にスタブ関数 `file_path_to_route_path` + `extract_nextjs_routes` + テスト17件追加
- `crates/cli/src/main.rs` に E2E テスト `nx_rt_e2e_01_observe_nextjs_routes_coverage` 追加
- RED 状態確認: lang-typescript lib: 17 FAILED, cli: 1 FAILED, 既存テスト全グリーン

### 2026-03-18 - REFACTOR - Phase completed
- チェックリスト全7項目確認
- 重複コード: NestJS (AST tree traversal) と Next.js (QueryCursor + ファイルパス変換) は処理が根本的に異なるため共通化不要
- 定数・命名・未使用import: 問題なし
- `NEXTJS_HTTP_METHODS` (大文字) と `HTTP_METHODS` (PascalCase) は内容が異なり別定数として適切
- cargo test: 全PASS / cargo clippy: エラー0件 / cargo fmt: 差分なし / self-dogfooding: BLOCK 0件
- 変更なし（コード品質は既に十分）

## Next Steps

1. [Done] INIT
2. [Done] SYNC-PLAN
3. [Done] RED
4. [Done] GREEN
5. [Done] REFACTOR
6. [ ] REVIEW
7. [ ] COMMIT
