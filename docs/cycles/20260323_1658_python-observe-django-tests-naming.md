---
feature: python-observe-django-tests-naming
cycle: 20260323_1658
phase: DONE
complexity: trivial
test_count: 7
risk_level: low
codex_session_id: ""
created: 2026-03-23 16:58
updated: 2026-03-23 16:58
---

# Python observe Django tests.py naming convention support (Phase 24)

## Scope Definition

### In Scope
- [ ] `is_python_test_file` (CLI) に `tests.py` を追加
- [ ] `test_stem` (Python observe) に `tests.py` → 親ディレクトリ名 のマッピングを追加
- [ ] `production_stem` (Python observe) から `tests.py` を除外

### Out of Scope
- `is_non_sut_helper` の変更なし (tests.py は production_files に含まれなくなるため不要)
- Layer 2 import tracing の変更なし (test_sources に入れば自動処理される)

### Files to Change (target: 10 or less)
- `crates/cli/src/main.rs` (edit)
- `crates/lang-python/src/observe.rs` (edit)

## Environment

### Scope
- Layer: Backend
- Plugin: cargo test + clippy + fmt + self-dogfood
- Risk: 20 (PASS)

### Runtime
- Language: Rust

### Dependencies (key packages)
- tree-sitter: workspace
- tree-sitter-python: workspace

### Risk Interview (BLOCK only)
(N/A - Risk 20, PASS)

## Context & Dependencies

### Reference Documents
- [CONSTITUTION.md] - Static AST analysis, language-agnostic approach. Django convention support per "Detect structural test smells" goal.
- [ROADMAP.md] - observe recall improvement is ongoing.

### Dependent Features
- Python observe Layer 1 / Layer 2: `crates/lang-python/src/observe.rs`

### Related Issues/PRs
- Issue #95: Python observe Django tests.py naming convention support (Phase 24)

## Test List

### TODO
- [ ] PY-STEM-13: `test_stem("app/tests.py")` → `Some("app")`
  - Given: path = "app/tests.py"
  - When: test_stem(path)
  - Then: Some("app")
- [ ] PY-STEM-14: `test_stem("tests/aggregation/tests.py")` → `Some("aggregation")`
  - Given: path = "tests/aggregation/tests.py"
  - When: test_stem(path)
  - Then: Some("aggregation")
- [ ] PY-STEM-15: `test_stem("tests.py")` → `None` (no parent dir)
  - Given: path = "tests.py"
  - When: test_stem(path)
  - Then: None
- [ ] PY-STEM-16: `production_stem("app/tests.py")` → `None`
  - Given: path = "app/tests.py"
  - When: production_stem(path)
  - Then: None
- [ ] CLI-PY-TESTS-01: `is_python_test_file("app/tests.py")` → true
  - Given: path = "app/tests.py"
  - When: is_python_test_file(path)
  - Then: true
- [ ] CLI-PY-TESTS-02: `is_python_test_file("tests/__init__.py")` → false
  - Given: path = "tests/__init__.py"
  - When: is_python_test_file(path)
  - Then: false
- [ ] PY-L2-DJANGO-01: Django layout tests.py mapped via Layer 2
  - Given: tempdir with `src/models.py` (production, `class Model: ...`) and `app/tests.py` (test, `from src.models import Model`)
  - When: map_test_files_with_imports
  - Then: `tests.py` maps to `models.py` via ImportTracing strategy

### WIP
(none)

### DISCOVERED
(none)

### DONE
(none)

## Implementation Notes

### Goal
Django の `tests.py` 命名規則をサポートし、Django プロジェクトのテストファイルが observe に認識されるようにする。

### Background
Python observe は `test_*.py` と `*_test.py` のみをテストファイルと認識する。Django は `tests.py` (完全一致) を標準規則として使用するため、Django プロジェクトの全テストファイル (例: Django 本体の 1669 ファイル) が observe に認識されず、マッピングが不可能な状態になっている。

具体的な問題:
1. `is_python_test_file` (CLI) が `tests.py` に対して false を返す
2. `tests.py` が production source ファイルとして分類される
3. `test_sources` に入らないため Layer 2 import tracing が処理しない
4. Django の全テストファイルが observe で不可視になる

### Design Approach
3 つのタッチポイントを修正する:

1. **CLI `is_python_test_file`** (`crates/cli/src/main.rs:85`): `filename == "tests.py"` を条件に追加。
2. **Python observe `test_stem`** (`crates/lang-python/src/observe.rs:50`): `tests.py` の stem が `"tests"` のとき、親ディレクトリ名を stem として返す (例: `app/tests.py` → `"app"`)。パスにセパレータがない場合 (`tests.py` 単体) は None を返し L1 マッチをスキップ、L2 import tracing に委ねる。
3. **Python observe `production_stem`** (`crates/lang-python/src/observe.rs:70`): `stem == "tests"` の場合は None を返し、production ファイルから除外する。

Layer 2 (`map_test_files_with_imports`) は `test_sources` を直接イテレートするため、`tests.py` が `test_sources` に入れば変更不要。

## Progress Log

### 2026-03-23 16:58 - INIT
- Cycle doc created
- Scope definition ready

### 2026-03-23 - PLAN-REVIEW
- design-reviewer: PASS (blocking_score 42)
- Important: lifetime安全性 → plan案の rfind('/') ベース実装で対応済み
- Important: production/test排他性 → discover_files で test判定が先、排他的構造を確認済み
- Optional: is_non_sut_helperとの相互作用をImplementation Notesに明記
- Optional: ROADMAP.md へ Phase 24 エントリ追加（commit時）
- Phase completed

### 2026-03-23 - RED
- 7 tests written: PY-STEM-13~16, CLI-PY-TESTS-01~02, PY-L2-DJANGO-01
- 4 FAILED (RED): PY-STEM-13, PY-STEM-14, PY-STEM-16, CLI-PY-TESTS-01
- 3 PASS (existing behavior docs): PY-STEM-15, CLI-PY-TESTS-02, PY-L2-DJANGO-01
- Self-dogfooding: BLOCK 0
- Phase completed

### 2026-03-23 - GREEN
- 3 files modified: cli/main.rs, lang-python/observe.rs (test_stem + production_stem)
- All 225 tests PASS, clippy 0, fmt clean, self-dogfood BLOCK 0
- Phase completed

### 2026-03-23 - REFACTOR
- Checklist review: no improvements needed (clean implementation, no duplication)
- Verification Gate: PASS (1106 tests, clippy 0, fmt clean, BLOCK 0)
- Phase completed

### 2026-03-23 - REVIEW
- correctness-reviewer: PASS (score 12). Optional: tests/tests.py edge case test, condition reorder.
- security-reviewer: PASS (score 3). UTF-8 slicing safe, no path traversal risk.
- Phase completed

---

## Next Steps

1. [Done] INIT <- Current
2. [Done] PLAN
3. [Done] RED
4. [Done] GREEN
5. [Done] REFACTOR
6. [Done] REVIEW — PASS (correctness: 12, security: 3)
7. [Done] COMMIT
