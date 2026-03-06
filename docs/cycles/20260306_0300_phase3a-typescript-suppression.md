# Phase 3A: TypeScript + Inline Suppression + Output Polish

## Status: DONE

## Goal
- Tech debt: OnceLock query caching, descendant_for_byte_range O(log N)
- TypeScript extraction: test/it functions, assertions, mocks (Jest/Vitest/sinon)
- Inline suppression wiring for Python + TypeScript
- CLI: TypeScript file discovery, --lang filter
- Output: terminal summary header/footer, JSON version/summary

## Architecture Decisions
- OnceLock<Query> static caching (Parser stays per-call due to mutability)
- descendant_for_byte_range replaces find_node_by_id for O(log N) node lookup
- Inline suppression: previous-line comment approach (language-agnostic)
- TS mock class name: camelCase strip (mockDb -> "Db"), Python: snake_case strip (mock_db -> "db")
- format_terminal/format_json now take (diagnostics, file_count, function_count)

## TDD Cycles

| Cycle | Target | Tests | Status |
|-------|--------|-------|--------|
| 0 | Tech debt: OnceLock + byte range | 13 (regression) | DONE |
| 1 | TS: test function extraction | +4 | DONE |
| 2 | TS: assertion + mock detection | +7 | DONE |
| 3 | Inline suppression (Py + TS) | +4 | DONE |
| 4 | CLI multi-lang + output | +14 | DONE |
| 5 | E2E validation | (manual) | DONE |

## Results

- 87 tests, all passing (58 -> 87, +29 new)
- clippy: 0 warnings
- fmt: clean

## Review

| Reviewer | Score | Verdict |
|----------|-------|---------|
| Security | 12 | PASS |
| Correctness | 42 | PASS |

### Key Findings
- descendant_for_byte_range correctness: tests confirm correct behavior
- pass_count double-subtracts multi-violation functions -> #2
- TS suppression inside describe() limitation -> #3
- fn_node_id naming misleading after refactor -> #4
- --lang validation missing -> #1

## Files Changed

| Action | File |
|--------|------|
| Modify | crates/lang-python/src/lib.rs (OnceLock, byte range, suppression) |
| Modify | crates/lang-typescript/src/lib.rs (full extraction impl) |
| Modify | crates/lang-typescript/Cargo.toml (streaming-iterator) |
| Modify | crates/cli/src/main.rs (TS discovery, --lang, multi-extractor) |
| Modify | crates/cli/Cargo.toml (exspec-lang-typescript) |
| Modify | crates/core/src/output.rs (summary header/footer, JSON version/summary) |
| Create | crates/lang-typescript/queries/test_function.scm |
| Create | crates/lang-typescript/queries/assertion.scm |
| Create | crates/lang-typescript/queries/mock_usage.scm |
| Create | crates/lang-typescript/queries/mock_assignment.scm |
| Create | tests/fixtures/typescript/*.test.ts (8 files) |
| Create | tests/fixtures/python/suppressed.py |

## DISCOVERED (Phase 3B)
- #1: --lang argument validation
- #2: pass_count multi-violation fix
- #3: TS suppression describe() limitation docs
- #4: fn_node_id rename
