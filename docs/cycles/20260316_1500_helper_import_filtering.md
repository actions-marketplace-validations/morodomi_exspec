---
feature: "Task 7 — Helper/Non-SUT Import Filtering (constants.ts, index.ts)"
phase: done
complexity: S
test_count: 5
risk_level: low
created: 2026-03-16
updated: 2026-03-16
---

# Cycle: Helper/Non-SUT Import Filtering

## Objective

Phase 8b Task 7. observe precision evaluation (Task 6) で Precision 66.3%、FP 68件を確認。FP分析の結果、安全にフィルタ可能なのは constants.ts (25件) と index.ts (7件) の計32件。ファイル名ベースでLayer 2のimport tracingからフィルタし、Precision 66.3%→78.8%、F1 72.8%→79.7%を目指す。

## Design Decisions

- **フィルタ箇所**: `map_test_files_with_imports()` の Layer 2 ループ内。extract_imports()でフィルタするとバレル展開実装時にインポート情報が失われる
- **検出方法**: ファイル名ベース (`constants.ts`, `index.ts` + .js/.tsx/.jsx)。content-based不要。O(1)文字列比較
- **設定**: ハードコード（v1）。YAGNI。将来 `[observe]` セクション必要時に追加

## Test List

- [ ] HELPER-01: is_non_sut_helper — constants.ts → true
- [ ] HELPER-02: is_non_sut_helper — index.ts → true
- [ ] HELPER-03: is_non_sut_helper — 拡張子バリエーション (.js/.tsx/.jsx) → true
- [ ] HELPER-04: is_non_sut_helper — my-constants.ts, service.ts → false
- [ ] HELPER-05: is_non_sut_helper — constants/app.ts (ディレクトリ名) → false

## Progress Log

### sync-plan (2026-03-16)

- Cycle doc作成

### plan-review (2026-03-16)

- Design review: PASS (score 18)
- Important: index.ts barrel import FNリスク → Recall変化は評価スクリプトで即検証可能。リスク低
- Important: FP件数不一致 (25 vs 26) → 評価スクリプトで確認
- Optional: app.constants.ts等のsuffix matchは対象外 → v1ハードコードの制約として記録
- Phase completed

### RED Phase (2026-03-16)

- 5テスト作成 (HELPER-01〜05)
- RED確認: 3 failed (01, 02, 03), 2 passed (04, 05)
- Phase completed

### GREEN Phase (2026-03-16)

- `is_non_sut_helper()` 実装 (Path::file_name + matches!)
- Layer 2ループにフィルタ挿入 (continue)
- 全テスト通過
- Phase completed

### REFACTOR Phase (2026-03-16)

- 変更が最小限のためリファクタ不要
- Phase completed

### REVIEW Phase (2026-03-16)

- Code review: PASS (score 5)
- Correctness: OK (ロジック正確、テストカバレッジ十分)
- Security: OK (外部入力処理なし)
- 評価結果: Precision 78.8% / Recall 80.7% / F1 79.8% (plan予測と一致)
- Phase completed
