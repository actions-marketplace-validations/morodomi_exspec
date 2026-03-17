# exspec Roadmap

## Design Principles

1. **exspec is a static lint + observe tool.** Not a template generator or documentation generator
2. **Solo-dev scope constraint.** Don't pursue 2+ large features in parallel
3. **Ship then iterate.** Don't over-polish before release -- but don't ship broken lint
4. **AI separation.** exspec outputs data; humans/AI decide. exspec itself never calls LLMs

## Now

### Phase 9: observe multi-language (test-to-code mapping)

Goal: Expand `exspec observe` from TypeScript-only to all 4 supported languages. Test-to-code mapping only (route extraction deferred).

**Why multi-language, why now**: observe is exspec's differentiator -- no competing tool does static test-to-code mapping. TypeScript PoC proved the approach (NestJS F1 96.3%, typeorm Precision 100%). Expanding to Python/Rust/PHP before deepening TypeScript precision because: (a) "4 languages supported" is a stronger OSS signal than "TypeScript is 99.9% precise", (b) the two-layer mapping algorithm is proven and portable, (c) remaining TypeScript FN (#85 namespace re-export, cross-package barrel) are edge cases with diminishing returns.

**Scope**: Layer 1 (filename convention) + Layer 2 (import tracing) for each language. No route extraction (framework-specific, deferred). No framework-specific patterns (Django URL conf, Actix routes etc.).

**Architecture prerequisite**: Extract `ObserveExtractor` trait from TypeScript-specific implementation. Current observe is hardcoded to TypeScript in CLI + all logic lives in `crates/lang-typescript/src/observe.rs`.

#### 9a: ObserveExtractor trait extraction

Extract language-agnostic observe interface from TypeScript implementation.

**Why first**: All language implementations depend on this trait. Without it, each language would duplicate the mapping algorithm.

| Component | Location | Reusable? |
|-----------|----------|-----------|
| `ObserveReport`, `ObserveFileEntry`, `ObserveSummary` | `core/observe_report.rs` | Already language-agnostic |
| Two-layer mapping algorithm | `lang-typescript/observe.rs` | Extract to trait |
| Barrel/re-export resolution | `lang-typescript/observe.rs` | Extract pattern, impl per-language |
| Helper filtering (`is_non_sut_helper`) | `lang-typescript/observe.rs` | Language-specific suffixes |
| CLI dispatch | `cli/main.rs` | Needs trait-based dispatch |

**Deliverable**: `ObserveExtractor` trait in `crates/core/`, TypeScript refactored to implement it, CLI dispatches via trait object. All existing TypeScript observe tests pass unchanged.

**Review gate**: 9a 完了時に trait の公開 API を設計レビューする。9b 以降で破壊的変更が生じないよう、trait 境界（言語非依存の two-layer アルゴリズム vs 言語固有の import/barrel 解決）を確定してから次に進む。

**Technical debt to address**: `ObserveReport` の `routes` フィールドは TypeScript (NestJS) 固有。Phase 9 では Python/Rust/PHP が空の routes を返す設計になる。route extraction 対応時に `ObserveReport` を refactor する前提で、現時点では許容する。

#### 9b: Python observe

**Why Python first**: Strongest naming conventions (`test_*.py`), simplest import system (no barrel complexity like TypeScript), largest user base.

| Layer | Python equivalent | Complexity |
|-------|-------------------|------------|
| Layer 1 (filename) | `test_user.py` -> `user.py` | Low -- strip `test_` prefix or `_test` suffix |
| Layer 2 (import) | `from myapp.models import User` | Medium -- dotted path to file resolution |
| Barrel equivalent | `__init__.py` with `__all__` | Low -- simpler than TypeScript barrels |
| Helper filtering | `conftest.py`, `constants.py`, `__init__.py` | Low |

**Queries needed**: `production_function.scm`, `import_mapping.scm` (Python `from X import Y` + `import X`), `exported_symbol.scm` (optional, for `__all__`).

**Dogfooding target**: fastapi or pytest (both in dogfooding corpus).

**Success criteria**: Precision >= 90%, Recall >= 80% on dogfooding target.

#### 9c: Rust observe

| Layer | Rust equivalent | Complexity |
|-------|-----------------|------------|
| Layer 1 (filename) | `tests/test_foo.rs` -> `src/foo.rs`, inline `mod tests` | Low -- but inline tests need special handling |
| Layer 2 (import) | `use crate::module::Foo` | Medium -- crate-relative paths |
| Barrel equivalent | `mod.rs`, `lib.rs` re-exports | Medium |
| Helper filtering | `test_utils.rs`, `fixtures/` | Low |

**Unique challenge**: Rust tests are often inline (`#[cfg(test)] mod tests` in the same file). Layer 1 maps these trivially (same file), but Layer 2 needs to handle `use super::*` and `use crate::` paths. **Important**: 同一ファイルが production_file かつ test_file を兼ねるケースで、`unmapped_production_files` の計算ロジックと `ObserveSummary` のカウントが破綻しないよう、実装前に計算ロジックを明確化すること。

**Dogfooding target**: tokio or clap (both in dogfooding corpus). exspec 自身も self-dogfooding 候補。

**Success criteria**: Precision >= 90%, Recall >= 80%.

#### 9d: PHP observe

| Layer | PHP equivalent | Complexity |
|-------|---------------|------------|
| Layer 1 (filename) | `tests/UserTest.php` -> `src/User.php` | Low -- strip `Test` suffix |
| Layer 2 (import) | `use App\Models\User` | Medium -- PSR-4 namespace to file mapping |
| Barrel equivalent | N/A (PHP has no barrel pattern) | N/A |
| Helper filtering | `TestCase.php`, `Factory.php`, `Trait*.php` | Low |

