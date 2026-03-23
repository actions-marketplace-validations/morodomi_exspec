# exspec Roadmap

## Design Principles

1. **exspec is a static lint + observe tool.** Not a template generator or documentation generator
2. **Solo-dev scope constraint.** Don't pursue 2+ large features in parallel
3. **Ship then iterate.** Don't over-polish before release -- but don't ship broken lint
4. **AI separation.** exspec outputs data; humans/AI decide. exspec itself never calls LLMs

## Now

### v0.4.2: observe recall improvement + Rust/PHP dogfooding

Goal: Improve observe recall for stable languages (TS, Python), establish ship criteria for experimental languages (Rust, PHP).

| Issue | Task | Type | Impact |
|-------|------|------|--------|
| #85 | TS namespace re-export (.scm addition) | recall | NestJS barrel FN improvement |
| #119 | Python sub-module direct import resolution | recall | httpx 3 unmapped FN recovery |
| #126 | Python stem-only fallback stem collision guard | precision | False match prevention |
| -- | Rust observe formal dogfooding (tokio, clap) | dogfood | P/R measurement, ship criteria |
| -- | PHP observe formal dogfooding (laravel) | dogfood | P/R measurement, ship criteria |

**Why**: TS and Python observe are stable (P>=98%, R>=90%), but #85 and #119 address known FN gaps. Rust and PHP observe have never been formally dogfooded — we need P/R baselines before investing in precision improvements.

### v0.4.3: observe precision improvement + helper delegation

Goal: Reduce observe false positives and lint BLOCK FP via helper delegation.

| Issue | Task | Type | Impact |
|-------|------|------|--------|
| #131 | L1 exclusive mode (opt-in: L1 match suppresses L2) | precision | httpx L2 FP ~25 elimination |
| #129 | L2 fan-out filter (high-frequency mapping suppression) | precision | Cross-project utility FP reduction |
| #93 | PHP PSR-4 multi-segment namespace resolution | recall | Depends on v0.4.2 dogfood results |
| -- | Phase 23b: 1-hop cross-file helper delegation | lint | #1 BLOCK FP source (laravel 222, symfony 616) |

**Why**: v0.4.2 dogfooding will reveal Rust/PHP precision gaps. #131/#129 are language-agnostic precision improvements. Phase 23b is the biggest remaining lint BLOCK reduction opportunity.

## Backlog

| Priority | Task | Trigger |
|----------|------|---------|
| P2 | Multi-path CLI for observe (B2 cross-package resolution) | 13 FN in NestJS, all B2 |
| P2 | `exspec init` (framework detection + auto-config) | User onboarding friction |
| P2 | #127 Python barrel suppression per-(test, prod) scope | Precision refinement |
| P2 | #92 L1 stem matching for cross-directory layouts | Recall architecture |
| P3 | #132 Phase 19 DISCOVERED (performance, maintainability) | Internal cleanup |
| P3 | #113/#114/#115 Refactoring (cached_query, dedup, trait) | Internal cleanup |

## Completed Recently

### Phase 24: Python observe Django tests.py support (2026-03-23)

Goal: Recognize Django's `tests.py` naming convention in Python observe.

**Why**: Django uses `tests.py` (exact name) as the standard test file convention. 1669 Django test files were completely invisible to observe because `is_python_test_file` only matched `test_*.py` and `*_test.py`.

**Approach**: 3 touch points: (1) CLI `is_python_test_file` adds `tests.py`, (2) `test_stem` returns parent directory name for `tests.py`, (3) `production_stem` excludes `tests.py`. Layer 2 import tracing handles actual mapping automatically once `tests.py` enters `test_sources`.

### Phase 23a: Same-file helper delegation tracing for Rust (2026-03-23)

Goal: Detect assertions inside helper functions called from test functions within the same file. Rust-only, same-file scope.

**Why**: Helper delegation is the #1 remaining FP source. Phase 23a implements same-file tracing as a first step toward full 1-hop cross-file resolution.

### Phase 22: Rust custom assert macro auto-detection (2026-03-23)

Goal: Automatically recognize `assert_*!` macro invocations as assertions.

**Why**: tokio had 124 FP and clap had 115 FP from custom assertion macros. These were the dominant FP source for Rust T001.

**Results**: tokio BLOCK 385→257 (-128), clap BLOCK 193→71 (-122). Combined -250 FP across 2 projects.

### Phase 21: Python observe re-dogfood + FP fix (2026-03-22)

**Results**: httpx P=98.2%, R=96.8%, F1=97.5%. Ship criteria PASS (P>=98%, R>=90%).

**Decision**: 1 known FP remains (`_models.py <- test_timeouts.py`, setup-only import). Accepted: P=98.2% meets target.

### Phase 10-20: observe improvements (2026-03-18 -- 2026-03-22)

