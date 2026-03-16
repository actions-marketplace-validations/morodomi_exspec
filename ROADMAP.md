# exspec Roadmap

## Design Principles

1. **exspec is a static lint.** Not a template generator or documentation generator
2. **Solo-dev scope constraint.** Don't pursue 2+ large features in parallel
3. **Ship then iterate.** Don't over-polish before release -- but don't ship broken lint
4. **AI separation.** exspec outputs data; humans/AI decide. exspec itself never calls LLMs

## Completed Phases (Detail)

### Phase 8a: Lint Reliability (DONE)

Goal: Measure and improve FP rates across all severity levels (BLOCK/WARN/INFO), establishing lint reliability as the foundation for all future directions.

**Why 8a first**: Every future direction (observe, GitHub Action, Go support, Note.com articles) depends on users trusting exspec output. A lint that cries wolf at WARN/INFO level trains users to ignore all output. Phase 6 dogfooding proved BLOCK FP rate is manageable, but WARN/INFO has never been validated -- that's half the output users see.

**Phase closure policy**: Items stay open until re-dogfooding confirms improvement and docs are updated. A merged PR is not closure -- validated behavior change is.

#### 8a-1: Known BLOCK FP fixes

Query-fixable BLOCK FPs with known fix strategies. These are addressed first because fix strategies are already determined from Phase 6 dogfooding -- no investigation needed.

| Task | Status |
|------|--------|
| #62 (P0): Python `^assert_` -> `^assert` (pytest 148 FPs) | DONE |
| #63 (P1): PHP `addToAssertionCount()` assertion recognition (Symfony 91 FPs) | DONE |
| #64 (P1): Exclude skip-only tests from T001 (Symfony 91 FPs) | DONE |
| Re-dogfooding: verify improvements on pytest + symfony | TODO (absorbed into 8a-2 survey) |

#### 8a-2: WARN/INFO dogfooding survey

Only BLOCK FPs have been classified so far. WARN/INFO counts exist but content is unverified.

**Why this matters**: Phase 6 dogfooding collected WARN/INFO hit counts (e.g. T101 at 16% in Laravel, T109 at 13% in NestJS) but never sampled individual hits to determine TP/FP. Without this data, we cannot know whether WARN/INFO rules are useful signals or noise. T107 was demoted WARN->INFO based on exactly this kind of analysis; other rules may need the same treatment.

**Execution strategy**: All 7 rules surveyed in a single pass, not split into sub-phases. Execution order is hit-rate descending (T101 -> T102 -> T003 -> T109 -> T105 -> T106 -> T108).

**Why no sub-phase split**: Survey execution is AI-driven (Claude Code reads exspec JSON output + source files to classify TP/FP), so human cognitive load is not the bottleneck. Sub-phase boundaries (8a-2a/b/c) add management overhead without reducing actual work. Low-frequency rules (T106 at 0.8%, T108 at 0.6%) have so few hits that full-count inspection is cheaper and more accurate than spot-check sampling -- skipping them saves almost nothing while leaving blind spots.

**Execution flexibility**: This is a default order, not a hard gate. If early high-frequency survey results reveal an obvious remediation path (query fix, severity change, threshold change), Phase 8a may temporarily switch to 8a-3 before the tail rules are fully reviewed.

**Method**: Sample 20-30 hits per rule per project, classify as TP/FP.

| Rule | Target projects | Concern |
|------|----------------|---------|
| T101 (how-not-what) WARN | Laravel(16%), Symfony(7.5%) | Framework mock-derived FPs? |
| T102 (fixture-sprawl) WARN | NestJS(14.1%), tokio(4.9%) | Threshold or pattern issue? |
| T109 (undescriptive-name) INFO | NestJS(13%) | Naming convention differences? |
| T105 (deterministic) INFO | NestJS(5.3%) | Uninvestigated |
| T003 (giant-test) WARN | fastapi(9.9%) | Is 50-line threshold appropriate? |
| T106 (duplicate-literal) WARN | NestJS(0.8%) | Low frequency, low priority |
| T108 (wait-and-see) WARN | Symfony(0.6%), tokio(2.7%) | Low frequency |

**Deliverable**: FP rates + FP pattern classification per rule -> file issues

#### 8a-3: WARN/INFO FP fixes

Scope determined by 8a-2 results. Expected remediation types:

