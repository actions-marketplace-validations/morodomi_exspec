---
feature: httpbin-url-match-comment-fix
cycle: 20260326_1700
phase: DONE
complexity: standard
test_count: 9
risk_level: low
codex_session_id: ""
created: 2026-03-26 17:00
updated: 2026-03-26 17:00
---

# has_url_match comment-aware scanning + Flask route parameter normalization

## Scope Definition

### In Scope
- [ ] `has_url_match` に行単位コメントスキップロジックを追加 (Python `#` / PHP,TS,Rust `//`)
- [ ] Flask `extract_routes` で `normalize_django_path` + `:param` → `{param}` 変換を適用

### Out of Scope
- `/* */` ブロックコメント内のアポストロフィ対応 (Reason: 稀なケース。将来必要なら tree-sitter アプローチ (ADR 方式 B) にアップグレード可能)
- `has_url_match` の tree-sitter パイプライン統合 (Reason: パイプライン構造変更コストが高い。ADR で C を選択)

### Files to Change (target: 10 or less)
- `crates/cli/src/main.rs` (edit)
- `crates/lang-python/src/observe.rs` (edit)

## Environment

### Scope
- Layer: Backend
- Plugin: rust, python
- Risk: 10 (PASS)

### Runtime
- Language: Rust (cargo)

### Dependencies (key packages)
- tree-sitter: workspace
- lang-python: workspace

### Risk Interview (BLOCK only)
(該当なし)

## Context & Dependencies

### Reference Documents
- [CONSTITUTION.md](../../CONSTITUTION.md) - Precision >= 98% 要件
- [docs/dogfooding-results.md](../dogfooding-results.md) - httpbin observe 精度 8/81 (9.9%) の記録
- [ROADMAP.md](../../ROADMAP.md) - observe 設計方針・判断理由

### Dependent Features
- Flask route extraction: `crates/lang-python/src/observe.rs`
- Django path normalization: `normalize_django_path` (既存関数)
- has_url_match: `crates/cli/src/main.rs`

### Related Issues/PRs
- Issue #213: httpbin Flask dogfooding — route coverage observe の精度が 9.9% と極端に低い

## Test List

