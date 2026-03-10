# Cycle: #47 T001 FP — supertest .expect() method-call oracle

**Issue**: #47
**Date**: 2026-03-10
**Status**: DONE

## Goal

supertest の `.expect()` メソッドチェーンを assertion として検出し、NestJS dogfooding の T001 BLOCK を 813 → ~238 に削減する。

## Background

NestJS dogfooding で T001 BLOCK 813件中 575件 (71%) が supertest の `.expect()` メソッドチェーン。
現在の assertion.scm は `expect()` をスタンドアロン関数としてのみ検出するため、メソッド呼び出しの `.expect()` を見逃していた。

## Scope

### In Scope
- `call_expression.expect()` パターン (supertest, Fastify inject 等)
- 6テストケース: 単一/複数/set+expect/no-assertion/standalone/非supertest

### Out of Scope
- import/型情報によるフィルタリング (tree-sitter query レベルでは不可)
- NestJS dogfooding 検証 (別途実施)

## Design

### Approach: assertion.scm に 1パターン追加

```scheme
(call_expression
  function: (member_expression
    object: (call_expression)
    property: (property_identifier) @_supertest_prop
    (#eq? @_supertest_prop "expect"))) @assertion
```

**Why `object: (call_expression)` constraint:**
- supertest の `.expect()` は常にチェーン上 (`request().get().expect()`)
- `identifier.expect()` を除外 → 非assertion `.expect()` メソッドの誤検出を防ぐ
- 既存 `expect()` 関数パターンとの重複なし (identifier vs call_expression で構造的に分離)

**Broad by design:**
- `someBuilder().expect('foo')` のような非supertest chain上の `.expect()` も検出される
- リスク方向: false negative (assertion-free見逃し) であり、false positive (正当テストのBLOCK) ではない

### Rust コード変更なし
assertion.scm のパターン追加のみ。

## Test List

| # | Given | When | Then |
|---|-------|------|------|
| TC-01 | single `.expect(200)` on chain | assertion count | 1 |
| TC-02 | two `.expect()` on same chain | assertion count | 2 |
| TC-03 | `.set()` + two `.expect()` | assertion count | 2 |
| TC-04 | no assertion (plain request) | assertion count | 0 (T001 BLOCK) |
| TC-05 | standalone `expect(x).toBe(y)` | assertion count | 1 (no double-count) |
| TC-06 | `someBuilder().expect('foo')` | assertion count | 1 (broad by design) |

## Files Changed

| File | Change |
|------|--------|
| `tests/fixtures/typescript/t001_supertest.test.ts` | NEW: fixture (6 test cases) |
| `crates/lang-typescript/src/lib.rs` | ADD: integration test |
| `crates/lang-typescript/queries/assertion.scm` | ADD: 1 query pattern (~6 lines) |

## Progress Log

- GREEN: 全6 TC PASS, 全596テスト PASS, clippy clean, fmt clean
- REFACTOR: /simplify 3-agent review完了。指摘は全て既存パターンとの一貫性に関するもの(LOW/MEDIUM)でスコープ外。コードクリーン。Verification Gate通過。
- REVIEW: code review PASS (score 8). Security 0, Correctness 8 (optional x3: コメント追記→反映済み).
