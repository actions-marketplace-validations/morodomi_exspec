---
feature: rust-assert-fn-call
cycle: "#66"
phase: DONE
complexity: small
test_count: 6
risk_level: medium
created: 2026-03-12 13:35
updated: 2026-03-12 15:25
---

# T001 FP -- Rust assert* prefix function call detection

## Scope Definition

### In Scope
- [ ] Decide whether helper delegation should move from `custom_patterns` to built-in Rust detection
- [ ] If built-in is approved, add tree-sitter patterns for simple `assert_*()` function calls
- [ ] If built-in is approved, add tree-sitter patterns for scoped `module::assert_*()` function calls
- [ ] New fixture: helper delegation examples
- [ ] Tests for helper assertion detection
- [ ] Negative tests for overbroad `assert*` matching
- [ ] Update Rust docs to describe the chosen behavior and its limits

### Out of Scope
- `self.assert_*()` method call form (Reason: extremely rare in Rust tests, unlike Python/PHP. Add when needed)

### Files to Change (target: 10 or less)
- `crates/lang-rust/queries/assertion.scm` (edit)
- `tests/fixtures/rust/t001_pass_helper_delegation.rs` (new)
- `tests/fixtures/rust/t001_violation_assertion_named_helper.rs` (new, if negative fixture is needed)
- `crates/lang-rust/src/lib.rs` (edit)
- `docs/SPEC.md` (edit, if Rust built-in T001 behavior changes)
- `docs/languages/rust.md` (edit)
- `README.md` (edit, if known constraints change)
- `docs/known-constraints.md` (edit, if helper delegation policy changes)
- `docs/configuration.md` (edit, if `custom_patterns` guidance changes)

## Environment

### Scope
- Layer: Rust language module + docs
- Plugin: N/A (Rust lang module)
- Risk: medium (behavior change in BLOCK rule T001)

### Runtime
- Language: Rust (stable)

### Dependencies (key packages)
- tree-sitter: 0.24
- tree-sitter-rust: existing

### Risk Interview (BLOCK only)
N/A (PASS)

## Context & Dependencies

### Reference Documents
- [docs/languages/rust.md] - Current Rust language doc
- [docs/SPEC.md] - Rule specifications
- [docs/configuration.md] - Current `custom_patterns` contract
- [docs/dogfooding-results.md] - Dogfooding results
- [README.md] - User-facing known constraints

### Dependent Features
- None

### Related Issues/PRs
- Issue #66: T001 FP: Rust assert* prefix function call detection

## Test List

### TODO
(none)

### WIP
(none)

### DISCOVERED
(none)

### DONE
- [x] T1: Simple assert_* function call detected as assertion
- [x] T2: Scoped assert_* function call detected as assertion
- [x] T3: Mixed macro + function call counted correctly
- [x] T4: Macro assertions still work (regression) — covered by existing `assertion_count_positive_for_pass`
- [x] T5: `assertion_helper()` NOT counted — passes (assertion_count == 0)
- [x] T6: `helper_check()` NOT counted — passes (assertion_count == 0)
- T7: Dropped (built-in path chosen)

## Implementation Notes

### Goal
Resolve Rust T001 false positives around helper delegation, while preserving the current contract that project-local helpers are normally configured via `custom_patterns`.

### Background
clap dogfooding found 103 T001 FPs from `common::assert_matches()` helper delegation. Currently Rust assertion.scm only matches macros (`assert!`, `assert_eq!`, etc.), and project docs describe helper delegation as `custom_patterns` territory. Python/PHP already handle broader assert-prefix helpers, but Rust docs and README currently document a narrower built-in set.

### Design Decision: Built-in path with `^assert_`
Decided at RED phase. Rationale:
- Python/PHP already detect `assert` prefix at query level. Rust was the only outlier.
- `^assert_` (underscore required) prevents overbroad matching (`assertion_helper` excluded).
- In Rust, `assert_*()` function calls (without `!`) are never standard library — always custom helpers.
- False negative risk is effectively zero. T5 negative test validates boundary.
- Docs (rust.md, configuration.md) to be updated in GREEN to maintain contract consistency.

### Review Rebuttal Notes
Counterpoints raised during review follow-up:
- Python/PHP comparison is directionally useful, but not exact parity. Their built-in queries are constrained to framework-shaped method/static-call forms, while Rust here proposes broader free-function matching.
- Therefore, this change should be documented as a Rust policy refinement, not merely "alignment" with existing languages.
- `^assert_` is materially safer than bare `^assert`, but one negative fixture (`assertion_helper`) does not prove false-negative risk is zero for a BLOCK rule.
- The remaining design question is whether exspec treats Rust free-function `assert_*()` as an oracle convention, or whether only known vocabulary such as `assert_matches` should be built-in.

