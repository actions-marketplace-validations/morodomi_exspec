# exspec Roadmap

## Design Principles

1. **exspec is a static lint + observe tool.** Not a template generator or documentation generator
2. **Solo-dev scope constraint.** Don't pursue 2+ large features in parallel
3. **Ship then iterate.** Don't over-polish before release -- but don't ship broken lint
4. **AI separation.** exspec outputs data; humans/AI decide. exspec itself never calls LLMs

## Now

### v0.4.5: Rust recall + PHP precision

Goal: Rust observe recall を 62.9% → 90% に引き上げる。PHP Str.php FP を解消する。

| Priority | Task | Type | Expected Impact |
|----------|------|------|-----------------|
| P1 | Rust cfg macro multi-hop barrel resolution (nested cfg_net!/cfg_not_wasi!) | observe recall | R 62.9% → ~75% (tokio/tests/ 60 FN の主因) |
| P1 | Rust inline test module improvement (loom/runtime internal tests) | observe recall | R → ~80% (20 FN) |
| P2 | PHP Str.php FP resolution (import frequency or usage pattern analysis) | observe precision | PHP P 96.0% → >=98% |
| P2 | PHP re-audit (50-pair, laravel + symfony) | observe validation | ship criteria 判定 |

**Why**: #179 で R=38.2% → 62.9%, #181 で 62.9% → 71.0%。cfg macro 内の pub items を text fallback で検出 + multi-line pub use を結合。残り 79 FN。

**Rust FN 内訳 (126 FN)**:
- cfg macro multi-hop barrel: 60 FN (tokio/tests/) — `pub use tcp::listener::TcpListener` が cfg_net! 内で解決されない
- loom/runtime inline tests: 20 FN (tokio/src/runtime/tests/) — 内部テストモジュール
- cross-crate import: 20 FN (tokio-stream/tests/) — `tokio_stream::` import
- compile-tests: 13 FN (tests-build/) — production mapping 不適
- other: 13 FN

**PHP Str.php FP 分析**: Str.php は 61/912 テスト (6.7%) にマッピングされる。テストは `Str::random()` 等を utility として使うだけで Str を直接テストしていない。fan-out threshold を下げると Model.php (20%) 等の正当な高頻度 production file も除外される FN リスクがある。

## Backlog

| Priority | Task | Trigger |
|----------|------|---------|
| P2 | Multi-path CLI for observe (B2 cross-package resolution) | 13 FN in NestJS, all B2 |
| P2 | `exspec init` (framework detection + auto-config) | User onboarding friction |
| P2 | #127 Python barrel suppression per-(test, prod) scope | Precision refinement |
| P2 | #92 L1 stem matching for cross-directory layouts | Recall architecture |
| P2 | Rust observe recall: remaining 79 FN (inline tests, cross-crate, compile-tests) | R=71.0% → 90%。v0.4.5 で継続 |
| P2 | #153 Cross-file 1-hop helper delegation | lint BLOCK FP。observe precision 完了後に再評価 (v0.4.3 で defer 確定) |
| P3 | #93 PHP PSR-4 multi-segment namespace resolution | GT audit FP にmulti-segment起因なし。優先度低下 |
| P3 | #132 Phase 19 DISCOVERED (performance, maintainability) | Internal cleanup |
| P3 | #113/#114/#115 Refactoring (cached_query, dedup, trait) | Internal cleanup |

## Completed Recently

### #181: Rust cfg macro export fallback + multi-line pub use (2026-03-25)

Goal: Rust observe recall 改善 (62.9% → 71.0%)。cfg macro 内 pub items の text fallback + multi-line pub use 結合。

| Issue | Task | Status |
|-------|------|--------|
| #181 | `file_exports_any_symbol` text fallback (comment-skip) | DONE (R +8.1pp) |
| #181 | `join_multiline_pub_use` (brace depth tracking) | DONE |
| #181 | `extract_single_re_export_stmt` (semicolon split) | DONE |

**Key insight**: cfg macro 内の `pub struct TcpListener` は tree-sitter の `exported_symbol.scm` query にマッチしない (token_tree が不透明)。text fallback で行単位検索 (コメント行スキップ) を追加。

### #179: Rust L2 self:: prefix + single-segment import fix (2026-03-24)

Goal: Rust observe recall 改善 (38.2% → 62.9%)。2つの L2 バグを修正。

| Issue | Task | Status |
|-------|------|--------|
| #179 | `parse_use_path` single-segment import fix | DONE (R +24.7pp) |
| #179 | `extract_pub_use_re_exports` self:: prefix strip | DONE |
| #179 | `extract_re_exports_from_text` self:: prefix strip | DONE |

**Key insight**: `use tokio::fs` (single-segment) が `parse_use_path` で無視されていた。`pub use self::file::File` が `./self/file` に解決されていた。修正で +67 test files mapped, regression 0。残り 126 FN は cfg macro multi-hop barrel が主因。

### v0.4.4: observe precision improvement (2026-03-24)

