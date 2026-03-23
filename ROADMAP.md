# exspec Roadmap

## Design Principles

1. **exspec is a static lint + observe tool.** Not a template generator or documentation generator
2. **Solo-dev scope constraint.** Don't pursue 2+ large features in parallel
3. **Ship then iterate.** Don't over-polish before release -- but don't ship broken lint
4. **AI separation.** exspec outputs data; humans/AI decide. exspec itself never calls LLMs

## Now

### Phase 22: Rust custom assert macro auto-detection

Goal: Automatically recognize `assert_*!` macro invocations as assertions, eliminating the need for manual `custom_patterns` config in most Rust projects.

**Why**: tokio has 124 FP and clap has 115 FP from custom assertion macros (`assert_pending!`, `assert_ready!`, `assert_data_eq!`, etc.). These are the dominant FP source for Rust T001 (95% of tokio FPs, 53% of clap FPs). Currently users must manually enumerate every custom assert macro in `.exspec.toml` -- a poor onboarding experience.

**Approach**: In Rust assertion detection, match `macro_invocation` nodes whose name starts with `assert` (e.g. `assert_pending!`, `assert_ready_ok!`). tree-sitter can see the macro name even though the body is opaque `token_tree`. This is strictly additive -- existing `custom_patterns` config continues to work for non-`assert_*` patterns.

**Expected impact**: tokio BLOCK 385→~261 (-124), clap BLOCK 193→~78 (-115). Combined -239 FP across 2 projects.

### Next: Phase 23: Helper delegation (1-hop assertion tracing)

Goal: Detect assertions inside project-local helper functions called from test functions. Trace 1 hop from test → helper → assertion.

**Why**: Helper delegation is the #1 remaining FP source across all languages (laravel 222, symfony ~50, fastapi ~15, django 8, clap 103). The `custom_patterns` workaround requires manual config per project. 1-hop tracing would eliminate most helper delegation FPs automatically.

**Approach**: Reuse observe's import/function resolution infrastructure. For each assertion-free test function, check if any called function (same file or 1-hop import) contains assertions. Scope: same-file first, then 1-hop cross-file. opt-in via config to avoid performance regression.

## Completed Recently

### Phase 21: Python observe re-dogfood + FP fix (2026-03-22)

Goal: Formal re-dogfood of Python observe after Phase 13-20 improvements. Measure P/R/F1, fix remaining FPs to meet ship criteria.

**Results**: httpx P=98.2%, R=96.8%, F1=97.5%. Ship criteria PASS (P>=98%, R>=90%).

**Why**: Phase 12 measured P=66.7%, R=6.2%. Phase 13-20 improvements (L1 prefix strip, src/ layout, barrel import, assertion filter, helper exclusion) were estimated to bring P~94%, but only hand-counted. Formal re-dogfood needed for ship criteria validation.

**Code fix**: `is_non_sut_helper()` extended to exclude `mock*.py` (test fixtures), `__version__.py` (metadata), `_types.py` (type definitions) from production files. These cause barrel fan-out FP.

**GT re-audit**: 23 secondary targets added to httpx ground truth. Original GT focused on primary targets with sparse secondary coverage.

**Decision**: 1 known FP remains (`_models.py <- test_timeouts.py`, setup-only import with 0 assertions). Accepted: fixing requires barrel sym-tracking which caused 3 FN regression in testing. P=98.2% meets target.

### Phase 12: Python observe dogfooding + GT (2026-03-19)

Goal: Dogfood Python observe on httpx (30 test files) and Requests (9 test files). Measure P/R/F1 against hand-audited ground truth.

**Results**: httpx P=66.7%, R=6.2%, F1=11.4%. Requests ~0% recall. Both FAIL first-pass criteria (P>=90%, R>=80%).

**Why**: Python observe implementation (Phase 9b) was untested against real projects. Dogfooding revealed fundamental gaps in L1 filename matching and L2 import tracing for Python's common patterns.

**Root Causes**:
1. L1 filename: `_` prefix not stripped (`test_decoders` vs `_decoders.py`) — 13 FN
2. L2 barrel: `import httpx` not resolved through `__init__.py` to production files — 28 FN
3. L1 cross-directory: `tests/client/test_client` vs `httpx/_client` — 10 FN
4. `src/` layout: Requests' `src/requests/` not detected as production root — total miss

**Decision**: Python observe was `[experimental]` at Phase 12. Phase 13-21 fixed all P0/P1 issues. Phase 21 re-dogfood confirmed ship criteria PASS (P=98.2%, R=96.8%). Promoted to stable in v0.4.0.

**Ground truth**: `docs/observe-ground-truth-python-httpx.md`

### Phase 11: TS observe re-dogfood + GT audit (2026-03-18)

Goal: Re-validate NestJS ground truth after Phase 8c/10 changes. Measure actual Precision/Recall.

**Results**: P=100%, R=91.0% (separate packages, after GT audit). 12 FP reclassified as legitimate secondary targets. B4 barrel fix deemed unnecessary — FN are dominantly B2 (cross-package), not B4.

**Why**: Phase 8c fixes improved barrel/import resolution but also introduced peripheral file mappings. Re-dogfood confirmed observe meets ship criteria (P>=98%, R>=90%) when GT accounts for all legitimate secondary targets.

**Decision**: B4 barrel fix NOT implemented. Reason: (1) only 2 FN are same-package B4; (2) fixing would resolve .exception.ts through barrels, likely adding more FP than TP gained; (3) 13/15 FN are B2 cross-package, fixable only with multi-path CLI support.

