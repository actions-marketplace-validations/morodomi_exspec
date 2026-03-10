# Cycle: #48 Chai/Sinon Vocabulary + #49 Arrow Body Property Detection

**Issue**: #48, #49
**Date**: 2026-03-10
**Status**: DONE

## Goal

NestJS dogfooding T001 FP を ~229件削減。Chai/Sinon vocabulary拡張 (~214 FP) + arrow concise body property検出 (~15 FP)。

## Background

Phase 6 NestJS dogfooding で発見されたT001 FP 2件。リスクプロファイルが異なるが同一 assertion.scm への変更のため1 cycle で実施。

## Scope

### In Scope
- #48: Chai/Sinon vocabulary expansion (method-call terminals, intermediates, property terminals, sinon.assert)
- #49: Arrow function concise body での Chai property terminal 検出

### Out of Scope
- NestJS dogfooding 再検証 (別途実施)
- custom helper / project-local DSL パターン

## Design

### #48: Vocabulary Expansion

| 変更箇所 | 追加内容 |
|----------|---------|
| Method-call terminal (depth 2-5) | `eq`, `rejectedWith` |
| Intermediate chain (depth 2-5) | `eventually`, `a`, `an` |
| Property terminal (depth 1-5) | `rejected`, `fulfilled` |
| 新規パターン | `sinon.assert.X()` / `Sinon.assert.X()` depth-2 |

Note: plan では `instanceof` も追加予定だったが、tree-sitter が JS keyword を `property_identifier` として parse しないため除外。既存 `instanceOf` (camelCase) で Chai API はカバー済み。

### #49: Arrow Body Property Detection (Option B)

`expression_statement` パターンと並列に `arrow_function body:` パターンを depth 1-5 で追加。
- double-count リスクなし: `expression_statement` と `arrow_function body:` は構造的に排他
- method-call パターンは `expression_statement` 不使用のため対象外

## Test List

### #48 Fixture: `t001_chai_vocab_expansion.test.ts` (18 TC)

| # | Code | assertion_count | Status |
|---|------|----------------|--------|
| TC-01 | `expect(x).to.eq(y)` | 1 | DONE |
| TC-02 | `expect(x).to.not.eq(y)` | 1 | DONE |
| TC-03 | `expect(p).to.be.rejectedWith(Error)` | 1 | DONE |
| TC-04 | `expect(p).to.not.be.rejectedWith(TypeError)` | 1 | DONE |
| TC-05 | `expect(x).to.be.an.instanceOf(Foo)` | 1 | DONE |
| TC-06 | `expect(x).to.be.instanceOf(Foo)` | 1 | DONE |
| TC-07 | `expect(p).to.eventually.equal(42)` | 1 | DONE |
| TC-08 | `expect(p).to.eventually.be.rejectedWith(Error)` | 1 | DONE |
| TC-09 | `expect(p).to.be.rejected` | 1 | DONE |
| TC-10 | `expect(p).to.be.fulfilled` | 1 | DONE |
| TC-11 | `expect(p).to.eventually.be.rejected` | 1 | DONE |
| TC-12 | `expect(p).to.eventually.be.fulfilled` | 1 | DONE |
| TC-13 | `sinon.assert.callOrder(spy1, spy2)` | 1 | DONE |
| TC-14 | `sinon.assert.calledOnce(spy)` | 1 | DONE |
| TC-15 | `Sinon.assert.calledWith(spy, 'a')` | 1 | DONE |
| TC-16 | `assert.equal(a, b)` regression | 1 | DONE |
| TC-17 | `sinon.stub()` negative | 0 | DONE |
| TC-18 | no assertion negative | 0 | DONE |

### #49 Fixture: `t001_chai_property_arrow.test.ts` (8 TC)

| # | Code | assertion_count | Status |
|---|------|----------------|--------|
| TC-01 | `items.forEach(x => expect(x).to.be.ok)` | 1 | DONE |
| TC-02 | `.then(obj => expect(obj).to.not.be.undefined)` | 1 | DONE |
| TC-03 | `.then(obj => expect(obj).to.have.been.calledOnce)` | 1 | DONE |
| TC-04 | `.then(obj => expect(obj).to.not.have.been.calledOnce)` | 1 | DONE |
| TC-05 | `.then(obj => { expect(obj).to.be.ok; })` regression | 1 | DONE |
| TC-06 | `.then(obj => expect(obj).to.equal(42))` regression | 1 | DONE |
| TC-07 | `.then(obj => expect(p).to.be.rejected)` #48+#49 | 1 | DONE |
| TC-08 | `.map(x => x + 1)` negative | 0 | DONE |

## Files Changed

| File | Change |
|------|--------|
| `crates/lang-typescript/queries/assertion.scm` | MODIFIED: vocab expansion + arrow body patterns + sinon.assert |
| `crates/lang-typescript/src/lib.rs` | ADD: 2 integration tests |
| `tests/fixtures/typescript/t001_chai_vocab_expansion.test.ts` | NEW: 18 TC |
| `tests/fixtures/typescript/t001_chai_property_arrow.test.ts` | NEW: 8 TC |

## Progress Log

- 2026-03-10 12:01 GREEN: 全26 TC PASS, 全598テスト PASS, clippy clean, fmt clean
- 2026-03-10 12:10 REFACTOR: /simplify 3-agent review完了。コメント更新1件 (property terminal allowlist docstring drift)。.scm重複はtree-sitter制約のためスキップ。Verification Gate通過。
- 2026-03-10 12:15 REVIEW: code review PASS (score 12). Security 0, Correctness 12 (optional x3: not depth-2 gap=既存/a|an dual-role=doc十分/hard-index=一貫性優先). Phase completed.
- 2026-03-10 12:20 COMMIT: Phase completed.
