---
feature: File Mapping Layer 1 (Filename Convention)
phase: DONE
complexity: medium
test_count: 10
risk_level: low
created: 2026-03-15
updated: 2026-03-15
---

# File Mapping Layer 1 (Filename Convention)

Phase 8b Task 3a: ファイル名規約による静的テスト→プロダクションコードマッピング。
同一ディレクトリ内の `.spec.ts` / `.test.ts` パターンで対応関係を検出する。

## Test List

- [x] FM1: basic_spec_mapping - `.spec.ts` suffix match
- [x] FM2: test_suffix_mapping - `.test.ts` suffix match
- [x] FM3: multiple_test_files - both `.spec.ts` and `.test.ts` match
- [x] FM4: nestjs_controller - nested dir `users/users.controller.spec.ts`
- [x] FM5: no_matching_test - unmatched prod file → empty test_files
- [x] FM6: different_directory_no_match - cross-dir → no match (Layer 1)
- [x] FM7: empty_input - empty slices → empty Vec
- [x] FM8: tsx_files - `.tsx` extension support
- [x] FM9: unmatched_test_ignored - orphan test not in any mapping
- [x] FM10: stem_extraction - production_stem / test_stem helpers

## Progress Log

### RED Phase
- Started: 2026-03-15
- Completed: 2026-03-15

### GREEN Phase
- Implemented `FileMapping` / `MappingStrategy`
- Added `TypeScriptExtractor::map_test_files`
- Added `production_stem` / `test_stem`

### REFACTOR Phase
- `cargo clippy -- -D warnings`: passed
- `cargo fmt --check`: passed

### Self-Dogfooding
- `cargo run -- --lang rust .`: BLOCK 0 confirmed
