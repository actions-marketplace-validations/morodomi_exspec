# Project Status

## Current Phase

Phase 5C complete (Tier 2 PHP/Rust expansion). T101-T105 implemented for all 4 languages.

## Progress

| Phase | Status |
|-------|--------|
| 0 - SPEC.md + naming | DONE |
| 1 - Rust + tree-sitter scaffolding | DONE |
| 2 - Python + Tier 1 (T001-T003) | DONE |
| 3A - TypeScript + inline suppression + output polish | DONE |
| 3B - T004-T008 + .exspec.toml parsing | DONE |
| 3B cleanup - Discovered items | DONE |
| 3C - SARIF output + metrics | DONE |
| 3 cleanup - NaN guard, TestCase false positive, dead code | DONE |
| 4 - PHP support (PHPUnit/Pest) + dev-crew integration | DONE |
| 4.1 - PHP FQCN attribute + Pest arrow function | DONE |
| 4.2 - Nested class, docblock dedup, FQCN pattern | DONE |
| 5A - Rust language support (cargo test) | DONE |
| 5B - Tier 2 rules (T101-T105) Python + TypeScript | DONE |
| 5C - Tier 2 PHP/Rust expansion (T101-T105) | DONE |
| 6 - Tier 3 (AI Prompt generation) | NOT STARTED |
| 7 - OSS release + Note article + MCP Server | NOT STARTED |

## Supported Languages

| Language | Extraction | Assertions | Mocks | Suppression |
|----------|-----------|------------|-------|-------------|
| Python (pytest) | Yes | Yes | Yes | Yes |
| TypeScript (Jest/Vitest) | Yes | Yes | Yes | Yes |
| PHP (PHPUnit/Pest) | Yes | Yes | Yes | Yes |
| Rust (cargo test) | Yes | Yes | Yes | Yes |

## Active Rules

| ID | Rule | Level | Python | TypeScript | PHP | Rust |
|----|------|-------|--------|-----------|-----|------|
| T001 | assertion-free | BLOCK | Yes | Yes | Yes | Yes |
| T002 | mock-overuse | WARN | Yes | Yes | Yes | Yes |
| T003 | giant-test | WARN | Yes | Yes | Yes | Yes |
| T004 | no-parameterized | INFO | Yes | Yes | Yes | Yes |
| T005 | pbt-missing | INFO | Yes | Yes | N/A | Yes |
| T006 | low-assertion-density | WARN | Yes | Yes | Yes | Yes |
| T007 | test-source-ratio | INFO | -- | -- | -- | -- |
| T008 | no-contract | INFO | Yes | Yes | Yes | N/A |
| T101 | how-not-what | WARN | Yes | Yes | Yes | Yes* |
| T102 | fixture-sprawl | WARN | Yes | Yes | Yes | Yes |
| T103 | missing-error-test | INFO | Yes | Yes | Yes | Yes |
| T104 | hardcoded-only | INFO | Yes | Yes | -- | -- |
| T105 | deterministic-no-metamorphic | INFO | Yes | Yes | Yes | Yes* |

\* Rust: token_tree limitation. Private field access in macros (T101) and relational operators in `assert!()` (T105) are not detectable.

## Quality Metrics

| Metric | Current | Target |
|--------|---------|--------|
| Tests | 466 passing | -- |
| Coverage | N/A | 90%+ (min 80%) |
| Clippy errors | 0 | 0 |

## Output Formats

| Format | Status |
|--------|--------|
| terminal | Supported |
| json | Supported |
| sarif | Supported (v2.1.0) |
| ai-prompt | Tier 3 (Phase 6) |

## Open Issues

- #20 T102 PHP: DataProvider params counted as fixtures (false positive)
- #21 T102 Rust: let-binding count over-counts (threshold calibration)
- #22 T103 Rust: `.is_err()` is a weak error-test proxy