Route extraction (NestJS, FastAPI, Next.js, Django). TS re-dogfood (P=100%, R=91%). Python observe: L1 fixes, barrel import, assertion filter, helper exclusion. Test helper exclusion.

## Backlog

| Priority | Task | Trigger |
|----------|------|---------|
| P2 | T201 spec-quality (advisory mode) | "I want semantic quality checks" |
| P2 | GitHub Action marketplace | After route extraction ships |
| P3 | T203 AST similarity duplicate detection | "I want duplicate test detection" |
| P3 | Go language support (lint) | After observe multi-language proves demand |
| Rejected | LSP/VSCode extension | Too early -- low user count for UI investment |

**Decision: Go language deferred** -- observe multi-language for existing 4 languages takes priority over adding a 5th language to lint. The product differentiator is observe, not language breadth for lint.

**Decision: LSP/VSCode rejection** -- exspec has near-zero external users as of v0.3.0. Building an IDE extension before establishing a user base invests in distribution UX before the core product has proven its value.

## Non-goals

- **Semantic validator**: exspec does not judge whether test names are meaningful or properties are sound
- **Coverage tool**: use lcov/istanbul/coverage.py for that
- **AI reviewer**: no LLM calls, zero API cost
- **Framework-specific linter**: rules should be language-agnostic where possible

## Key Design Decisions

### observe multi-language strategy (Phase 9)

- **ObserveExtractor trait** -- language-agnostic interface in `crates/core/`, each lang crate implements it
- **Two-layer algorithm is portable** -- Layer 1 (filename convention) + Layer 2 (import tracing) applies to all 4 languages
- **Success bar**: Ship criteria P>=98%, R>=90% per language. TypeScript (Phase 11) and Python (Phase 21) are stable. Rust and PHP remain experimental (no formal dogfooding yet)

### B4 barrel fix rejection (Phase 11)

- **Why not fix**: B4 same-package barrel FN is only 2 pairs. Fixing would likely increase FP. Net precision impact: negative.
- **Root cause of FN**: 13/15 FN are B2 (cross-package), not B4. The real fix is multi-path CLI support.

### observe PoC success (Phase 8b-8c, updated Phase 11)

- TypeScript observe validated on NestJS: P=100%, R=91.0% (separate), P=94.1%, R=95.8% (root)
- Static AST-only test-to-code mapping is viable -- no existing tool does this
- Product narrative: "AI generates code -> exspec lint for quality -> exspec observe for gap discovery"

### T001 FP strategy (Phase 6, 4-AI consensus)

- T001 = "oracle-free" detection, not "assert-free"
- Bounded vocabulary approach (not ML)
- Custom helpers: `.exspec.toml` `[assertions] custom_patterns` as escape hatch

### Severity philosophy (Phase 6)

- BLOCK: near-zero false positives required
- WARN: heuristic-based, context-dependent
- INFO: opinionated, may be intentional
- Phase 8a results: T101/T102/T108 demoted WARN->INFO, T106 disabled (93% FP)

### Helper delegation (Phase 8a-4, Phase 23a)

- User-owned config + runtime guidance. No framework-specific knowledge in exspec core
- Phase 23a: same-file helper tracing for Rust (auto-detect assertions in called functions within the same file)
- Phase 23b (next): 1-hop cross-file tracing for all languages

## Completed Phases

| Phase | Content | Version |
|-------|---------|---------|
| 0-3C | Foundation: SPEC, scaffolding, Python, TypeScript, T001-T008, SARIF | -- |
| 4-4.2 | PHP support (PHPUnit/Pest), FQCN, nested class, Pest arrow | -- |
| 5A-5C | Rust support, Tier 2 rules (T101-T105), T106-T109 | -- |
| 6 | Release Hardening: dogfooding 13 projects / 4 langs / ~45k tests | -- |
| 7 | OSS Release: crates.io publish, GitHub Release | v0.1.2 |
| 8a | Lint Reliability: BLOCK FP fixes, WARN/INFO survey, severity adjustments | v0.1.2 |
| 8b-8c | observe PoC + MVP: TypeScript test-to-code mapping, ship criteria | v0.2.0 |
| 9 | observe multi-language: Python, Rust, PHP. Workspace, barrel, PSR-4 | v0.3.0 |
| 10-17 | Route extraction, TS/Python dogfood, ai-prompt output format | v0.4.0 |
| 20-21 | Python observe: helper exclusion, re-dogfood (P=98.2%, R=96.8%) | v0.4.0 |
| 22 | Rust custom assert macro auto-detection (-250 BLOCK) | v0.4.0 |
| 23a | Same-file helper delegation tracing for Rust | v0.4.0 |
| 24 | Python observe: Django tests.py naming convention | v0.4.1 |
| -- | v0.4.1 cleanup: should_panic exact match, PHP query align, docs, tests | v0.4.1 |

Detail for completed phases is archived in git history. Key decisions are preserved in "Key Design Decisions" above.
