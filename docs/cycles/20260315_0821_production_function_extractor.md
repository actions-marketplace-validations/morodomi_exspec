---
feature: observe-production-function-extractor
cycle: 8b-task1
phase: DONE
complexity: complex
test_count: 10
risk_level: low
created: 2026-03-15 08:21
updated: 2026-03-15 08:22
---

# Phase 8b Task 1: Production Function Extractor (TypeScript)

## Scope Definition

### In Scope
- [ ] `ProductionFunction` 型定義 (`observe.rs`)
- [ ] `extract_production_functions()` メソッド実装
- [ ] `production_function.scm` tree-sitter query (5パターン)
- [ ] Fixture 5ファイル作成
- [ ] テスト10個 (Given/When/Then)
- [ ] `lib.rs` に `pub mod observe;` 追加

### Out of Scope
- CLI 変更 (Reason: Task 5 で対応)
- ObserveExtractor trait (Reason: YAGNI, PoC段階では TypeScript のみ)
- 他言語対応 (Reason: PoC は TypeScript のみ)

### Files to Change (target: 10 or less)
- crates/lang-typescript/src/observe.rs (new)
- crates/lang-typescript/src/lib.rs (edit: `pub mod observe;` 追加)
- crates/lang-typescript/queries/production_function.scm (new)
- tests/fixtures/typescript/observe/exported_functions.ts (new)
- tests/fixtures/typescript/observe/class_methods.ts (new)
- tests/fixtures/typescript/observe/arrow_exports.ts (new)
- tests/fixtures/typescript/observe/mixed.ts (new)
- tests/fixtures/typescript/observe/nestjs_controller.ts (new)

### Design Decisions
- **impl vs trait**: impl を選択。PoC段階でポリモーフィズム不要。YAGNI。
- **全関数抽出 + is_exported フラグ**: テストが内部関数を参照するケースにも対応可能。
- **重複排除**: exported/non-exported 両マッチを行番号 Set で排除。

## Test List

| # | テスト名 | Phase |
|---|---------|-------|
| 1 | `exported_functions_extracted` | RED |
| 2 | `non_exported_function_has_flag_false` | RED |
| 3 | `class_methods_with_class_name` | RED |
| 4 | `exported_class_is_exported` | RED |
| 5 | `arrow_exports_extracted` | RED |
| 6 | `non_exported_arrow_flag_false` | RED |
| 7 | `mixed_file_all_types` | RED |
| 8 | `decorated_methods_extracted` | RED |
| 9 | `line_numbers_correct` | RED |
| 10 | `empty_source_returns_empty` | RED |

## Progress Log

### 2026-03-15 08:21 - Cycle created
- Plan: zany-wobbling-deer.md (Phase 8b Task 1)
- Plan review: PASS (軽微な指摘2件、修正不要)

### 2026-03-15 08:22 - RED
- Complexity: complex (10 items)
- observe.rs: ProductionFunction型 + スタブメソッド + テスト10個作成
- Fixture 5ファイル作成 (exported_functions, class_methods, arrow_exports, mixed, nestjs_controller)
- lib.rs に `pub mod observe;` 追加
- 9 failed, 1 passed (empty_source_returns_empty はスタブで通過 - 期待通り)
- Phase completed

### 2026-03-15 08:23 - GREEN
- production_function.scm: 5パターン (exported func, non-exported func, method, exported arrow, non-exported arrow)
- observe.rs: extract_production_functions 実装 (OnceLock + cached_query + QueryCursor + 行番号Set重複排除)
- find_class_info(): 親ノード探索で class_name + is_exported 判定
- 全726テスト通過、clippy 0、fmt差分なし、self-dogfooding BLOCK 0
- Phase completed

### 2026-03-15 08:24 - REFACTOR
- チェックリスト7項目確認: リファクタリング不要
- Verification Gate: PASS (tests 10/10, clippy 0, fmt OK)
- Phase completed

### 2026-03-15 08:26 - REVIEW
- Mode: code, Risk: LOW
- security-reviewer: PASS (score 5) - optional 2件 (.unwrap()→.expect(), ネスト関数精度)
- correctness-reviewer: WARN (score 62) - CRITICAL 2件修正済み:
  - CRITICAL-1: マッチ順序依存の重複排除 → HashMap + is_exported 上書き方式に変更
  - CRITICAL-2: 行番号基準不一致 → name ノード基準に統一
- clippy if_same_then_else 修正 (exported/non-exported パターンをOR結合)
- .unwrap() → .expect() 修正 (security-reviewer optional 指摘)
- 全テスト通過、clippy 0、fmt OK
- Phase completed

### 2026-03-15 08:27 - COMMIT
- Phase completed