Working conclusion for GREEN:
- Proceed with `^assert_` only if docs explicitly state the new Rust convention.
- If implementation or review uncovers ambiguous helper patterns, narrow to explicit vocabulary instead of keeping generic prefix matching.

### Design Approach
If the built-in path is approved, start with the narrowest viable pattern and verify negatives:
1. Prefer explicit vocabulary such as `assert_matches` before considering generic `^assert`
2. If generic matching is still desired, constrain it to `^assert_` rather than bare `^assert`
3. Cover both simple and scoped calls in `assertion.scm`
4. Add negative fixtures proving `assertion_*` or unrelated helpers are not counted accidentally

Safety argument to validate, not assume:
- Positive: `assert_matches()`-style helpers are common in Rust dogfooding
- Negative risk: broad prefix matching can silently convert real T001 violations into false negatives

### Acceptance Criteria
- The chosen path is explicit in the RED tests
- Public docs remain internally consistent after the change
- At least one negative test guards against overbroad prefix matching
- Existing macro assertions and `#[should_panic]` behavior remain unchanged

## Progress Log

### 2026-03-12 13:35 - KICKOFF
- Cycle doc created
- Scope definition ready

### 2026-03-12 13:46 - RED
- Design decision: Built-in path with `^assert_` pattern (underscore required)
- 5 tests created (T1-T3 positive, T5-T6 negative), T4 covered by existing test
- T7 dropped (built-in path chosen)
- 3 tests failing as expected (T1, T2, T3), 2 negative tests passing (T5, T6)
- Phase completed

### 2026-03-12 14:05 - REVIEW
- Plan review found a contract mismatch with current `custom_patterns` guidance
- Added decision gate for built-in vs config-only handling
- Expanded tests to include negative coverage and doc consistency checks

### 2026-03-12 14:20 - REVIEW FOLLOW-UP
- Recorded rebuttal that Python/PHP are not exact precedent for Rust free-function matching
- Kept `^assert_` as the current candidate, but noted fallback to explicit vocabulary if ambiguity appears
- Clarified that docs must describe this as a Rust-specific policy refinement

### 2026-03-12 14:30 - REVIEW RESOLUTION
- Verified Python/PHP assertion.scm: both use method/static-call forms, not free-function. Rebuttal is factually correct.
- `^assert_` maintained as Rust-specific policy refinement (not "alignment" with Python/PHP)
- Safety argument confirmed: Rust standard assert is macro-only, so `assert_*()` free function = always custom helper
- Remaining theoretical gap ("custom helper != oracle") to be closed by documenting as explicit Rust oracle convention
- GREEN deliverables: assertion.scm patterns + docs/languages/rust.md + docs/SPEC.md policy statement

### 2026-03-12 14:35 - GREEN
- Added Rust assertion query support for free-function `assert_*()` helpers
- Added scoped helper support for `module::assert_*()`
- Confirmed positive and negative RED tests pass in `exspec-lang-rust`
- Phase completed

### 2026-03-12 15:00 - REFACTOR
- 3-agent parallel review (reuse, quality, efficiency): no actionable issues
- Quality reviewer flagged test boilerplate but pattern is established codebase convention
- Applied `cargo fmt` (chain formatting on `.iter().find().unwrap()`)
- Verification Gate: 688 tests PASS, clippy clean, format clean
- Phase completed

### 2026-03-12 15:10 - REVIEW
- Mode: code, Risk: LOW (score 0)
- Security: PASS (3), Correctness: PASS (12). Aggregate: PASS (12)
- Notable: T107 message counting silently skips call_expression nodes (future issue)
- No DISCOVERED items
- Phase completed

### 2026-03-12 15:25 - COMMIT
- Verified `cargo test` and `cargo clippy -- -D warnings` pass
- `cargo run -- --lang rust .` reports expected fixture BLOCKs and is not a useful pre-commit gate in this repository
- Ready to commit Rust helper assertion detection changes for #66
- Phase completed

---

## Next Steps

1. [Done] KICKOFF
2. [Done] RED
3. [Done] GREEN
4. [Done] REFACTOR
5. [Done] REVIEW
6. [Done] COMMIT
