# Project Status

## Current Phase

v0.4.5-dev. Rust precision FAIL: stratified GT audit reveals P=23.3% (barrel import fan-out). PHP precision FAIL (P=96.0%).

observe TypeScript: P=100%, R=91% (stable). Python: P=98.2%, R=96.8% (stable). Rust: **P=23.3%** (stratified GT), R=33.3% (experimental, P FAIL R FAIL. barrel import fan-out is blocking issue). PHP: P~100%, R=85.1% (experimental, P PASS R FAIL. fan-out+name-match filter). Lint: 17 active rules, 4 languages, same-file helper tracing enabled. Default output: ai-prompt.

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
| 5.5 - Gap rules (T106-T109) + T104 removal | DONE |
| 6 - Release Hardening (FP fixes, dogfooding) | DONE |
| 7 - OSS release (crates.io v0.1.2) | DONE |
| 8a-1 - BLOCK FP fixes (#62/#63/#64) | **DONE** |
| 8a-2 - WARN/INFO dogfooding survey | **DONE** |
| 8a-3 - Severity adjustments (T101/T102/T108->INFO, T106->OFF) | **DONE** |
| 8a-4 - Helper delegation strategy | **DONE** |
| 8b - observe PoC (static test-to-code mapping) | **DONE** |
| 8c-1 - observe failure boundaries | **DONE** |
| 8c-2 - observe MVP ship (README, ship criteria) | **DONE** |
| 8c-3 - tsconfig path resolution | **DONE** |
| 8c-4 - context-aware enum/interface filter | **DONE** |
| 10 - Route extraction (NestJS, FastAPI, Next.js, Django) | **DONE** |
| 11 - TS observe re-dogfood + GT audit | **DONE** |
| 12 - Python observe dogfooding + GT | **DONE** |
| 17 - ai-prompt output format (default) | **DONE** |
| 20 - Python observe test helper exclusion | **DONE** |
| 21 - Python observe re-dogfood + FP fix | **DONE** |
| #179 - Rust L2 self:: prefix + single-segment import fix | **DONE** |
| #181 - Rust cfg macro export fallback + multi-line pub use | **DONE** |
| GT re-audit - Rust observe stratified 53-file audit | **DONE** |

### Rust Observe Stratified GT Re-audit (2026-03-25)

| Metric | Previous (random 50-pair) | Stratified GT (53-file) | Target |
|--------|--------------------------|------------------------|--------|
| Precision | 100% | **23.3%** | >= 98% |
| Recall | 71.0% | **33.3%** (GT sample) | >= 90% |

**Barrel import fan-out is the blocking precision problem.** 2 files (io_driver.rs, fs_write.rs) generate 62/66 FP due to `use tokio::runtime::Builder` / `use tokio::fs` resolving to ALL module exports. Excluding these outliers: P=82.6%.

**Previous P=100% was misleading**: random pair sampling avoided barrel FP by chance.

**Fan-out filter: 92.3% accurate** (12/13 correctly filtered in sample).

**FN root causes**: barrel import (10), no use statement (4), inline src/ tests (5), cross-crate (4), macro body imports (1).

**Next**: Fix barrel import precision -- when barrel resolves to many files, prefer L1 filename match to select specific target.

### #181 Rust Observe Recall Improvement (2026-03-25)

| Metric | Post-#179 | Post-#181 | Target |
|--------|-----------|-----------|--------|
| Precision | 100% | 100% | >= 98% |
| Recall (test file) | **62.9%** (171/272) | **71.0%** (193/272) | >= 90% |
| Regression | 0 | 0 | 0 |

**+22 test files mapped.** `file_exports_any_symbol` text fallback for cfg macro pub items + `join_multiline_pub_use` with brace depth tracking.

**Remaining 79 FN analysis (fan-out filter impact):**
- ~25 FN: trait import (AsyncReadExt etc.) correctly filtered by fan-out name-match (not true FN)
- ~15 FN: true FN in tokio/tests/ (needs investigation)
- 19 FN: tokio/src/ inline tests (loom/runtime internal, mapping困難)
- 20 FN: tokio-stream/ cross-crate import (別crate `tokio_stream::`)
- 13 FN: tests-build/ compile-tests (production mapping不適)
- Without fan-out filter: R=no-filter ~80%+ (async_read_ext.rs alone maps 32 tests)

**Next**: GT re-audit (50-pair) to measure true precision/recall with fan-out filter.

### #179 Rust Observe Recall Improvement (2026-03-24)

| Metric | Pre-#179 | Post-#179 | Target |
|--------|----------|-----------|--------|
| Precision | 100% | 100% | >= 98% |
| Recall (test file) | **38.2%** (104/272) | **62.9%** (171/272) | >= 90% |
| Regression | - | 0 | 0 |

**+67 test files mapped.** Two L2 bugs fixed: `parse_use_path` single-segment drop, `extract_pub_use_re_exports` self:: prefix. Remaining 126 FN: cfg macro multi-hop barrel (60), loom inline tests (20), cross-crate tokio-stream (20), compile-tests (13), other (13).

### Phase 21 Python Observe Re-dogfood Results (2026-03-22)

| Metric | httpx | Requests (spot-check) | Target |
|--------|-------|-----------------------|--------|
| Precision (pair) | **98.2%** (55/56) | ~100% | >= 98% |
| Recall (test file) | **96.8%** (30/31) | 100% | >= 90% |
| F1 | **97.5%** | -- | -- |

**Ship criteria: PASS** (both P>=98% and R>=90%).

Code fix: `is_non_sut_helper()` extended to exclude `mock*.py`, `__version__.py`, `_types.py` from production files. GT re-audited: 23 secondary targets added. 1 known FP remaining (`_models.py <- test_timeouts.py`, no assertion on model).

### Phase 12 Python Observe Dogfooding Results (2026-03-19)

| Project | Precision | Recall | F1 | Status |
|---------|-----------|--------|----|--------|
| httpx (30 test files) | 66.7% | 6.2% | 11.4% | FAIL |
| Requests (9 test files) | N/A | ~0% | N/A | FAIL |

**Both below first-pass criteria (P>=90%, R>=80%).** Root causes: L1 `_` prefix mismatch, L2 barrel import unresolved, `src/` layout not detected. Improvement plan filed in dogfooding-results.md.

### Phase 11 Re-dogfood Results (2026-03-18)

NestJS ground truth re-validated after Phase 8c/10 changes. 12 FP reclassified as legitimate secondary targets.

| Scope | Precision | Recall | F1 | FP | FN |
|-------|-----------|--------|----|----|-----|
| Separate packages (common + core) | 100.0% | 91.0% | 95.2% | 0 | 15 |
| Root (full monorepo) | 94.1% | 95.8% | 94.9% | 10 | 7 |
| typeorm (50-pair spot-check) | 100% | -- | -- | 0 | -- |

Remaining FN (separate): B2 cross-package (8), B2+B4 cross-package enum/interface (5), B4 same-package barrel (2).
Root mode resolves most B2 FN but introduces FP from peripheral imports not yet in GT.

### Phase 8b Historical Results (for reference)

| Repository | Precision | Recall | F1 | FP | FN |
|------------|-----------|--------|----|----|-----|
| nestjs/nest (GT complete) | 99.4% | 93.4% | 96.3% | 1 | 11 |

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
| T101 | how-not-what | INFO | Yes | Yes | Yes | Yes* |
| T102 | fixture-sprawl | INFO | Yes | Yes | Yes* | Yes* |
| T103 | missing-error-test | INFO | Yes | Yes | Yes | Yes* |
| T105 | deterministic-no-metamorphic | INFO | Yes | Yes | Yes | Yes* |
| T106 | duplicate-literal-assertion | OFF | Yes | Yes | Yes | Yes |
| T107 | assertion-roulette | INFO | Yes | -- | Yes | Yes |
| T108 | wait-and-see | INFO | Yes | Yes | Yes | Yes |
| T109 | undescriptive-test-name | INFO | Yes | Yes | Yes | Yes |
| T110 | skip-only-test | INFO | Yes | -- | Yes | -- |

\* Notes:
- Rust T101: token_tree limitation -- private field access in macros not detectable.
- Rust T105: token_tree limitation -- relational operators in `assert!()` not detectable.
- PHP T102: `#[DataProvider]` params excluded from fixture count (#20).
- Rust T102: Smart fixture detection -- constructor/struct/macro counted, method calls on locals excluded (#21).
- Rust T103: `.is_err()` removed as weak proxy -- only `#[should_panic]` and `.unwrap_err()` (#22).
- T107: TypeScript skipped -- Jest/Vitest expect() has no message argument.
- T104: Deprecated and removed in Phase 5.5 (replaced by T106).

## Quality Metrics

| Metric | Current | Target |
|--------|---------|--------|
| Tests | 1187 passing | -- |
| Coverage | N/A | 90%+ (min 80%) |
| Clippy errors | 0 | 0 |

## Output Formats

| Format | Status |
|--------|--------|
| ai-prompt | Supported (default since Phase 17) |
| terminal | Supported |
| json | Supported |
| sarif | Supported (v2.1.0) |