| Remediation | Example |
|-------------|---------|
| Query improvement | Reduce framework-derived T101 FPs |
| Severity adjustment | Demote high-FP rules: WARN->INFO or INFO->OFF |
| Threshold tuning | T003 max_lines etc. |
| Defer to Phase 8c+ | Issues requiring major rework |

**Results (commit 3670a10)**:

| Rule | Action | Rationale |
|------|--------|-----------|
| T101 | WARN->INFO | High FP rate in framework projects (Laravel 16%, Symfony 7.5%). Framework mock-derived patterns indistinguishable from real how-not-what violations |
| T102 | WARN->INFO | NestJS 14.1% hit rate largely driven by DI setup. Fixture count threshold is structural, not a defect signal |
| T108 | WARN->INFO | 93% FP rate. Most `sleep`/`wait` calls are legitimate async coordination, not test anti-patterns |
| T106 | WARN->OFF | 93% FP rate. Duplicate literal detection too noisy -- DAMP-style repetition is intentional in well-written tests |
| T105 | Keep INFO | No change needed. Hit rate low (0.7-3.7%), signal quality acceptable at INFO |
| T109 | Keep INFO | 50% FP rate at INFO is tolerable. Unlike T106 (93%)->OFF or T108 (93%)->INFO, 50% TP provides useful signal. INFO level delegates judgment to user |
| T003 | Keep WARN/50 lines | fastapi-specific issue (snapshot pattern, 9.9% hit rate). Threshold change affects all projects. fastapi users can override via `.exspec.toml`. Other projects show low hit rates (0.7-3.7%), not worth global adjustment |

#### 8a-4: Helper delegation strategy decision

Remaining BLOCK FPs from helper delegation. Not query-fixable but impacts user experience.

**Why this is a separate decision**: These FPs cannot be fixed by improving tree-sitter queries -- the helpers are project-specific and don't follow detectable naming conventions (e.g. `fnmatch_lines()`, `$assert->has()`). The question is where the responsibility sits: exspec's built-in knowledge, `exspec init` tooling, or user configuration.

| Project | Remaining FPs | Pattern |
|---------|--------------|---------|
| pytest | 415 | fnmatch_lines() |
| Laravel | 222 | AssertableJson, validation, route helpers |
| clap | 218 | assert_data_eq!, assert_matches |
| tokio | 124 | assert_pending!, assert_ready! etc. |

**Options**:
- A: Enhanced `exspec init` (framework detection -> auto-suggest custom_patterns). Keeps exspec language-agnostic; users see the config and can modify it.
- B: Built-in framework patterns (recognize major frameworks by default). Better out-of-box experience, but couples exspec to specific frameworks and requires maintenance as frameworks evolve.
- C: Documentation only (custom_patterns usage guide). Lowest effort, but ~1000 FPs remain for users to configure manually.

**Decision**: User-owned config + runtime guidance. No framework-specific knowledge in exspec core.

**Why**: Helper delegation FPs (~979 across dogfooding projects) are project-specific and cannot be solved by query improvements. The `[assertions] custom_patterns` escape hatch already works. The gap is discoverability, not capability.

**Implementation**:
1. Runtime hint: when T001 BLOCK >= 10 and custom_patterns is empty, exspec outputs actionable guidance with TOML config example
2. Hint is designed for AI agent consumption (Claude Code, Codex etc.) -- structured enough for an agent to auto-generate .exspec.toml from the output
3. `exspec init` with framework detection deferred to Phase 8c

#### Phase 8a exit criteria

- [x] #62/#63/#64 closed
- [x] WARN/INFO FP rates measured for all major projects, recorded in docs/dogfooding-results.md
- [x] Severity adjustments applied where needed
- [x] Query-fixable WARN/INFO FPs filed as issues and addressed
- [x] Helper delegation strategy recorded in ROADMAP

---

## Now

### Phase 8b: `exspec observe` PoC

Goal: Validate whether static AST-only test-to-code mapping can achieve practical precision. 1-2 week timebox.

**Why observe, why now**: No existing tool does static test-to-code mapping -- Microsoft TIA, Launchable, SeaLights all use dynamic instrumentation. If AST-only analysis works, exspec creates a new category with zero competition. The risk is asymmetric: failure costs 1-2 weeks; success opens a product narrative ("AI generates code -> exspec lint checks quality -> exspec observe finds coverage gaps") that no competitor can match. This comes after 8a because observe's credibility depends on lint being trustworthy first.

