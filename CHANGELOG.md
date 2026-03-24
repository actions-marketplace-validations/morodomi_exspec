# Changelog

## v0.4.3 (2026-03-24)

Same-file helper tracing for all 4 languages, L1 exclusive mode for observe, and GT audit.

### Features

- **Same-file helper tracing (Python)**: Port of Phase 23a. Tests calling helpers with assertions in the same file are no longer T001 BLOCK FP. (#150)
- **Same-file helper tracing (TypeScript)**: Includes arrow function helper support. (#151)
- **Same-file helper tracing (PHP)**: Free function helpers at file scope. (#152)
- **L1 exclusive mode**: `--l1-exclusive` flag suppresses L2 import tracing for L1-matched test files. Reduces FP from incidental imports. (#131)

### Dogfooding

- Same-file helper tracing: near-zero BLOCK reduction across all projects. Helper delegation FP dominated by cross-file class method calls.
- GT audit (#149): Rust P=76.7% (tokio), PHP P=90.0% (laravel). Both FAIL ship criteria (P>=98%). Remain experimental.
- #153 (cross-file) deferred to v0.4.4 (all languages FP <= 5%).
- #129 (fan-out filter) deferred to backlog (#131 resolved httpx FP).

### Internal

- 1142 tests (up from 1119 in v0.4.2).
- #144 closed (already fixed by #146 in v0.4.2).

## v0.4.2 (2026-03-23)

Python observe precision/recall improvements and Rust/PHP observe dogfooding baselines.

### Features

- **Python stem collision guard**: When multiple production files share the same stem (e.g., `models.py` in different directories), stem-only fallback now defers to L2 import tracing instead of mapping to all matches. Improves precision for projects with common filenames. (#126)
- **Python sub-module direct import bypass**: Assertion filter now bypasses for direct sub-module imports (`from pkg._urlparse import normalize`), preventing false negatives when tests import non-barrel production files. (#119, #145)
- **Relative direct import support**: Assertion filter bypass extended to relative import branches (`from ._config import Config`, `from . import utils`). Previously only absolute imports were covered. (#146)

### Dogfooding

- Rust observe re-dogfooding: tokio 51->71 mapped (+20), clap 20->22 mapped (+2).
- PHP observe re-dogfooding: laravel 968->973 mapped (+5).
- Rust/PHP observe remain experimental (GT audit pending #149).

### Internal

- 1119 tests (up from 1101 in v0.4.1).

## v0.4.1 (2026-03-23)

Rust lint improvements, Django tests.py support, and internal cleanup.

### Features

- **Django `tests.py` recognition**: Python observe now recognizes Django's `tests.py` naming convention. 1669 Django test files were previously invisible. `test_stem` returns parent directory name, `production_stem` excludes `tests.py`. (#95)
- **Rust same-file helper tracing**: Detect assertions inside helper functions called from test functions within the same file. Phase 23a of helper delegation. (#140)
- **Rust custom assert macro auto-detection**: `assert_*!` macro invocations (e.g., `assert_pending!`, `assert_ready!`) are automatically recognized as assertions. tokio BLOCK 385->247 (-138), clap BLOCK 193->43 (-150). (#138)

### Bug Fixes

- **Rust `should_panic` exact match**: Tightened `#[should_panic]` detection from substring match to exact tree-sitter identifier walk. Attributes like `#[my_should_panic_wrapper]` no longer falsely match. tokio -10, clap -28 BLOCK. (#29)

### Internal

- PHP `error_test.scm` aligned to `assertion.scm` matching convention (inner `name` node). Round-trip test added. (#30)
- Document shadow variable limitation in `known-constraints.md`. (#122)
- Test: `resolve_absolute_base_to_file` file-vs-package priority. (#97)
- Test: bare import attribute-access narrowing and dotted fallback. (#121)
- ROADMAP.md updated: Phase 22-24 completed, v0.4.1 scope.
- 1101 tests (up from 1087 in v0.4.0).

## v0.4.0 (2026-03-22)

Python observe reaches stable (ship criteria P>=98%, R>=90%), new default output format, and route extraction for 4 frameworks.

### Features

- **Python observe stable**: Ship criteria achieved (P=98.2%, R=96.8% on httpx). L1 prefix stripping, L2 barrel import resolution, assertion-referenced import filter, test helper exclusion, and non-SUT helper filtering (mock/version/types).
- **`ai-prompt` default output**: New `--format ai-prompt` output with actionable fix guidance, now the default format. Previous default was `terminal`.
- **Route extraction**: NestJS decorators, FastAPI route decorators, Next.js App Router `route.ts`, Django URL conf patterns.
- **Python observe L2 improvements**: Barrel wildcard re-export resolution, bare `import` statement resolution, attribute-access filtering for precision, stem-only fallback with barrel suppression.

### Bug Fixes

- **Python observe FP reduction**: `is_non_sut_helper()` now excludes `mock*.py` (test fixtures), `__version__.py` (metadata), `_types.py` (type definitions) from production file candidates, eliminating barrel fan-out false positives.
- **Python `_` prefix in L1**: `production_stem()` strips leading `_` for filename matching (`_decoders.py` matches `test_decoders.py`).
- **Python `src/` layout**: L2 import resolution detects `src/<package>/` project structure.

### Internal

- Ground truth re-audited for httpx: 23 secondary targets added.
- 1087 tests (up from 918 in v0.3.0).

## v0.3.0 (2026-03-18)

Multi-language observe (Python, Rust, PHP), route extraction framework, and Python observe initial implementation.

### Features

- **Multi-language observe**: `ObserveExtractor` trait enables test-to-code mapping for Python, Rust, and PHP (all `[experimental]`). TypeScript remains stable.
- **Python observe**: Dotted import resolution, `__init__.py` barrel detection. First-pass: P=66.7%, R=6.2% on httpx (improved in v0.4.0).
- **Rust observe**: `use crate::`/`use cratename::` resolution, workspace member aggregation, `pub mod` barrel.
- **PHP observe**: PSR-4 namespace resolution.

## v0.2.0 (2026-03-17)

New `exspec observe` subcommand for static test-to-code mapping, lint reliability improvements, and workspace consolidation.

### Features

- **`exspec observe` subcommand**: Static test-to-code mapping via AST analysis. TypeScript only (PoC). Filename convention (L1) + import tracing (L2) with barrel import resolution, tsconfig path alias support, and context-aware enum/interface filtering. Precision 99.4%, Recall 93.4% on nestjs/nest ground truth.
- **T001 runtime hint** (#68): When `custom_patterns` is unconfigured, T001 now suggests adding project-specific assertion helpers.
- **Severity rebalance** (#69, #70, #72, #73): T101/T102/T108 demoted from WARN to INFO. T106 disabled by default. Reduces noise for gradual adoption.

### Bug Fixes

- **Empty `custom_patterns`** (#36): Empty string patterns in config no longer cause false matches.
- **Python nested `test_*` functions** (#41): Inner functions named `test_*` inside test functions are no longer extracted as separate tests.
- **Barrel re-export symbol filter**: Wildcard re-exports (`export * from`) now filter by actually exported symbols, preventing false observe mappings.
- **Abstract class handling**: Abstract classes no longer produce duplicate entries in observe extraction.
- **Layer 2 import tracing scope**: Import tracing now runs on all test files, not just those unmatched by Layer 1.

### Internal

- Workspace consolidated: `workspace.package` + `workspace.dependencies` reduce version update points from 11 to 6.

## v0.1.2 (2026-03-12)

Continued false-positive reduction from extended dogfooding (13 projects / 4 languages / ~45,000 tests) and a new rule.

### Features

- **T110 skip-only-test detection** (#65): New INFO rule that flags test functions whose only logic is `skip()` / `markTestSkipped()` / `pytest.skip()`. These are placeholder tests that should either be implemented or removed.

### Bug Fixes

- **Python `^assert_` -> `^assert` broadening** (#62): Python assertion pattern now matches `assertoutcome()` and other helpers without underscore after `assert`. Fixes ~148 FPs in pytest's own test suite.
- **PHP `addToAssertionCount()`** (#63): Recognized as a valid assertion for T001. Fixes 91 FPs in Symfony.
- **Skip-only tests excluded from T001** (#64): Test functions that only call skip/markTestSkipped are no longer flagged as assertion-free. Fixes 91 FPs in Symfony.
- **Rust `assert*()` helper function calls** (#66): Simple `assert_matches()` and scoped `common::assert_foo()` function calls are now detected as assertions for T001.
- **Return-wrapped Chai property assertions** (#52): `return expect(x).to.be.true` is now correctly counted as an assertion.

### Documentation

- v0.1.0 historical correction: crates.io publish happened at v0.1.1, not deferred as originally stated.

## v0.1.1 (2026-03-11)

Bug fixes and two new configuration features since the initial public beta.

### Features

- **`--min-severity` display filter** (#59): Filter terminal/JSON output by severity level. `exspec --min-severity warn .` hides INFO diagnostics. Does not affect exit code (BLOCK violations still fail regardless of filter).
- **Per-rule severity override** (#60): `[rules.severity]` in `.exspec.toml` lets you change a rule's evaluation severity or disable it entirely. `T107 = "off"` disables the rule; `T101 = "info"` downgrades from WARN to INFO. This is orthogonal to `--min-severity`: severity overrides change *evaluation*, while `--min-severity` controls *display*.

### Bug Fixes

- **`.tsx` files**: TypeScript assertion detection now uses the TSX parser, fixing false positives on `.tsx` test files (#53)
- **`[paths] ignore` config**: The `ignore` patterns in `.exspec.toml` were not applied to file discovery. Fixed (#54)
- **T109 CJK test names**: Single-word heuristic falsely flagged Japanese/Chinese test names as undescriptive. CJK character sequences are now excluded (#55)
- **`@pytest.fixture` false positives**: Functions decorated with `@pytest.fixture` that happen to start with `test_` are no longer analyzed as test functions (#56)
- **`pytest.fail()` as test oracle**: `pytest.fail()` is now recognized as a valid assertion for T001 (#57)
- **PHP `Facade::shouldReceive()`**: Static Mockery calls on Laravel Facades (`Event::shouldReceive()`, etc.) are now recognized as assertions for T001 (#58)

### Internal

- T109 suffix check uses `chars().count()` instead of `len()` for correct Unicode handling (#61)
- `KNOWN_RULE_IDS` extracted as single source of truth for rule ID validation (#60)

## v0.1.0 (2026-03-10) -- Public Beta

First public release. Dogfooded across 9 projects, 4 languages, ~23,000 tests.

### What this release includes

- **16 check rules** (Tier 1 + Tier 2) for test design quality
- **4 languages**: Python (pytest), TypeScript (Jest/Vitest), PHP (PHPUnit/Pest), Rust (cargo test)
- **Output formats**: Terminal, JSON, SARIF (GitHub Code Scanning)
- **Inline suppression**: `# exspec-ignore: T001` per function
- **Custom assertion helpers**: `[assertions] custom_patterns` in `.exspec.toml`
- **Gradual adoption**: disable Tier 2 rules, enable one at a time

### What this release does NOT promise

- **Not production-ready**: This is a public beta for trial and gradual adoption
- **~~Not on crates.io~~**: *(Correction: published to crates.io at v0.1.1. At v0.1.0 release time, install was git-only.)*
- **No stability guarantee**: Rule IDs, severity levels, and config format may change in minor versions
- **Known false positives**: Helper delegation patterns require `custom_patterns` config. See [Known Constraints](README.md#known-constraints) in README

### Install

```bash
cargo install exspec
```
