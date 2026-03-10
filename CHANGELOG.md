# Changelog

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
- **Not on crates.io**: Install via `cargo install --git`. crates.io publish is intentionally deferred
- **No stability guarantee**: Rule IDs, severity levels, and config format may change in minor versions
- **Known false positives**: Helper delegation patterns require `custom_patterns` config. See [Known Constraints](README.md#known-constraints) in README

### Install

```bash
cargo install --git https://github.com/morodomi/exspec.git
```
