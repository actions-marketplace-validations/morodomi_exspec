---
feature: "#131 L1 exclusive mode for observe"
phase: green
complexity: M
test_count: 0
risk_level: medium
created: 2026-03-24T09:00:00Z
updated: 2026-03-24T09:00:00Z
---

# #131 L1 exclusive mode for observe

## Summary

observe の L2 (import tracing) が L1 (filename convention) でマッチ済みのテストファイルに対しても追加マッピングを生成し、FP が発生する。httpx dogfooding で ~25 FP の原因。L1 マッチ済みテストは L2 を全面抑制する opt-in モードを追加する。

## Implementation Notes

- CLI: `--l1-exclusive` フラグ追加 (ObserveArgs)
- 4言語の `map_test_files_with_imports()` に `l1_exclusive: bool` パラメータ追加
- L2 ループの先頭で `l1_exclusive && layer1_matched.contains(test_file)` なら skip
- TypeScript: `layer1_matched` (HashSet<String>)
- Python: `l1_matched_tests` (HashSet<String>)  — `.contains(test_file.as_str())`
- Rust: `apply_l2_imports` に `layer1_matched: &HashSet<String>` を追加
- PHP: `layer1_tests_per_prod` から `layer1_matched` を構築して使用
- 既存テスト全呼び出しに `false` を追加
- 新規テスト: TypeScript で L1 exclusive テスト追加

## Progress Log

- 2026-03-24T09:00:00Z: GREEN phase 開始