Goal: Rust/PHP observe precision improvement。Rust P=100% 達成、PHP P=96.0% (Str.php FP は v0.4.5 へ)。

| Issue | Task | Status |
|-------|------|--------|
| #161 | Rust L0 barrel self-mapping exclusion | DONE (P 76.7% → 92.0%) |
| #162 | Rust L0 mod_item check + L2 export filter | DONE (P 92.0% → 96.0%) |
| #168 | Rust pub-only visibility filter | DONE (P 96.0% → 100%) |
| #129 | Fan-out filter (`[observe] max_fan_out_percent`) | DONE (infrastructure。Str.php には効果なし) |
| #163 | GT re-audit (50-pair, tokio + laravel) | DONE. Rust P=100% PASS, PHP P=96.0% FAIL |

**Key insight**: Fan-out filter (20% threshold) は Str.php (6.7% fan-out) を捕捉できない。utility class の incidental import は fan-out 閾値ではなく import の「目的」(assert 対象か否か) で判定する必要がある。v0.4.5 で別アプローチを検討。

### v0.4.3: lint helper tracing + observe L1 exclusive + GT audit (2026-03-24)

Goal: Same-file helper tracing (all languages), L1 exclusive mode, Rust/PHP GT audit.

| Issue | Task | Status |
|-------|------|--------|
| #150/#151/#152 | Same-file helper tracing (Python/TS/PHP) | DONE. Near-zero BLOCK impact |
| #131 | L1 exclusive mode (`--l1-exclusive`) | DONE |
| #149 | GT audit: Rust P=76.7%, PHP P=90.0% | DONE. Both FAIL → experimental |
| #144 | Relative direct import | CLOSED (already fixed by #146) |
| #129 | L2 fan-out filter | DEFERRED → v0.4.4 (PHP precision fix) |
| #153 | Cross-file helper delegation | DEFERRED → backlog |

**Key insight**: same-file helper tracing は BLOCK FP に効果なし (helper delegation は cross-file class method が主体)。GT audit で Rust/PHP の精度 gap が定量化された。v0.4.4 で targeted fix に集中。

### Phase 23b: Same-file helper tracing for Python/TypeScript/PHP (2026-03-24)

Goal: Port Phase 23a (Rust) same-file helper tracing to remaining 3 languages.

| Issue | Task | Status |
|-------|------|--------|
| #150 | Same-file helper tracing: Python | DONE (PR #155) |
| #151 | Same-file helper tracing: TypeScript | DONE (PR #156) |
| #152 | Same-file helper tracing: PHP | DONE (PR #157) |

**Results**: Dogfooding showed near-zero BLOCK reduction. Helper delegation FP is dominated by cross-file class method calls (`self.method()`, `$this->method()`), not free functions. nestjs was the only project with measurable improvement (-2 BLOCK).

**Decision: #153 deferred to v0.4.4** -- All languages BLOCK FP rate <= 5% after same-file tracing. Cross-file helper delegation is the real solution but requires import resolution infrastructure. Re-evaluate when observe precision work is complete.

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
| P2 | T201 spec-quality (structural advisory mode) | "I want structural spec-quality advisory" (Note: semantic validation is Non-Goal per CONSTITUTION) |
| P2 | GitHub Action marketplace | After Rust/PHP observe stabilize (route extraction shipped in v0.4.0) |
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
- **Success bar**: Ship criteria P>=98%, R>=90% per language. TypeScript (P=100%, R=91%) and Python (P=98.2%, R=96.8%) are stable. Rust (P=100%, R=71.0%) experimental (precision PASS, recall improving). PHP (P=96.0%, R=88.6%) experimental (Str.php incidental import FP)

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

### Helper delegation (Phase 8a-4, Phase 23a-23b)

- User-owned config + runtime guidance. No framework-specific knowledge in exspec core
- Phase 23a: same-file helper tracing for Rust (v0.4.0)
- Phase 23b: same-file tracing ported to Python/TypeScript/PHP (v0.4.3). Dogfooding result: near-zero BLOCK reduction — helper delegation FP is dominated by cross-file class method calls, not free functions
- Cross-file 1-hop (#153): deferred to backlog (v0.4.3 で defer 確定。observe precision 完了後に再評価)。Requires import resolution or class hierarchy tracing — significantly more complex than same-file
- `custom_patterns` remains the primary user-facing escape hatch for helper delegation FP

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
| 23b | Same-file helper tracing: Python (#150), TypeScript (#151), PHP (#152). Near-zero BLOCK impact | v0.4.3 |
| -- | L1 exclusive mode (#131), GT audit (#149): Rust P=76.7%, PHP P=90.0% | v0.4.3 |
| -- | Rust observe precision: L0 barrel exclusion (#161), L0 mod_item + L2 export filter (#162), pub-only visibility (#168) | v0.4.4 |
| -- | Fan-out filter (#129), final re-audit (#163): Rust P=100%, PHP P=96.0% | v0.4.4 |

Detail for completed phases is archived in git history. Key decisions are preserved in "Key Design Decisions" above.
