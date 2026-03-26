---
feature: query-param-tolerance
cycle: 20260326_1800
phase: DONE
complexity: trivial
test_count: 2
risk_level: low
codex_session_id: ""
created: 2026-03-26 18:00
updated: 2026-03-26 18:00
---

# route_path_to_regex query parameter tolerance

## Scope Definition

### In Scope
- [ ] `route_path_to_regex` の `$` アンカーをクエリパラメータ許容に変更

### Out of Scope
- PHP use alias resolution (S3SongController ≠ LambdaSongController)

### Files to Change (target: 10 or less)
- `crates/cli/src/main.rs` (edit)

## Test List

### TODO
- [ ] TC-01: Given: path `/connect`, source `'/connect?token=abc'`, When: has_url_match, Then: true
- [ ] TC-02: Given: path `/connect`, source `'/connect'` (パラメータなし), When: has_url_match, Then: true (回帰なし)

### DONE
(none)

### DISCOVERED
(none)

## Verification

```bash
cargo test url_match -- --nocapture
cargo test
cargo clippy -- -D warnings
cargo fmt --check
cargo run -- --lang rust .
cargo run --release -- observe --lang php /tmp/exspec-dogfood/koel 2>/dev/null | grep "Routes:"
```

## Progress Log

### 2026-03-26 18:00 - INIT
- Cycle doc created
