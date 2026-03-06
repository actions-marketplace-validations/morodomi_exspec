# Phase 1: Rust + tree-sitter Scaffolding

## Status: DONE

## Goal
- Cargo workspace (4 crates: core, lang-python, lang-typescript, cli)
- tree-sitter + language bindings working
- crate dependency direction: core trait -> lang-* impl
- `cargo build` + `cargo test` + `cargo clippy` all pass

## TDD Steps

| Step | Target | Status |
|------|--------|--------|
| 1 | Workspace + Core crate | DONE |
| 2 | lang-python crate | DONE |
| 3 | lang-typescript crate | DONE |
| 4 | CLI crate | DONE |
| 5 | Query directories (empty) | DONE |

## Results

- 17 tests, all passing
- clippy: 0 warnings
- fmt: clean

## Phase Summaries

### Step 1: Core crate
- Severity enum with Ord, FromStr, as_str, exit_code
- RuleId, Diagnostic structs
- Rule trait, LanguageExtractor trait
- parse_suppression for inline suppression
- Tests: severity ordering, roundtrip, exit_code, invalid parse, suppression parsing

### Step 2: lang-python
- PythonExtractor implementing LanguageExtractor trait
- tree-sitter-python 0.23 parsing confirmed (root node = "module")
- Contract test: PythonExtractor is &dyn LanguageExtractor

### Step 3: lang-typescript
- TypeScriptExtractor implementing LanguageExtractor trait
- tree-sitter-typescript 0.23 parsing confirmed (root node = "program")
- Contract test: TypeScriptExtractor is &dyn LanguageExtractor

### Step 4: CLI
- clap derive-based Cli struct
- Args: path, --format, --lang, --strict
- Tests: path parsing, defaults, strict flag, format option, --help

### Step 5: Query directories
- crates/lang-python/queries/ (empty, .scm files added in Phase 2)
- crates/lang-typescript/queries/ (empty)
