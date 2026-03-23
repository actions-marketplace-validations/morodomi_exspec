# exspec Roadmap

## Design Principles

1. **exspec is a static lint + observe tool.** Not a template generator or documentation generator
2. **Solo-dev scope constraint.** Don't pursue 2+ large features in parallel
3. **Ship then iterate.** Don't over-polish before release -- but don't ship broken lint
4. **AI separation.** exspec outputs data; humans/AI decide. exspec itself never calls LLMs

## Now

### v0.4.3: lint BLOCK FP reduction + observe precision

Goal: Reduce BLOCK FP from helper delegation via same-file tracing (3 language ports), improve observe precision, and begin Rust/PHP observe ship criteria audit.

| Issue | Task | Type | Impact |
|-------|------|------|--------|
| #150 | Same-file helper tracing: Python | lint | **DONE** (PR #155). Dogfooding: 0 BLOCK change |
| #151 | Same-file helper tracing: TypeScript | lint | **DONE** (PR #156). Dogfooding: nestjs -2 BLOCK |
| #152 | Same-file helper tracing: PHP | lint | **DONE** (PR #157). Dogfooding: 0 BLOCK change |
| #153 | Cross-file 1-hop helper delegation | lint | **DEFERRED to v0.4.4** (all languages FP <= 5%) |
| #144 | Relative direct import assertion filter bypass | observe | Python observe FN fix |
| #131 | L1 exclusive mode (opt-in: L1 match suppresses L2) | observe precision | httpx L2 FP ~25 elimination |
| #129 | L2 fan-out filter (high-frequency mapping suppression) | observe precision | Cross-project utility FP reduction |
| #149 | Rust/PHP observe formal GT audit | observe ship | Ship criteria validation for experimental languages |

**Why**: v0.4.2 dogfooding confirmed helper delegation remains the dominant BLOCK FP across all languages. Phase 23a (same-file, Rust-only) proved the approach; 23b extends to all 4 languages (#150/#151/#152 same-file, #153 cross-file). #144 is a v0.4.2 residual observe fix. #131/#129 are low-cost precision improvements. #149 is needed to graduate Rust/PHP observe from "experimental" to "stable".

**Execution order**: ~~#150/#151/#152~~ (DONE) → ~~dogfooding~~ (DONE) → ~~#153 Go/No-Go~~ (DEFERRED) → #144/#131/#129 (next) → #149 (last).

**Decision: #153 DEFERRED to v0.4.4** -- same-file tracing dogfooding 結果 (2026-03-24): 全言語で BLOCK FP率 <= 5%。same-file helper は実プロジェクトではほぼ使われておらず、helper delegation FP の大半は `self.method()` / `$this->method()` (cross-file class method) パターン。cross-file は v0.4.4 で再評価。dogfooding 数値: requests 10→10, django 32→37, tokio 247→247, clap 43→43, nestjs 13→11, laravel 222→222, symfony 616→617。

**Decision: #93 deferred to backlog** -- v0.4.2 PHP dogfooding showed laravel at 973/1951 mapped (50%) with 100% precision. Multi-segment namespace impact is marginal. Note: PHP observe の50% recall はproduction file coverageであり、test file coverageは89%。production coverage が低いのは「テストのないファイル」が多い構造的要因であり、#93 のmulti-segment解決では改善しない可能性が高い。GT audit (#149) で実際のrecall gapを特定後に再評価。#93 が再スコープに入る条件: GT audit で multi-segment が原因の FN が 10件以上発見された場合。

**Clarification: #144 vs #146** -- #146 (CLOSED) は absolute direct import の `direct_import_indices` 追記。#144 (OPEN) は relative import ブランチでの同等修正。同じ assertion filter bypass 機能の absolute/relative 非対称性を解消する別 Issue。

**Note: Solo-dev constraint and #150/#151/#152 parallel** -- これら3タスクは Phase 23a (Rust) の mechanical port であり、各タスクの実装コストは小さい (helper_trace.scm作成 + OnceLock統合 + fixture)。"large features" ではなく同一アプローチの言語別適用であるため、並列実行は Solo-dev constraint に抵触しない。

**Decision: #149 scope** -- GT audit の結果は ship criteria (P>=98%, R>=90%) に対する判定のみ。PASS なら stable 昇格、FAIL なら追加実装が必要。#93 の再評価はaudit結果の副産物であり、#149 自体のスコープには含めない。

## Backlog

| Priority | Task | Trigger |
|----------|------|---------|
| P2 | Multi-path CLI for observe (B2 cross-package resolution) | 13 FN in NestJS, all B2 |
| P2 | `exspec init` (framework detection + auto-config) | User onboarding friction |
| P2 | #127 Python barrel suppression per-(test, prod) scope | Precision refinement |
| P2 | #92 L1 stem matching for cross-directory layouts | Recall architecture |
| P2 | #93 PHP PSR-4 multi-segment namespace resolution | GT audit (#149) で再評価 |
| P2 | #153 Cross-file 1-hop helper delegation | v0.4.4 再評価。same-file dogfooding で FP <= 5% |
| P3 | #132 Phase 19 DISCOVERED (performance, maintainability) | Internal cleanup |
| P3 | #113/#114/#115 Refactoring (cached_query, dedup, trait) | Internal cleanup |

## Completed Recently

### v0.4.2: observe recall/precision improvement + Rust/PHP dogfooding (2026-03-23)

Goal: Improve observe recall for stable languages (TS, Python), establish baselines for experimental languages (Rust, PHP).

| Issue | Task | Status |
|-------|------|--------|
| #85 | TS namespace re-export | DONE |
| #119 | Python sub-module direct import resolution | DONE |
| #126 | Python stem-only fallback stem collision guard | DONE |
| #146 | Relative direct import assertion filter bypass | DONE |
| -- | Rust observe dogfooding (tokio +20, clap +2 mapped) | DONE |
| -- | PHP observe dogfooding (laravel +5 mapped) | DONE |

**Why**: TS and Python observe were stable (P>=98%, R>=90%), but #85 and #119 addressed known FN gaps. #126/#146 improved precision and assertion filter coverage. Rust/PHP dogfooding established first baselines (GT audit deferred to #149).

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

## Future

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
- **Success bar**: Ship criteria P>=98%, R>=90% per language. TypeScript (P=100%, R=91%) and Python (P=98.2%, R=96.8%) are stable. Rust and PHP remain experimental (baselines established in v0.4.2, GT audit pending #149)

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

### Helper delegation (Phase 8a-4, Phase 23a, Phase 23b)

- User-owned config + runtime guidance. No framework-specific knowledge in exspec core
- Phase 23a: same-file helper tracing for Rust (auto-detect assertions in called functions within the same file)
- Phase 23b: same-file tracing 3言語ポート (v0.4.3 confirmed) + cross-file 1-hop (conditional, Go/No-Go after same-file dogfooding)
- Dogfooding data: helper delegation is #1 BLOCK FP across all languages (laravel 222, symfony 616, clap 43, requests 10, django 32)

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
| -- | observe recall/precision: #85 TS re-export, #119/#126/#146 Python, Rust/PHP dogfood baselines | v0.4.2 |

Detail for completed phases is archived in git history. Key decisions are preserved in "Key Design Decisions" above.
