# Cycle: #40 T001 FP: TS Chai method-call chain terminals

**Issue**: #40
**Date**: 2026-03-09
**Status**: DONE

## Goal

Detect Chai BDD method-call chain assertions (`expect(x).to.equal(y)`) as oracles, eliminating ~5% of vitest T001 false positives.

## Background

Chai BDD chains use `to/be/have/been` as intermediates and `equal/a/throw` etc. as terminals. Existing depth-2/3 patterns constrain intermediates to `not|resolves|rejects` and terminal to `^to[A-Z]`, missing all Chai-style method-call chains. Property-style assertions (no parens) were already fixed in #32.

## Scope

### In Scope
- Chai BDD method-call chain patterns at depths 2-5
- Terminal vocabulary: 35 dogfooding-driven assertion methods
- Double-count avoidance with existing modifier-chain patterns

### Out of Scope
- Python nested function assertion counting (#41)
- Severity review (#24)

## Design

### Approach: Chai vocabulary-constrained pattern addition

Added 4 new scm patterns (depth 2-5) separate from existing Jest/Vitest modifier-chain patterns.

**Intermediates**: `to|be|been|have` (depth-2, `not` excluded to avoid overlap), `to|be|been|have|not` (depth-3+).

**Terminals**: bounded vocabulary of 35 methods: `equal|eql|a|an|include|contain|throw|match|property|keys|lengthOf|members|satisfy|closeTo|above|below|least|most|within|instanceOf|respondTo|oneOf|change|increase|decrease|by|string|calledWith|calledOnceWith|calledWithExactly|calledOn|callCount|returned|thrown`.

### Double-count avoidance
- depth-2: `not` excluded from intermediates (existing modifier-chain already handles `expect(x).not.<method>()`)
- depth-3+: `not` included because existing depth-3 terminal constraint `^to[A-Z]` doesn't overlap with Chai terminals

## Changes

| File | Change |
|------|--------|
| `crates/lang-typescript/queries/assertion.scm` | +4 Chai method-call patterns (depth 2-5) |
| `tests/fixtures/typescript/t001_chai_method_call.test.ts` | NEW: 10 test cases |
| `crates/lang-typescript/src/lib.rs` | +2 tests (fixture + double-count) |

## Test Results

- 570 tests pass
- clippy clean, fmt clean
- Dogfooding: vitest BLOCK 350 -> 334 (16 FPs resolved)

## Dogfooding History

| Change | BLOCK count |
|--------|------------|
| Initial (#23) | 432 |
| #37 (.not/.resolves chains) | 350 |
| #38 (PHP Mockery + Python mock) | 350 (TS unchanged) |
| #39 (expect.assertions/unreachable) | 339 |
| **#40 (Chai method-call chains)** | **334** |