**Unique challenge**: PSR-4 autoloading maps namespace to directory. Need to detect `composer.json` autoload config or fall back to convention. composer.json パースは外部ファイル読み取りを伴い、他言語より実装コストが高い。convention fallback で精度が出るかを先に検証し、不足なら composer.json パースを追加する段階的アプローチを取る。

**Dogfooding target**: Laravel or Symfony (both in dogfooding corpus).

**Success criteria**: Precision >= 90%, Recall >= 80%.

## Next

| Priority | Task | Trigger |
|----------|------|---------|
| P1 | TypeScript observe precision (#85 namespace re-export, cross-package barrel) | After Phase 9 |
| P1 | observe route extraction (NestJS, Django, FastAPI, Actix) | After multi-language observe stabilizes |
| P2 | GitHub Action marketplace | After observe multi-language ships |
| P2 | `exspec init` (framework detection + auto-config) | User onboarding friction |

## Backlog

| Priority | Task | Trigger |
|----------|------|---------|
| P2 | T201 spec-quality (advisory mode) | "I want semantic quality checks" |
| P3 | T203 AST similarity duplicate detection | "I want duplicate test detection" |
| P3 | Go language support (lint) | After observe multi-language proves demand |
| Rejected | LSP/VSCode extension | Too early -- low user count for UI investment |

**Decision: Go language deferred** -- observe multi-language for existing 4 languages takes priority over adding a 5th language to lint. The product differentiator is observe, not language breadth for lint.

**Decision: LSP/VSCode rejection** -- exspec has near-zero external users as of v0.2.0. Building an IDE extension before establishing a user base invests in distribution UX before the core product has proven its value. Reconsiderable after external adoption signals (GitHub stars, issues from non-maintainers).

## Non-goals

- **Semantic validator**: exspec does not judge whether test names are meaningful or properties are sound
- **Coverage tool**: use lcov/istanbul/coverage.py for that
- **AI reviewer**: no LLM calls, zero API cost
- **Framework-specific linter**: rules should be language-agnostic where possible

## Key Design Decisions

### observe multi-language strategy (Phase 9)

- **Test-to-code mapping only** -- route extraction is framework-specific and deferred
- **ObserveExtractor trait** -- language-agnostic interface in `crates/core/`, each lang crate implements it
- **Two-layer algorithm is portable** -- Layer 1 (filename convention) + Layer 2 (import tracing) applies to all 4 languages
- **Language order**: Python (strongest conventions) -> Rust (inline test advantage) -> PHP (PSR-4 complexity)
- **Success bar**: Precision >= 90%, Recall >= 80% per language (lower than TypeScript's 98%/90% due to first-pass nature). observe の低精度出力は lint の FP よりユーザー影響が大きい（「テストされていない」という誤情報）。first-pass 言語は出力に `[experimental]` マーカーを付与し、精度が ship criteria (P>=98%, R>=90%) を超えた言語から stable に昇格する
- **Phase 9 completion**: CONSTITUTION.md Section 8 (Scope Boundaries) の observe 欄を更新する。CONSTITUTION 変更は Human approval 必須 (Section 5)

### observe PoC success (Phase 8b-8c)

- TypeScript observe validated on NestJS (F1 96.3%) and typeorm (Precision 100% spot-check)
- Static AST-only test-to-code mapping is viable -- no existing tool does this
- Product narrative: "AI generates code -> exspec lint for quality -> exspec observe for gap discovery"
- Ship criteria: Precision >= 98%, Recall >= 90% (TypeScript exceeds both)

### T104 removal (Phase 5.5)

"Hardcoded-only" rule penalized DAMP-style tests. Replaced by T106 (duplicate-literal-assertion).

### T001 FP strategy (Phase 6, 4-AI consensus)

- T001 = "oracle-free" detection, not "assert-free"
- Oracle shapes: root (expect/assert) -> modifier chain -> terminal (call or property)
- Bounded vocabulary approach (not ML)
- Custom helpers: `.exspec.toml` `[assertions] custom_patterns` as escape hatch

### Severity philosophy (Phase 6)

- BLOCK: near-zero false positives required
- WARN: heuristic-based, context-dependent
- INFO: opinionated, may be intentional
- Phase 8a results: T101/T102/T108 demoted WARN->INFO, T106 disabled (93% FP)

### Helper delegation (Phase 8a-4)

- User-owned config + runtime guidance. No framework-specific knowledge in exspec core
- T001 runtime hint when custom_patterns is empty and BLOCK count >= 10
- `exspec init` deferred

## Completed Phases

| Phase | Content | Version |
|-------|---------|---------|
| 0-3C | Foundation: SPEC, scaffolding, Python, TypeScript, T001-T008, SARIF | -- |
| 4-4.2 | PHP support (PHPUnit/Pest), FQCN, nested class, Pest arrow | -- |
| 5A-5C | Rust support, Tier 2 rules (T101-T105), T106-T109 | -- |
| 6 | Release Hardening: dogfooding 13 projects / 4 langs / ~45k tests | -- |
| 7 | OSS Release: crates.io publish, GitHub Release | v0.1.2 |
| 8a | Lint Reliability: BLOCK FP fixes, WARN/INFO survey, severity adjustments, helper delegation | v0.1.2 |
| 8b | observe PoC: TypeScript test-to-code mapping (NestJS F1 96.3%, typeorm Precision 100%) | v0.2.0 |
| 8c | observe MVP: failure boundaries, ship criteria, tsconfig path alias, enum/interface filter | v0.2.0 |

Detail for completed phases is archived in git history. Key decisions are preserved in "Key Design Decisions" above.
