# exspec Roadmap

## Design Principles

1. **exspec is a static lint.** Not a template generator or documentation generator
2. **Solo-dev scope constraint.** Don't pursue 2+ large features in parallel
3. **Ship then iterate.** Don't over-polish before release -- but don't ship broken lint
4. **AI separation.** exspec outputs data; humans/AI decide. exspec itself never calls LLMs

## Now

Phase 6 (Release Hardening) -- in progress.

- Dogfooding complete across 10 projects / 4 languages / ~25,000 tests
- FP fixes done (#25-#51, except #41 and #52 still open)
- Severity review done (T107 WARN->INFO)
- Remaining open issues: FP edge cases (#41, #52, #55-#57), internal cleanup (#29, #30, #36)

## Next

Phase 7 (OSS Release polish)

| Task | Status |
|------|--------|
| LICENSE (MIT) | DONE |
| README: Limitations + gradual adoption guide (#26) | DONE |
| README: CI integration examples (#27) | DONE |
| crates.io publish (`cargo install exspec`) | TODO |
| Note.com article | TODO |

## Later

Phase 8 (Post-Release, feedback-driven)

| Priority | Task | Trigger |
|----------|------|---------|
| P1 | FP fixes + threshold tuning | User feedback |
| P2 | T201 spec-quality (advisory mode) | "I want semantic quality checks" |
| P3 | T203 AST similarity duplicate detection | "I want duplicate test detection" |
| P4 | Test observability (`exspec observe`) | See below |

## Non-goals

- **Semantic validator**: exspec does not judge whether test names are meaningful or properties are sound
- **Coverage tool**: use lcov/istanbul/coverage.py for that
- **AI reviewer**: no LLM calls, zero API cost
- **Framework-specific linter**: rules should be language-agnostic where possible

## Completed Phases

| Phase | Content |
|-------|---------|
| 0 | SPEC.md + naming |
| 1 | Rust + tree-sitter scaffolding |
| 2 | Python + Tier 1 (T001-T003) |
| 3A | TypeScript + inline suppression + output polish |
| 3B | T004-T008 + .exspec.toml parsing |
| 3C | SARIF output + ProjectMetrics (MVP) |
| 4 | PHP support (PHPUnit/Pest) + dev-crew integration |
| 5A | Rust language support (cargo test) |
| 5B | Tier 2 rules T101-T105 (Python + TypeScript) |
| 5C | Tier 2 PHP/Rust expansion (T101-T105, T104 removed) |
| 5.5 | Gap rules T106-T109 |

## Explore: Test Observability (`exspec observe`)

4-AI brainstorm (Grok/Gemini/GPT/Claude, 2026-03-11). Not committed -- exploring feasibility.

**Idea**: Route/method-level test density visualization. "What is tested, where are the gaps?" Not a lint (no FAIL), purely descriptive hints.

**OSS gap**: No tool does static test-to-code mapping (all competitors use dynamic instrumentation), automatic test classification (happy/error/validation), or OpenAPI-free route coverage. All three are wide open.

**Open question**: Can AST-only static analysis achieve useful anchor precision? All existing tools (Microsoft TIA, Launchable, SeaLights) chose dynamic instrumentation for a reason. Need prototype experiment on 1 project before committing.

**Consensus**: Lint FP reduction and crates.io publish come first. Observe is Phase 8+ at earliest. If pursued, start with route view on 1 language (TypeScript/supertest), subcommand architecture (`exspec observe`), never FAIL.

**Alternative worth considering** (Gemini): Instead of observe, deepen lint with Contract/PBT enforcement rules (Tier 3 territory). This leverages exspec's existing moat rather than entering a new domain.

## Key Design Decisions

### T104 removal (Phase 5.5)

"Hardcoded-only" rule penalized DAMP-style tests. Replaced by T106 (duplicate-literal-assertion).

### T001 FP strategy (Phase 6, 4-AI consensus)

- T001 = "oracle-free" detection, not "assert-free"
- Oracle shapes: root (expect/assert) -> modifier chain -> terminal (call or property)
- Bounded vocabulary approach (not ML)
- Custom helpers: `.exspec.toml` `[assertions] custom_patterns` as escape hatch

### Severity philosophy (Phase 6)

- BLOCK: near-zero false positives required
- WARN: heuristic-based, context-dependent
- INFO: opinionated, may be intentional
- T107 demoted WARN->INFO (36-48% FP rate in dogfooding)
