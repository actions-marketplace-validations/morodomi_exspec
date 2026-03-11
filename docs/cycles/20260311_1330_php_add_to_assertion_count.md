# Cycle: #63 PHP addToAssertionCount() as assertion

## Phase: DONE

## Context
Symfony dogfooding (17,148 tests) で 91 T001 FPs。原因: `$this->addToAssertionCount(1)` が assertion として認識されない。

## Scope
- `crates/lang-php/queries/assertion.scm` — 1パターン追加
- New fixture: `tests/fixtures/php/t001_pass_add_to_assertion_count.php`
- New test in `crates/lang-php/src/lib.rs`

## Design
`#eq?` 完全一致で `addToAssertionCount` を assertion として認識。object制約なし（既存パターンと同じ方針）。

## Test List
- [x] `$this->addToAssertionCount(1)` → assertion_count >= 1
- [x] 既存PHP T001テスト全PASS（回帰）

## Progress Log

### 2026-03-11 - RED/GREEN/REFACTOR/REVIEW
- Added `addToAssertionCount` pattern to assertion.scm (1 pattern, #eq? exact match)
- New fixture + test. 659 tests all pass.
- clippy clean, fmt clean, self-dogfooding BLOCK 3 (fixtures only)
- Plan review: PASS (5/100)
- Phase completed