- **Scope**: 1 language (TypeScript), 1 project (NestJS), route/method test density report
- **Success**: 70%+ of major routes correctly mapped
- **Failure**: <50% precision, or AST limitations make practical mapping impossible

#### Precision evaluation results (Task 6, 2026-03-16)

nestjs/nest (packages/common + packages/core, 130 test files, 166 primary mappings, 59% human-audited):

| Metric | Value |
|--------|-------|
| Precision | 66.3% (134 TP, 68 FP) |
| Recall | 80.7% (134 TP, 32 FN) |
| F1 | 72.8% |

| Stratum | Recall | Notes |
|---------|--------|-------|
| direct_import | 100% (134/134) | Layer 2 import tracing is complete |
| barrel_import | 0% (0/32) | Expected: tree-sitter does not follow index.ts re-exports |

**FP breakdown**: `constants.ts` (26), enum/interface files (35), `index.ts` (7). All are helper/non-SUT imports that observe maps as production files.

**FN breakdown**: All 32 are barrel imports (`import { Foo } from '../index'` or `from '@nestjs/common'`). No direct_import FN exists.

**Decision: Improvement priority order**.

1. **Helper/non-SUT import filtering** (Precision 66%→90%+). Filter `constants.ts`, enum-only files, interface-only files from mappings. Low cost, high impact on F1 (projected 86.4%).
2. **Strict/lenient dual metrics**. Current evaluation ignores secondary_targets entirely. Adding lenient mode (secondary as partial match) separates "SUT finder" vs "dependency extractor" evaluation.
3. **Barrel import expansion** (Recall 80%→100%). Requires tree-sitter query for `export { X } from './y'` + recursive file tracking. High cost, lower F1 impact than filtering (projected 79.7% if done alone).

**Why filtering first**: F1 improvement from precision (66→93%, F1 86.4%) exceeds recall improvement (80→100%, F1 79.7%). User experience: noise (FP) is worse than missing entries (FN) for a mapping tool. Implementation cost is 1/5 of barrel expansion.

**Adjacent opportunity: helper traversal**. Phase 8a-4 discussion (4-AI consensus) identified that `custom_patterns` helper verification (checking if a registered helper actually contains assertions) is interprocedural analysis -- the same problem observe solves. If observe's call-graph infrastructure works, helper verification comes as a byproduct. Constraints agreed upon:
- `custom_patterns` contract stays as text fallback (no semantic change)
- Helper traversal, if implemented, is a separate opt-in setting (e.g. `helper_oracles`)
- Initial scope: same-file, 1-hop, no recursion, known-assertion-only
- Cross-file/cross-module traversal deferred until observe proves feasibility

#### Task 7.5: Helper filter extension (2026-03-16)

Precision 66.3% → 90.3% by filtering non-SUT helper files (constants, enum, interface, exception, test-util, index.ts).

#### Task 8b: Barrel import resolution (2026-03-16)

Same-package barrel import resolution via index.ts re-exports. Named + wildcard re-export support with symbol-aware filtering.

| Metric | Task 6 | Task 7.5 | Task 8b |
|--------|--------|----------|---------|
| Precision | 66.3% | 90.3% | **96.3%** |
| Recall | 80.7% | 78.3% | **93.4%** |
| F1 | 72.8% | 83.8% | **94.8%** |
| FP | 68 | 14 | **6** |
| FN | 32 | 36 | **11** |

After GT audit (4 FP were GT misses), final NestJS scores: **Precision 99.4%, Recall 93.4%, F1 96.3%, FP 1, FN 11**.

**Key learnings**:
1. Named + wildcard symbol-aware barrel resolution is effective. Wildcard file-level expansion without symbol filter is catastrophic (FP 847)
2. Both Precision and Recall improved from baseline -- barrel was the Recall bottleneck, helper filtering was the Precision bottleneck
3. Remaining FN: cross-package barrel (Pattern A, 7/11) and interface/enum filter side-effect (4/11)
4. Remaining FP after GT audit: 1 genuine FP (barrel over-resolution)

#### External validity: typeorm (2026-03-16)

Second repository validation to test generalization beyond NestJS.

