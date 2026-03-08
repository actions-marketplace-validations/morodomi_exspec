# exspec Philosophy

## What exspec Is

exspec is a **static lint for executable-specification-oriented tests**. It checks whether your tests are structurally sound as specifications -- fast, language-agnostic, zero LLM cost.

## What exspec Is NOT

- **Not a semantic validator.** exspec cannot judge whether your test *name* truly describes the behavior, or whether your Property invariant is *mathematically sound*. Those require human or AI review.
- **Not a coverage tool.** exspec does not measure code coverage. Use existing tools (lcov, istanbul, coverage.py) for that.
- **Not an AI reviewer.** exspec generates no LLM calls. It runs in milliseconds on CI with zero API cost.

## The 4 Properties

exspec is built on 4 properties that define what makes a test a good specification. These properties originate from [Test Architecture](https://github.com/morodomi/test-architecture) (Grok/Gemini/GPT/Claude collaborative design, 2026).

| Property | Definition | exspec Rules |
|----------|-----------|-------------|
| **What not How** | Tests describe behavior, not implementation. Test doubles for uncontrollable boundaries (time, randomness, external APIs) are acceptable | T002 (mock-overuse), T101 (how-not-what) |
| **Living Documentation** | Tests are readable as specs without separate docs | T107 (assertion-roulette), T109 (undescriptive-test-name) |
| **Compositional** | Each test verifies one responsibility (judged by failure-reason singularity, not assertion count) | T003 (giant-test), T006 (low-assertion-density), T102 (fixture-sprawl), T108 (wait-and-see) |
| **Single Source of Truth** | One spec, one place (within same level/viewpoint) | T106 (duplicate-literal-assertion) |

Additional rules enforce baseline test quality:

| Category | Rules |
|----------|-------|
| Baseline quality | T001 (assertion-free), T004 (no-parameterized), T005 (pbt-missing), T007 (test-source-ratio), T008 (no-contract) |
| Error coverage | T103 (missing-error-test) |
| Test paradigm | T105 (deterministic-no-metamorphic) |

## What exspec Covers vs What It Doesn't

| Aspect | exspec (static) | Requires human/AI |
|--------|----------------|-------------------|
| Assertion presence | T001 detects | -- |
| Mock count / diversity | T002 detects | Whether mocks test behavior or impl. Mocks for uncontrollable boundaries (time, external APIs) are legitimate |
| Test size | T003 detects (line count as proxy for responsibility count) | Whether test has 1 or N responsibilities |
| Test naming | T109 heuristic | Whether name truly describes behavior |
| Assertion messages | T107 detects absence | Whether messages are meaningful |
| Literal duplication | T106 detects in-function | Cross-file semantic duplication |
| Private field access | T101 detects in assertions | Subtle implementation coupling |
| Sleep/delay usage | T108 detects | Whether wait is justified |
| PBT/Contract import | T005/T008 detect presence | Whether properties/contracts are sound |
| Metamorphic relations | T105 detects absence | Whether relations are mathematically valid |

**Design principle**: exspec catches structural smells that correlate with specification quality violations. It does not attempt semantic analysis. When semantic judgment is needed, exspec defers to human review or future AI-assisted tooling (Tier 3 roadmap).

## Severity Philosophy

| Level | Meaning | Confidence |
|-------|---------|------------|
| BLOCK | Almost certainly a test quality problem | High (near-zero false positives) |
| WARN | Likely a problem, but context-dependent | Medium (heuristic-based) |
| INFO | Worth considering, may be intentional | Lower (opinionated) |

**exspec errs on the side of being quiet.** A false positive at BLOCK level destroys trust. INFO-level noise is tolerable. When in doubt, exspec chooses INFO over WARN.

## Known Limitations

1. **Rust macro arguments** (`assert_eq!`, `assert!`): tree-sitter parses macro arguments as `token_tree` (flat tokens), not structured AST. Private field access and complex expressions inside macros are not detectable.
2. **TypeScript T107**: Jest/Vitest `expect()` has no message argument. T107 is skipped for TypeScript.
3. **Semantic quality**: exspec cannot judge whether a test is a *good specification* -- only whether it has structural characteristics that correlate with good specifications.
4. **Cross-file duplication**: T106 detects in-function literal duplication only. Cross-file semantic duplication (T203) is deferred to future AI-assisted analysis.
