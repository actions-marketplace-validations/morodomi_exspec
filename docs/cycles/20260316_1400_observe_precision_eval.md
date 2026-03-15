---
feature: "#82 NestJS Precision Verification (observe ground truth comparison)"
phase: done
complexity: M
test_count: 6
risk_level: low
created: 2026-03-16
updated: 2026-03-16
---

# Cycle: Observe Precision Evaluation

## Objective

Phase 8b Task 6. exspec observeの出力をhand-audited ground truth (166 primary mappings) と比較し、precision/recall/F1を算出する。observeの現在の能力を定量化し、改善優先度を決定する。

## Design Decisions

- **Pythonスクリプト** (`scripts/evaluate_observe.py`): 使い捨て評価コードをRustコアに入れない
- **Pairwise (test_file, production_file) ペア単位評価**
- **secondary_targets は評価対象外 (ignored)**: observeがsecondaryに含まれるファイルを出力してもTP/FPどちらにもカウントしない
- **パス変換**: observe出力のabsolute pathからscan_rootプレフィックスを除去してrelative化

## Test List

- [ ] GT-EVAL-01: ground truth JSON パース (primary/secondary分離)
- [ ] GT-EVAL-02: observe JSON パース (production_file→test_files 逆変換)
- [ ] GT-EVAL-03: パス正規化 (absolute→relative, 拡張子補完)
- [ ] GT-EVAL-04: TP/FP/FN/ignored カウント
- [ ] GT-EVAL-05: precision/recall/F1 計算 (ゼロ除算ガード)
- [ ] GT-EVAL-06: stratum別 breakdown (direct_import vs barrel_import)

## Scope

| In | Out |
|----|-----|
| file_mappings評価 | routes評価 (NestJSはフレームワーク) |
| Pythonスクリプト + pytest | Rustコア変更 |
| Markdown結果出力 | CLI統合 |

## Progress Log

- 2026-03-16 14:00 sync-plan: Cycle doc created
