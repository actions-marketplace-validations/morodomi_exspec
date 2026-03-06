# Phase 2: Python + Tier 1 Rules (T001-T003)

## Status: DONE

## Goal
- Python test function extraction via tree-sitter queries
- T001 (assertion-free, BLOCK), T002 (mock-overuse, WARN), T003 (giant-test, WARN)
- Terminal + JSON output formatters
- CLI pipeline with file discovery
- Dogfooding-ready MVP

## Architecture Decisions
- Rules in core, extraction in lang-*
- TestAnalysis embedded in TestFunction (not tuple/Map)
- LanguageExtractor trait: check() removed, extract_test_functions() only
- .scm queries via include_str! (MVP)
- Config struct with defaults in rules.rs
- Assertion/mock detection scoped to function node (mock: decorated_definition if present)
- Mock class name: strip mock_ prefix

## TDD Cycles

| Cycle | Target | Tests | Status |
|-------|--------|-------|--------|
| 1 | Core: TestAnalysis + evaluate_rules + trait | 20 | DONE |
| 2+3 | Python: extraction + assertion/mock detection | 13 | DONE |
| 4a | Output: terminal/JSON formatters, exit code | 11 | DONE |
| 4b | CLI: file discovery, E2E pipeline | 14 | DONE |

## Results

- 58 tests, all passing
- clippy: 0 warnings
- fmt: clean

## Review

| Reviewer | Score | Verdict |
|----------|-------|---------|
| Security | 12 | PASS |
| Correctness | 35 | PASS |
| Performance | 35 | PASS |

### Key Findings
- Suppression wiring deferred to Phase 3 (planned scope)
- Test temp dir race condition: fixed (PID-based isolation)
- Query/Parser per-file allocation: v0.2 optimization target
- find_node_by_id O(N*T): v0.2 optimization target

## Phase Summaries

### Cycle 1: Core
- TestAnalysis struct (assertion_count, mock_count, mock_classes, line_count, suppressed_rules)
- TestFunction.analysis field added
- Config struct with defaults (mock_max=5, mock_class_max=3, test_max_lines=50)
- evaluate_rules() implementing T001/T002/T003
- is_disabled() / is_suppressed() helpers
- LanguageExtractor trait: check() removed
- lang-typescript stub updated

### Cycle 2+3: Python Extraction
- test_function.scm: decorated + undecorated patterns with deduplication
- assertion.scm: assert_statement + self.assert* (unittest)
- mock_usage.scm: MagicMock/Mock + @patch decorator (attribute + identifier forms)
- mock_assignment.scm: mock_xxx = MagicMock() class name extraction
- StreamingIterator handling with TestMatch intermediate + find_node_by_id
- 8 Python fixture files

### Cycle 4a: Output
- format_terminal(): SEVERITY file:line RULE message
- format_json(): {"diagnostics": [...], "metrics": {}}
- compute_exit_code(): BLOCK=1, WARN=0, strict+WARN=1

### Cycle 4b: CLI Pipeline
- is_python_test_file(): test_*.py / *_test.py matching
- discover_test_files(): ignore crate with .gitignore/.hidden support
- main(): discover -> extract -> evaluate -> format -> exit

## DISCOVERED (v0.2)
- Query/Parser caching (OnceLock or struct field)
- find_node_by_id -> descendant_for_byte_range O(log N)
- test_function.scm: non-TestCase class method false positive
- Empty result UX: summary line output
