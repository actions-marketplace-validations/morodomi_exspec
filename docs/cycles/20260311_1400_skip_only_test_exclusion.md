# Cycle: #64 T001 FP — exclude skip-only test functions

## Context
Symfony dogfooding: 91/759 T001 BLOCKs are `$this->markTestSkipped()` only functions.
These are intentionally skipped tests, not assertion-deficient. Exclude from T001.

## Design
Follow `has_wait` pattern: add `has_skip_call: bool` to TestAnalysis, detect via `skip_test.scm` query + `has_any_match()`, exclude from T001 when true.

## Scope
- core: TestAnalysis + T001 rule condition
- PHP: skip_test.scm (markTestSkipped, markTestIncomplete)
- Python: skip_test.scm (pytest.skip, self.skipTest)
- Fixtures + tests

## Test List
1. PHP `markTestSkipped()` only → T001 suppressed
2. PHP `markTestIncomplete()` only → T001 suppressed
3. Python `pytest.skip()` only → T001 suppressed
4. Python `self.skipTest()` only → T001 suppressed
5. PHP skip + logic (no assert) → T001 suppressed (has_skip_call=true)
6. Core TestAnalysis default → has_skip_call == false
7. Core T001 has_skip_call=true → no T001 diagnostic
8. Regression: all existing tests pass

## Phases
- [x] KICKOFF
- [x] RED (8 tests, all fail)
- [x] GREEN (669 tests pass, clippy clean, fmt clean, self-dogfooding OK)
- [x] REFACTOR (no changes needed - follows has_wait pattern exactly)
- [x] REVIEW (security 5/100 PASS, correctness 22/100 PASS)
- [x] COMMIT (42516ad)
