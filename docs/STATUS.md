# Project Status

## Current Phase

Phase 3C complete (MVP). Phase 4 (dev-crew hook integration) next.

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
| 4 - dev-crew hook integration | NOT STARTED |
| 5 - Tier 2 + PHP/Rust support | NOT STARTED |
| 6 - Tier 3 (AI Prompt generation) | NOT STARTED |
| 7 - OSS release + Note article + MCP Server | NOT STARTED |

## Supported Languages

| Language | Extraction | Assertions | Mocks | Suppression |
|----------|-----------|------------|-------|-------------|
| Python (pytest) | Yes | Yes | Yes | Yes |
| TypeScript (Jest/Vitest) | Yes | Yes | Yes | Yes |

## Active Rules

| ID | Rule | Level | Python | TypeScript |
|----|------|-------|--------|-----------|
| T001 | assertion-free | BLOCK | Yes | Yes |
| T002 | mock-overuse | WARN | Yes | Yes |
| T003 | giant-test | WARN | Yes | Yes |
| T004 | no-parameterized | INFO | Yes | Yes |
| T005 | pbt-missing | INFO | Yes | Yes |
| T006 | low-assertion-density | WARN | Yes | Yes |
| T007 | test-source-ratio | INFO | -- | -- |
| T008 | no-contract | INFO | Yes | Yes |

## Quality Metrics

| Metric | Current | Target |
|--------|---------|--------|
| Tests | 203 passing | -- |
| Coverage | N/A | 90%+ (min 80%) |
| Clippy errors | 0 | 0 |

## Output Formats

| Format | Status |
|--------|--------|
| terminal | Supported |
| json | Supported |
| sarif | Supported (v2.1.0) |
| ai-prompt | Tier 3 (Phase 6) |

## Open Issues (v0.2)

- pass_count semantic: rename to fn_pass or document scope
- hidden directory skip: add test + document
- NaN/Inf guard on ratio config
- Performance: diagnostics single-pass, clone elimination
- TestCase false positive filtering (tree-sitter query)
- OutputFormat enum vs string dispatch sync (issue #5)
- compute_metrics ratio clamp (issue #5)
