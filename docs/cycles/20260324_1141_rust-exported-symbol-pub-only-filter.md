---
feature: "#168 Rust exported_symbol.scm pub-only visibility filter"
cycle: 20260324_1141
phase: RED
complexity: standard
test_count: 4
risk_level: low
codex_session_id: ""
created: 2026-03-24 11:41
updated: 2026-03-24 11:41
---

# #168 Rust exported_symbol.scm pub-only visibility filter

## Summary

Final re-audit (#163) で P=96.0% (48/50)。残存 2 FP は driver.rs の `pub(crate) struct Handle` が `exported_symbol.scm` にマッチすること。tree-sitter の `visibility_modifier` は `pub` も `pub(crate)` も含む。`exported_symbol.scm` に `@vis` キャプチャを追加し、`file_exports_any_symbol()` で `"pub"` のみを対象にフィルタリングする。

## Scope Definition

### In Scope

- `crates/lang-rust/queries/exported_symbol.scm`: `(visibility_modifier)` → `(visibility_modifier) @vis` (7箇所)
- `crates/lang-rust/src/observe.rs` の `file_exports_any_symbol()` (line 341-376): `@vis` テキスト == "pub" フィルタ追加

### Out of Scope

- 他言語の exported_symbol クエリへの変更なし
- `file_exports_any_symbol()` 以外の呼び出し箇所の変更なし

### Files to Change

| File | Change |
|------|--------|
| `crates/lang-rust/queries/exported_symbol.scm` | `(visibility_modifier)` → `(visibility_modifier) @vis` (7箇所) |
| `crates/lang-rust/src/observe.rs` | `file_exports_any_symbol()` に `@vis` テキスト == "pub" フィルタ追加 |

## Environment

- Layer: lang-rust/observe
- Plugin: Rust
- Risk: low
- Runtime: Rust (cargo test)
- Dependencies: tree-sitter, lang-rust crate

## Risk Interview

(low risk — no BLOCK interview required)

## Context & Dependencies

### Upstream References

- CONSTITUTION.md Section 7: quiet 原則 (FP を避ける方向)
- ROADMAP.md: observe precision improvement (GO 判定済み)
- Re-audit data: Issue #163 final re-audit P=96.0% (48/50)

### Related Issues/PRs

- Issue #168: Rust exported_symbol.scm pub-only visibility filter

## Implementation Notes

### Goal

残存 FP 2件を排除。P=96.0% (48/50) → P=100% (50/50)。

### Background

Final re-audit (#163) で残存 2 FP: driver.rs の `pub(crate) struct Handle` が `exported_symbol.scm` にマッチすること。tree-sitter の `visibility_modifier` ノードは `pub`、`pub(crate)`、`pub(super)` をすべて含む。`file_exports_any_symbol()` は現在 visibility を区別しないため、`pub(crate)` アイテムも "exported" として扱われてしまう。

### Design Approach

#### Step 1: exported_symbol.scm に @vis キャプチャ追加

全7パターンの `(visibility_modifier)` を `(visibility_modifier) @vis` に変更:

```scheme
(function_item
  (visibility_modifier) @vis
  name: (identifier) @symbol_name)
```

#### Step 2: file_exports_any_symbol() でテキストフィルタ

`crates/lang-rust/src/observe.rs` の `file_exports_any_symbol()` (line 341-376) で、`@vis` キャプチャのテキストが `"pub"` の場合のみマッチとして扱う:

```rust
let vis_idx = query.capture_index_for_name("vis");
// ... in match loop:
for cap in m.captures {
    if cap.index == symbol_idx {
        // Check visibility: only "pub" (not "pub(crate)", "pub(super)")
        let is_pub_only = m.captures.iter().any(|c| {
            vis_idx == Some(c.index)
                && c.node.utf8_text(source_bytes).unwrap_or("") == "pub"
        });
        if !is_pub_only { continue; }
        let name = cap.node.utf8_text(source_bytes).unwrap_or("");
        if symbols.iter().any(|s| s == name) { return true; }
    }
}
```

## Verification

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
cargo run -- --lang rust .
```

Evidence: (orchestrate が自動記入)

## Test List

### TODO

(none)

### WIP

(none)

### DISCOVERED

(none)

### DONE

- [x] TC-01: **Given** file with `pub fn create_user()`, **When** file_exports_any_symbol(["create_user"]), **Then** true (regression) — PASS (RED verified)
- [x] TC-02: **Given** file with `pub(crate) struct Handle`, **When** file_exports_any_symbol(["Handle"]), **Then** false — FAIL as expected (RED verified)
- [x] TC-03: **Given** file with `pub(super) fn helper()`, **When** file_exports_any_symbol(["helper"]), **Then** false — FAIL as expected (RED verified)
- [x] TC-04: **Given** file with `pub struct User` + `pub(crate) struct Inner`, **When** file_exports_any_symbol(["User"]), **Then** true (mixed visibility) — PASS (RED verified)

## Progress Log

- 2026-03-24 11:41: Cycle doc 作成 (sync-plan)
- 2026-03-24 11:41: RED phase 完了。TC-01/TC-04 PASS (regression)、TC-02/TC-03 FAIL as expected。RED state verified。