### Phase 10: observe route extraction

Goal: Extract API route definitions from framework decorators/config. NestJS, FastAPI, Next.js App Router, Django URL conf.

## Next

| Priority | Task | Trigger |
|----------|------|---------|
| P1 | Phase 23: Helper delegation (1-hop assertion tracing) | #1 FP source across all languages |
| P2 | Multi-path CLI for observe (B2 cross-package resolution) | 13 FN in NestJS, all B2 |
| P2 | `exspec init` (framework detection + auto-config) | User onboarding friction |
| P2 | Barrel sym-tracking for setup-only import FP | 1 remaining httpx FP (`_models.py <- test_timeouts.py`) |

## Backlog

| Priority | Task | Trigger |
|----------|------|---------|
| P2 | T201 spec-quality (advisory mode) | "I want semantic quality checks" |
| P2 | GitHub Action marketplace | After route extraction ships |
| P3 | T203 AST similarity duplicate detection | "I want duplicate test detection" |
| P3 | Go language support (lint) | After observe multi-language proves demand |
| Rejected | LSP/VSCode extension | Too early -- low user count for UI investment |

**Decision: Go language deferred** -- observe multi-language for existing 4 languages takes priority over adding a 5th language to lint. The product differentiator is observe, not language breadth for lint.

**Decision: LSP/VSCode rejection** -- exspec has near-zero external users as of v0.3.0. Building an IDE extension before establishing a user base invests in distribution UX before the core product has proven its value. Reconsiderable after external adoption signals (GitHub stars, issues from non-maintainers).

## Non-goals

- **Semantic validator**: exspec does not judge whether test names are meaningful or properties are sound
- **Coverage tool**: use lcov/istanbul/coverage.py for that
- **AI reviewer**: no LLM calls, zero API cost
- **Framework-specific linter**: rules should be language-agnostic where possible

## Key Design Decisions

### observe multi-language strategy (Phase 9)

- **Test-to-code mapping only** -- route extraction is framework-specific and deferred
- **ObserveExtractor trait** -- language-agnostic interface in `crates/core/`, each lang crate implements it
- **Two-layer algorithm is portable** -- Layer 1 (filename convention) + Layer 2 (import tracing) applies to all 4 languages
- **Language order**: Python (strongest conventions) -> Rust (inline test advantage) -> PHP (PSR-4 complexity)
- **Success bar**: Ship criteria P>=98%, R>=90% per language. TypeScript (Phase 11) and Python (Phase 21) are stable. Rust and PHP remain experimental (no formal dogfooding yet)
- **Phase 9 completion**: CONSTITUTION.md Section 8 (Scope Boundaries) の observe 欄を更新する。CONSTITUTION 変更は Human approval 必須 (Section 5)

### B4 barrel fix rejection (Phase 11)

- **Why not fix**: B4 same-package barrel FN is only 2 pairs (http.exception.spec.ts). Fixing barrel resolution to include .exception.ts files would likely increase FP (barrel `export *` would resolve 20+ exception files per barrel). Net precision impact: negative.
- **Root cause of FN**: 13/15 FN are B2 (cross-package), not B4. The real fix is multi-path CLI support.
- **GT audit**: 12 apparent FP were legitimate secondary targets missed in earlier audits. After correction, P=100% (separate mode).

### observe PoC success (Phase 8b-8c, updated Phase 11)

- TypeScript observe validated on NestJS: P=100%, R=91.0% (separate), P=94.1%, R=95.8% (root)
- Static AST-only test-to-code mapping is viable -- no existing tool does this
- Product narrative: "AI generates code -> exspec lint for quality -> exspec observe for gap discovery"
- Ship criteria: Precision >= 98%, Recall >= 90% (TypeScript meets both in separate mode)

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
- Phase 8a results: T101/T102/T108 demoted WARN->INFO, T106 disabled (93% FP)

### Helper delegation (Phase 8a-4)

- User-owned config + runtime guidance. No framework-specific knowledge in exspec core
- T001 runtime hint when custom_patterns is empty and BLOCK count >= 10
- `exspec init` deferred

## Completed Phases

| Phase | Content | Version |
|-------|---------|---------|
| 0-3C | Foundation: SPEC, scaffolding, Python, TypeScript, T001-T008, SARIF | -- |
| 4-4.2 | PHP support (PHPUnit/Pest), FQCN, nested class, Pest arrow | -- |
| 5A-5C | Rust support, Tier 2 rules (T101-T105), T106-T109 | -- |
| 6 | Release Hardening: dogfooding 13 projects / 4 langs / ~45k tests | -- |
| 7 | OSS Release: crates.io publish, GitHub Release | v0.1.2 |
| 8a | Lint Reliability: BLOCK FP fixes, WARN/INFO survey, severity adjustments, helper delegation | v0.1.2 |
| 8b | observe PoC: TypeScript test-to-code mapping (NestJS F1 96.3%, typeorm Precision 100%) | v0.2.0 |
| 8c | observe MVP: failure boundaries, ship criteria, tsconfig path alias, enum/interface filter | v0.2.0 |
| 9 | observe multi-language: Python, Rust, PHP. Workspace aggregation, barrel resolution, PSR-4 | v0.3.0 |

Detail for completed phases is archived in git history. Key decisions are preserved in "Key Design Decisions" above.
