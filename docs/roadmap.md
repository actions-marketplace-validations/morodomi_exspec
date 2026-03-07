# exspec Roadmap

## Completed

| Phase | Content | Tests |
|-------|---------|-------|
| 0 | SPEC.md + naming | -- |
| 1 | Rust + tree-sitter scaffolding | -- |
| 2 | Python + Tier 1 (T001-T003) | -- |
| 3A | TypeScript + inline suppression + output polish | 87 |
| 3B | T004-T008 + .exspec.toml parsing | 153 |
| 3C | SARIF output + ProjectMetrics (MVP) | 203 |
| 4 | PHP support (PHPUnit/Pest) + dev-crew integration | 263 |
| 5A | Rust language support (cargo test) | 317 |
| 5B | Tier 2 rules T101-T105 (Python + TypeScript) | 437 |
| 5C | Tier 2 PHP/Rust expansion (T101-T105, T104 excluded) | 466 |

## Upcoming

### Phase 6 - Tier 3: AI Prompt Generation

| Rule | Output |
|------|--------|
| T201 | spec-quality: test readability AI inspection prompt |
| T202 | contract-property-coherence: contract + property consistency prompt |
| T203 | test-duplication: similarity-based duplicate detection |

### Phase 7 - OSS Release

- README polish
- cargo publish (crates.io)
- Note.com article
- MCP Server integration

## Design Decisions

### T104 Redesign (Deferred)

**Problem**: Current "hardcoded-only" rule penalizes DAMP-style tests where explicit literals
are the correct choice (e.g., `assert add(1, 2) == 3`). Both Gemini and Grok independently
identified this as a conceptual flaw.

**Options under consideration**:

1. **Redefine as "duplicate-literal-assertion"** (T106): Detect same non-trivial literal
   repeated across multiple assertions in a function/file. Threshold TBD (N=3-4).
   Requires excluded-literal list (0, 1, true, false, None, "").
2. **Deprecate entirely**: Remove T104 and absorb into future "Magic Number Test" gap rule.

**Decision**: Deferred to Phase 6+. Current T104 remains implemented for Python/TypeScript
(INFO level, low harm) but will NOT be expanded to PHP/Rust.

### Gap Rules (Future)

Identified via Gemini/Grok analysis of existing test smell literature:

| Candidate | Description | Feasibility (tree-sitter) |
|-----------|-------------|--------------------------|
| Assertion Roulette | Multiple asserts without messages in one test | High |
| Duplicate Assert | Same assertion repeated in one test | High |
| Mystery Guest | External file dependency in test (open(), readFile()) | Medium |
| Wait-and-See | sleep()/delay() in test code | High |
| Eager Test | One test calls multiple unrelated production methods | Low (needs type info) |

Priority: Assertion Roulette > Wait-and-See > Duplicate Assert > Mystery Guest >> Eager Test

### Rust-specific Smells (Future)

- Excessive `unwrap()`/`expect()` in tests
- `todo!()`/`unimplemented!()` left in tests
- Over-reliance on `insta::assert_snapshot!` without property checks

### PHP-specific Smells (Future)

- ReflectionClass for private property access (strong warning candidate)
- Heavy `@dataProvider` with complex anonymous arrays