| Key | Value |
|-----|-------|
| Repository | typeorm/typeorm |
| Production files | 124 (scanned by observe) |
| Total test mappings | 374 |
| Spot-check sample | 50 random pairs |
| Precision (spot-check) | **100%** (50/50 TP) |

typeorm uses a different structure than NestJS: flat `src/` with single barrel (`src/index.ts`), `test/functional/` and `test/github-issues/` directories, entity schemas in `sample/`. observe correctly maps across these patterns.

**Decision**: observe PoC succeeds. Validated on 2 repositories (NestJS: F1 96.3%, typeorm: Precision 100% spot-check). Static AST-only test-to-code mapping is viable for TypeScript projects with barrel imports.

### Phase 8c: observe MVP (PoC succeeded)

**Decision (2026-03-16)**: observe PoC succeeded (NestJS F1 96.3%, typeorm Precision 100% spot-check). Taking the "observe MVP" branch.

**Why**: The product story shifts from "lint tool" to "test intelligence platform". The differentiator is observe, not language breadth. Go support becomes lower priority.

Phase 8c priorities (ordered):

1. **Failure boundary definition**. Identify where observe breaks: namespace imports, tsconfig path aliases, monorepo cross-package, generated code, decorator-heavy patterns. This defines the applicability scope before shipping.
2. **Product decision metrics**. Define ship criteria: Precision >= 98% required, Recall >= 90% shippable, confidence scoring for uncertain mappings.
3. **Remaining FN resolution** (if cost-effective). Cross-package barrel (7/11 FN) requires tsconfig/node_modules scope. Interface/enum filter refinement (4/11 FN).
4. **observe MVP output**. Markdown/JSON test density report for CI integration. "What is tested, where are the gaps?"
5. **Note.com article**. Write-up of the PoC journey and results.

**Deferred**: Go language support, Tier 3 rules, GitHub Action marketplace. Reconsidered after 8c delivers.

#### 8c-1: Failure boundary definition (DONE)

6 failure boundaries identified, tested, and documented. See [docs/observe-boundaries.md](docs/observe-boundaries.md) for full analysis.

| Boundary | Root Cause | Fixability | Priority for 8c-2 |
|----------|-----------|------------|-------------------|
| B1: Namespace re-export | `re_export.scm` missing pattern | Medium | Low (uncommon) |
| B2: Cross-package barrel | Non-relative path exclusion | Hard | High (7/11 FN) |
| B3: tsconfig path alias | Same as B2 | Hard | High (NestJS monorepo) |
| B4: Interface/enum filter | Intentional but over-broad | Medium | Medium (4/11 FN) |
| B5: Dynamic import | Static-only extraction | Low | Low (rare in tests) |
| B6: scan_root boundary | By design | N/A | N/A |

**Decision**: Generated code detection and decorator factory/chaining were excluded from 8c-1 scope because they did not appear in PoC evaluation results. If future evaluations reveal these as FN sources, they will be added as B7/B8.

**8c-2 scope decision**: B2+B3 (tsconfig path resolution) is the highest-impact fix target. B4 (context-aware filtering) is a secondary target. B1/B5 are low priority.

#### 8c-2: observe MVP ship (DONE)

Ship criteria confirmed:
- Precision 99.4% >= 98% threshold: PASS
- Recall 93.4% >= 90% threshold: PASS

README に observe セクション追加。applicability scope を明示して公開。

