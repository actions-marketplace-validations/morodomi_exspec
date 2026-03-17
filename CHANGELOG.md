# Changelog

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
