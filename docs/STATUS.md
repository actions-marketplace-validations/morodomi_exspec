# Project Status

## Current Phase

Phase 3A complete. Phase 3B (T004-T008 remaining Tier 1 rules + .exspec.toml parsing) next.

## Progress

| Phase | Status |
|-------|--------|
| 0 - SPEC.md + naming | DONE |
| 1 - Rust + tree-sitter scaffolding | DONE |
| 2 - Python + Tier 1 (T001-T003) | DONE |
| 3A - TypeScript + inline suppression + output polish | DONE |
| 3B - T004-T008 + .exspec.toml parsing | NOT STARTED |
| 3C - SARIF output + metrics | NOT STARTED |
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

## Quality Metrics

| Metric | Current | Target |
|--------|---------|--------|
| Tests | 87 passing | -- |
| Coverage | N/A | 90%+ (min 80%) |
| Clippy errors | 0 | 0 |

## Open Issues

- #1: validate --lang argument (Phase 3B)
- #2: pass_count multi-violation fix (Phase 3B)
- #3: TS suppression describe() limitation docs (Phase 3B)
- #4: fn_node_id rename (Phase 3B)