**Decision**: 現在の精度で ship する。monorepo 対応 (B2+B3) は 8c-3 で対応。
**Why**: "Ship then iterate" (Design Principle #3)。既に ship criteria 達成済み。monorepo 対応を待つことで全ユーザーへの公開が遅れるリスクの方が大きい。

#### 8c-3: tsconfig path resolution (DONE)

B3 (tsconfig path alias) を解消。`@app/*` 等のパスエイリアスが import tracing で解決可能に。

**Implementation**:
- `crates/lang-typescript/src/tsconfig.rs`: TsconfigPaths, PathAlias, discover_tsconfig, resolve_alias
- tsconfig.json `compilerOptions.paths` + `baseUrl` パース
- `extends` チェーン (相対パスのみ、max 3 levels)
- tsconfig 自動発見 (scan_root から上方探索、max 10 levels)
- `map_test_files_with_imports` に Layer 2b (alias resolution) を追加

**Limitations (documented in observe-boundaries.md)**:
- JSON5 tsconfig 非対応 (標準 JSON のみ、パース失敗時は graceful fallback)
- extends の npm パッケージ参照は無視
- baseUrl 単体解決 (paths なし) は非対応
- B2 (node_modules/workspace) は未解決

**Why B3 only (not B2)**: B3 はプロジェクト内の tsconfig.json を読むだけで完結する。B2 は node_modules 解決が必要で、Yarn/pnpm/npm のバージョン差異・workspace 構成・hoisting 戦略の組み合わせが爆発する。B3 単独でも NestJS monorepo の path alias パターンをカバーできる。

#### 8c-4: context-aware enum/interface filter (DONE)

B4 (interface/enum filter side-effect) を部分解消。`is_non_sut_helper` に `is_known_production` パラメータを追加し、`production_files` に含まれる enum/interface ファイルへのマッピングを許可。

**Implementation**:
- `is_type_definition_file`: suffix check (`.enum`, `.interface`, `.exception`) を独立関数に抽出
- `is_non_sut_helper(file_path, is_known_production)`: `is_known_production=true` のとき suffix filter を bypass
- `collect_matches` の 2 call sites で `canonical_to_idx.contains_key()` により production 判定

**Remaining limitation**: barrel 解決パス (`resolve_barrel_exports_inner`) は `is_known_production=false` を渡す。barrel 経由で enum ファイルに到達するケースでは filter が残存。direct import のみ解消。

**Why partial resolution**: barrel 内部解決は `canonical_to_idx` にアクセスできない。barrel 経由の enum は `collect_matches` で再チェックされるが、barrel が事前にフィルタすると到達しない edge case がある。完全解消は barrel infrastructure のリファクタリングが必要で、cost-benefit が合わない。

## Backlog

| Priority | Task | Trigger |
|----------|------|---------|
| P2 | T001 FP: Python nested test functions (#41) | Deferred from Phase 6 |
| P2 | T001 FP: return-wrapped Chai property (#52) | Deferred from Phase 6 |
| P2 | T201 spec-quality (advisory mode) | "I want semantic quality checks" |
| P3 | T203 AST similarity duplicate detection | "I want duplicate test detection" |
| Rejected | LSP/VSCode extension | Too early -- low user count for UI investment |
| Rejected | Go language (before FP cleanup) | Horizontal expansion with remaining FPs is a reliability risk |

**Decision: #41 backlog retention** -- The main nested-function FP fix landed on 2026-03-12, but this remains listed as issue-family bookkeeping. The roadmap keeps visibility on the surrounding limitation space (e.g. deeply nested helpers, decorator-wrapped tests) rather than treating the broader topic as permanently closed.

**Decision: LSP/VSCode rejection** -- exspec has near-zero external users as of v0.1.2. Building an IDE extension before establishing a user base invests in distribution UX before the core product has proven its value. Reconsiderable after external adoption signals (GitHub stars, issues from non-maintainers).

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
| 6 | Release Hardening: dogfooding 13 projects / 4 langs / ~45k tests, FP fixes (#25-#66), severity review, T110 |
| 7 | OSS Release: LICENSE, README (#26, #27), CHANGELOG, crates.io v0.1.2 publish, GitHub Release |
| 8a | Lint Reliability: BLOCK FP fixes (#62/#63/#64), WARN/INFO survey + severity adjustments, helper delegation strategy |

## Explore: Test Observability (`exspec observe`)

4-AI brainstorm (Grok/Gemini/GPT/Claude, 2026-03-11). Scheduled for Phase 8b PoC.

**Idea**: Route/method-level test density visualization. "What is tested, where are the gaps?" Not a lint (no FAIL), purely descriptive hints.

**OSS gap**: No tool does static test-to-code mapping (all competitors use dynamic instrumentation), automatic test classification (happy/error/validation), or OpenAPI-free route coverage. All three are wide open.

**PoC plan (Phase 8b)**: TypeScript/supertest on NestJS. 1-2 week timebox. Success = 70%+ route mapping precision.

**Narrative**: "AI-generated code -> exspec lint for quality -> exspec observe for gap discovery" completes the story.

**Fallback (if PoC fails)**: Deepen lint with Go support, Tier 3 rules, GitHub Action. Observe idea shelved.

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
