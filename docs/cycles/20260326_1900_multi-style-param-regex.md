---
feature: route_path_to_regex multi-style parameter support
cycle: 20260326_1900
phase: RED (verified)
complexity: trivial
test_count: 4
risk_level: low
codex_session_id: ""
created: 2026-03-26 19:00
updated: 2026-03-26 19:30
---

# route_path_to_regex multi-style parameter support

## Scope Definition

### In Scope
- [ ] `route_path_to_regex` に `<dynamic>` → `[^/]+` 変換を追加
- [ ] `route_path_to_regex` に `:<word>` → `[^/]+` 変換を追加
- [ ] 既存の `{param}` → `[^/]+` 変換の回帰確認

### Out of Scope
- NestJS の module/controller tree を辿る route 抽出側の正規化 (Reason: コスト高、今回の責務外)
- TS observe における `:param` → `{param}` 正規化レイヤーの追加 (Reason: route_path_to_regex 側で吸収するほうが変更が小さい)

### Files to Change (target: 10 or less)
- `crates/cli/src/main.rs` (edit)

## Environment

### Scope
- Layer: Backend
- Plugin: rust
- Risk: 5 (PASS)

### Runtime
- Language: Rust (stable)

### Dependencies (key packages)
- tree-sitter: workspace
- regex: workspace

### Risk Interview (BLOCK only)
(該当なし)

## Context & Dependencies

### Reference Documents
- [ROADMAP.md](../../ROADMAP.md) - route coverage 設計判断の経緯
- [docs/languages/typescript.md](../languages/typescript.md) - TypeScript observe の仕様

### Dependent Features
- Flask `<int:id>` 正規化 (#213): route 抽出側で正規化した先例。今回は matcher 側で吸収する方針を採用

### Related Issues/PRs
- Issue #218: cal.com dogfooding — route coverage 3.9% の原因調査と修正

## Test List

### TODO
(none)

### DONE (RED)
- [x] TC-01: Given path `/<dynamic>/:bookingUid`, When route_path_to_regex, Then regex が `/v2/abc123` にマッチする → FAILED (RED confirmed)
- [x] TC-02: Given path `/:id/profile`, When route_path_to_regex, Then regex が `/42/profile` にマッチする → FAILED (RED confirmed)
- [x] TC-03: Given path `/<dynamic>`, When route_path_to_regex, Then regex が `/v2` にマッチする → FAILED (RED confirmed)
- [x] TC-04: Given path `/users/{id}` (既存), When route_path_to_regex, Then 回帰なし — `/users/42` にマッチする → PASSED (regression guard)

### WIP
(none)

### DISCOVERED
(none)

### DONE
(none)

## Implementation Notes

### Goal
cal.com (Next.js/NestJS, 357 routes) の route coverage を 3.9% から大幅改善する。`route_path_to_regex` が `{param}` のみ対応のため、NestJS 標準の `:param` (188 routes) と `<dynamic>` (351 routes) がリテラル扱いされている問題を修正する。

### Background
cal.com dogfooding で route coverage が 14/357 (3.9%) と極端に低いことが判明。原因を調査したところ、`route_path_to_regex` が NestJS の出力するパラメータ構文 (`:param`, `<dynamic>`) を認識できず、URL マッチングが失敗していることが確認された。

### Design Approach
`route_path_to_regex` は「route path → URL match regex」の変換器であり、複数フレームワークのパラメータ構文を吸収するのは合理的な責務。以下の 3 パターンを対応させる:

1. `<dynamic>` → `[^/]+` (NestJS 動的 prefix)
2. `:<word>` → `[^/]+` (NestJS パラメータ `:bookingUid` 等)
3. `{param}` → `[^/]+` (既存、変更なし)

route 抽出側での正規化 (`:param` → `{param}`) は、NestJS の module/controller tree traversal が必要でコスト高。Flask (#213) とは異なり matcher 側での吸収を選択。

## Verification

```bash
cargo test url_match -- --nocapture
cargo test
cargo clippy -- -D warnings
cargo fmt --check
cargo run -- --lang rust .
cargo run --release -- observe --lang typescript --format json /tmp/exspec-dogfood/calcom 2>/dev/null | python3 -c "import json,sys; d=json.load(sys.stdin); r=d['routes']; c=[x for x in r if x['status']=='covered']; print(f'Covered: {len(c)}/{len(r)}')"
```

Evidence: (orchestrate が自動記入)

## Progress Log

### 2026-03-26 19:00 - INIT
- Cycle doc created
- Scope definition ready

---

## Next Steps

1. [Done] INIT <- Current
2. [Done] PLAN
3. [Next] RED
4. [ ] GREEN
5. [ ] REFACTOR
6. [ ] REVIEW
7. [ ] COMMIT
