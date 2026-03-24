# exspec Roadmap

## Design Principles

1. **exspec is a static lint + observe tool.** Not a template generator or documentation generator
2. **Solo-dev scope constraint.** Don't pursue 2+ large features in parallel
3. **Ship then iterate.** Don't over-polish before release -- but don't ship broken lint
4. **AI separation.** exspec outputs data; humans/AI decide. exspec itself never calls LLMs

## Now

### v0.4.4: observe precision improvement

Goal: Rust (P=76.7%) と PHP (P=90.0%) の observe precision を改善する。ship criteria (P>=98%, R>=90%) 達成は re-audit 結果次第であり、v0.4.4 で保証はしない。

| Issue | Task | Type | Expected Impact |
|-------|------|------|-----------------|
| #161 | Rust L1 safeguards: mod.rs除外 + テストファイル存在検証 | observe precision | Rust P 76.7% → ~90% (4 FP 排除) |
| #163 | Rust L1 fix → re-audit (50-pair, tokio) | observe validation | 中間計測。結果次第で #162 の Go/No-Go |
| #162 | Rust L2 re-export chain validation (conditional) | observe precision | Rust P ~90% → ~97% (2 FP 排除。re-audit 後に判断) |
| #129 | PHP L2 fan-out filter (高頻度utility class抑制) | observe precision | PHP P 90.0% → ~97% (Str, Collection 等の除外) |
| #163 | Final re-audit (50-pair, tokio + laravel + symfony) | observe validation | ship criteria 最終判定 |

**Why**: GT audit (#149) で判明した FP パターンは言語固有の構造的問題。Rust は L1 filename matching の曖昧性 (mod.rs, テスト不在ファイル)、PHP は L2 の高頻度utility class (Str, Collection 等) が原因。

**Execution order**: #161 (Rust L1) → #163 中間 re-audit → #162 (Rust L2, conditional) → #129 (PHP fan-out) → #163 final re-audit

**Note: Rust recall (R=36.8%) は v0.4.4 では改善しない。** L1 safeguards は precision 改善のみ。Recall 改善には L2 import tracing の拡充 (deep re-export, wildcard import 等) が必要で、これは別バージョンのスコープ。ship criteria の R>=90% は precision 改善後に再評価。

### Rust observe precision improvement

GT audit FP内訳 (7/30):
- **L1 mod.rs collision** (1): `production_stem()` が mod を返し、テスト側 mod.rs と衝突
- **L1 テストファイル不在** (3): L1 がファイル名一致を仮定するが、実際にはテストファイルが存在しない
- **L1 barrel mismatch** (1): lib.rs/mod.rs がproduction fileとして不適切にマッチ
- **L2 re-export confusion** (2): `pub mod` chain で wrapper と実体を混同

**Phase 1** (L1 safeguards): `map_test_files()` で (a) mod.rs を L1 候補から除外、(b) テストファイルリストとの照合を追加。予想: **4 FP 排除 → 27/30 = P 90.0%**。その後 50-pair re-audit で中間計測。

**Phase 2** (L2 re-export, conditional): Phase 1 の re-audit で P < 98% なら `file_exports_any_symbol()` を L2 マッチ時に検証。予想: **2 FP 排除 → ~29/30 = P ~97%**。P >= 98% なら Phase 2 はスキップ。

### PHP observe precision improvement

GT audit FP内訳 (3/30):
- **高頻度 utility class** (3/3): Str.php (全3件)。Collection, DB 等も同様のパターンが推定される。

Fix: L2 post-processing で fan-out 閾値超えの production file をデフォルトで除外。閾値は configurable (`[observe] max_fan_out_percent`, default 20%)。`--no-fan-out-filter` で無効化可能。

**Fan-out filter は default ON** — opt-in ではなく opt-out。ship criteria はデフォルト動作で判定する。閾値は Laravel (912 tests) + Symfony (2419 tests) の両方で dogfooding 検証し、FN が発生しない値を決定する。

**Note: FN リスク** — fan-out filter は「utility class をテストしているテスト」の mapping も除外する FN リスクがある。CONSTITUTION の quiet 原則 (FP を避ける方向にエラーする) に沿った設計だが、`--no-fan-out-filter` でエスケープ可能であることを README/docs に明記する。

### GT re-audit protocol

v0.4.3 の 30-pair サンプルは信頼区間が広い (P=96.7% でも 95%CI=[83-100%])。v0.4.4 では:
- **サンプルサイズ**: 50-pair (95%CI が ±10% 以内に収まる)
- **対象**: Rust: tokio (workspace)。PHP: laravel + symfony (2プロジェクト)
- **PASS 基準 (言語別)**:
  - PHP: P >= 49/50 (98%) AND R >= 90% → stable 昇格
  - Rust: P >= 49/50 (98%) のみ判定。R=36.8% は v0.4.4 で改善しないため、R の判定は precision 改善後の別バージョンに持ち越す。Rust は v0.4.4 で precision PASS しても experimental のまま (R が足りない)
- **FAIL 時**: 追加 fix を issue 起票し、v0.4.5 で再挑戦

## Backlog

| Priority | Task | Trigger |
|----------|------|---------|
| P2 | Multi-path CLI for observe (B2 cross-package resolution) | 13 FN in NestJS, all B2 |
| P2 | `exspec init` (framework detection + auto-config) | User onboarding friction |
| P2 | #127 Python barrel suppression per-(test, prod) scope | Precision refinement |
| P2 | #92 L1 stem matching for cross-directory layouts | Recall architecture |
| P2 | Rust observe recall improvement (L2 deep re-export, wildcard import) | R=36.8% → 90%。v0.4.4 precision 完了後に着手 |
| P2 | #153 Cross-file 1-hop helper delegation | lint BLOCK FP。observe precision 完了後に再評価 (v0.4.3 で defer 確定) |
| P3 | #93 PHP PSR-4 multi-segment namespace resolution | GT audit FP にmulti-segment起因なし。優先度低下 |
| P3 | #132 Phase 19 DISCOVERED (performance, maintainability) | Internal cleanup |
| P3 | #113/#114/#115 Refactoring (cached_query, dedup, trait) | Internal cleanup |

## Completed Recently

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
- **Success bar**: Ship criteria P>=98%, R>=90% per language. TypeScript (P=100%, R=91%) and Python (P=98.2%, R=96.8%) are stable. Rust (P=76.7%) and PHP (P=90.0%) remain experimental — GT audit #149 completed, both FAIL. Rust FP: filename ambiguity (mod.rs, missing test files). PHP FP: utility class imports (Str.php)

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

Detail for completed phases is archived in git history. Key decisions are preserved in "Key Design Decisions" above.