### TODO
- [ ] TC-01: Given: Python ソースに `# can't find` コメント + `'/headers'` 文字列, When: has_url_match(`^/headers$`), Then: true
- [ ] TC-02: Given: PHP ソースに `// don't do this` コメント + `'/csrf-token'` 文字列, When: has_url_match(`^/csrf\-token$`), Then: true
- [ ] TC-03a: Given: ソースに `log("# this isn't a comment /headers")` (ダブルクオート文字列内の #), When: has_url_match(`^/headers$`), Then: true (文字列内の # はスキップしない)
- [ ] TC-03b: Given: ソースに `log('# /headers endpoint')` (シングルクオート文字列内の #), When: has_url_match(`^/headers$`), Then: true
- [ ] TC-04: Given: コメントなしソース (既存テスト), When: has_url_match, Then: 既存テスト全パス (回帰なし)
- [ ] TC-05: Given: Flask route `@app.route('/users/<int:id>')`, When: extract_routes, Then: path = `/users/{id}`
- [ ] TC-06: Given: Flask route `@app.route('/files/<path:filepath>')`, When: extract_routes, Then: path = `/files/{filepath}`
- [ ] TC-07: Given: Flask route `@app.route('/api/<anything>')` (型なし), When: extract_routes, Then: path = `/api/{anything}`
- [ ] TC-08: Given: 既存 Flask route テスト, When: extract_routes, Then: 回帰なし

### WIP
(none)

### DISCOVERED
- [ ] Blueprint + typed Flask param テスト (`/api/users/<int:id>` → `/api/users/{id}`)
- [ ] 複数パラメータルートテスト (`/users/<int:user_id>/posts/<int:post_id>`)
- [ ] normalize_django_path docstring タイポ修正 (`:pk` のスラッシュ欠落)

### DONE
(none)

## Implementation Notes

### Goal
httpbin (Flask) dogfooding で route coverage observe の精度を 8/81 (9.9%) から大幅に改善する。

### Background
原因は2つのバグ/制限:

1. **has_url_match のコメント内アポストロフィバグ**: Python コメント `# can't` の `'` がクオートスキャナを壊し、以降の文字列リテラル (`'/headers'`) が検出不能に。全言語で同じ問題が起きうる。
2. **Flask `<type:param>` 未正規化**: `extract_routes` が `<int:n>` をそのまま出力し、`route_path_to_regex` は `{param}` のみ対応なのでリテラル扱い。

### Design Approach

**has_url_match コメントスキップ (ADR 方式 C)**:

`has_url_match` に行単位の前処理を追加:
- 各行について、Python `#` / PHP,TS,Rust `//` 以降を除去してからクオートスキャン
- 文字列リテラル内の `#` や `//` は誤除去しないよう、クオート外の `#`/`//` のみ除去

方式 B (tree-sitter でコメントノード除外) は理想的だが、has_url_match は CLI 層 (main.rs) にあり、tree-sitter parse tree は各言語の observe.rs にあるためパイプライン構造変更コストが高い。方式 A (正規表現で全体検索) はコメント内 URL で FP リスクがあり CONSTITUTION の P>=98% 要件に抵触する。

**Flask route parameter 正規化 (ADR 方式 Y)**:

`extract_routes` (Flask route extraction) で `normalize_django_path` を path に適用した後、`:param` → `{param}` 変換を追加。Django は既に `normalize_django_path` で正規化済みであり、Flask の `<int:n>` は Django の `<int:pk>` と同じ構文のため既存関数を再利用可能。`route_path_to_regex` は `{param}` のみの単一責務を維持する。

## Verification

```bash
cargo test url_match -- --nocapture
cargo test flask -- --nocapture
cargo test -- --nocapture
cargo clippy -- -D warnings
cargo run --release -- observe --lang python /tmp/exspec-dogfood/httpbin
cargo run -- --lang rust .  # self-dogfooding: BLOCK 0件
```

Evidence: (orchestrate が自動記入)

## Progress Log

### 2026-03-26 17:00 - INIT
- Cycle doc created
- Scope definition ready

### 2026-03-26 - PLAN REVIEW
- design-reviewer: blocking_score 22 (PASS)
- normalize_django_path → `:param` → `{param}` の2ステップ変換を明確化
- TC-03 をクオート種類別に TC-03a/TC-03b に分割
- Verification に self-dogfooding 追加
- Phase completed

### 2026-03-26 - RED/GREEN
- 7件の新規テスト作成 (TC-01〜TC-07)、全て RED → GREEN
- has_url_match: strip_line_comment + token_matches_path_regex 実装
- Flask extract_routes: normalize_django_path + colon→brace 変換
- 全1289テスト PASS
- Phase completed

### 2026-03-26 - REFACTOR
- strip_line_comment の byte_offset 重複計算を統合 (`#` と `//` の条件を1つの if に)
- チェックリスト全7項目確認完了
- Verification Gate PASS: テスト全PASS + clippy 0 + fmt OK + BLOCK 0件
- Phase completed

### 2026-03-26 - REVIEW
- security-reviewer: blocking_score 10 (PASS). ReDoS/パニック/情報漏洩なし
- correctness-reviewer: blocking_score 18 (PASS). ロジック正確性検証済み
- DISCOVERED 3件記録 (Blueprint+typed param テスト、複数パラメータテスト、docstring タイポ)
- Product Verification: httpbin coverage 8/81 → 32/81 (9.9% → 39.5%)
- Phase completed

---

## Next Steps

1. [Done] INIT
2. [Done] PLAN
3. [Next] RED
4. [ ] GREEN
5. [ ] REFACTOR
6. [ ] REVIEW
7. [ ] COMMIT
