---
feature: "Task 7.5 — observe Layer 2 追加フィルタ (enum/interface/exception/test-util)"
phase: commit
complexity: S
test_count: 6
risk_level: low
created: 2026-03-16
updated: 2026-03-16
---

# Cycle: observe Layer 2 追加フィルタ (enum/interface/exception/test-util)

## Objective

Task 7 で constants.ts/index.ts フィルタを実装し FP 68→36 (Precision 66.3%→78.8%)。残FP 36件のうちファイル名パターンでフィルタ可能な22件を追加除外し、Precision 78.8%→91.4% を目指す。

## FP Data Basis (from observe-eval-results.md)

| Category | Count | Example |
|----------|-------|---------|
| enum | 7 | `request-method.enum.ts`, `route-paramtypes.enum.ts`, `version-type.enum.ts` |
| interface | 8 | `middleware-configuration.interface.ts`, `nest-middleware.interface.ts` |
| exception | 6 | `unknown-module.exception.ts`, `circular-dependency.exception.ts` |
| test-util | 1 | `packages/core/test/utils/string.cleaner.ts` |
| other (secondary dep) | 14 | フィルタ不可。Task 8 dual metrics で対処 |
| **Total filtered** | **22** | |

## Design Decisions

- **フィルタ箇所**: 既存 `is_non_sut_helper()` 関数を拡張 (Task 7 と同一箇所)
- **suffix match**: `*.enum.ts`, `*.interface.ts`, `*.exception.ts` を file_stem の suffix で検出
- **test path フィルタ**: `/test/` or `/__tests__/` をパスに含むファイルを除外 (1件のみ、リスク低)
- **false positive 防止**: `enum.ts`, `interface.ts`, `exception.ts` (suffix ではなくファイル名そのもの) は除外しない

## Test List

- [x] HELPER-06: `*.enum.ts` → true (`src/enums/request-method.enum.ts`)
- [x] HELPER-07: `*.interface.ts` → true (`src/interfaces/middleware-configuration.interface.ts`)
- [x] HELPER-08: `*.exception.ts` → true (`src/errors/unknown-module.exception.ts`)
- [x] HELPER-09: test path 内ファイル → true (`packages/core/test/utils/string.cleaner.ts`)
- [x] HELPER-10: suffix-like だが異なるファイル → false (`src/enum.ts`, `src/interface.ts`, `src/exception.ts`)
- [x] HELPER-11: 拡張子バリエーション (.js/.tsx/.jsx) → true (`src/foo.enum.js`, `src/bar.interface.tsx`)

## Progress Log

### RED phase (2026-03-16)
HELPER-06〜11 を `crates/lang-typescript/src/observe.rs` L2135〜2197 に追加。
新規5件 (HELPER-06/07/08/09/11) がFAILED、HELPER-10 (rejects_plain_filename) はPASSED (false を返すため正常)。
clippy: 0 errors, fmt: clean。RED state confirmed。

### GREEN phase (2026-03-16)
is_non_sut_helper() を3段階ロジックに拡張: test path filter → exact match → suffix match。
全154 lang-typescript テスト通過。

### REFACTOR phase (2026-03-16)
doc comment 更新。Verification Gate 全通過。

### REVIEW phase (2026-03-16)
correctness-reviewer: PASS (score 42)。
Critical: `/test/` 文字列マッチを segment-based マッチ (`split('/')`) に修正。
HELPER-09 に `contest` 偽陽性防止テストと `__tests__` バリアントを追加。
