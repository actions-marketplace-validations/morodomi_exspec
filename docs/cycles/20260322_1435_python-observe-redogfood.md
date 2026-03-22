---
feature: "Phase 21 — Python observe re-dogfood + FP fix"
cycle: "20260322_1435"
phase: DONE
complexity: standard
test_count: 3
risk_level: low
codex_session_id: ""
created: 2026-03-22 14:35
updated: 2026-03-22 15:15
---

# Cycle: Phase 21 — Python observe re-dogfood + FP fix

## Scope Definition

### In Scope

1. **Re-dogfood 実行**: httpx に対して `exspec observe --lang python --format json` を実行 (正式計測)
2. **GT 突合**: ground truth (`docs/observe-ground-truth-python-httpx.md`) と出力を比較、正確な TP/FP/FN を計算
3. **FP 分析**: 残存 FP を分類。修正対象は `__version__.py` と `_types.py` の2ケースのみ
4. **FP 修正** (GT再分類基準に基づき判定 — 下記参照):
   - GT 再監査: 基準に合致すれば secondary_target として GT を更新
   - コード修正が必要なら TDD サイクルで対応
5. **Requests spot-check**: GT なし。出力のサニティチェックのみ (ship criteria の正式入力としない)
6. **ドキュメント更新**: dogfooding-results.md, STATUS.md, ROADMAP.md

### Out of Scope

- Requests の正式 GT 作成 (別 Phase)
- 新規言語の observe 対応
- Multi-path CLI (別 Phase)
- `[experimental]` マーカー除去判定 (別 Phase — ship criteria 達成後にバックログ追加)

### Scope Boundary Rules

- **修正対象の固定**: FP 修正は `__version__.py` と `_types.py` の2ケースのみ。実測で別の FP が発見された場合は分析・記録のみ行い、修正は新 cycle に切り出す
- **停止条件**: 実測 FP が推定 (4件) より 3件以上多い場合 (7件+)、Step 4 を別 Phase に分割する
- **P>=98% 未達時**: GT audit + scoped fix 後に P<98% なら、残課題をバックログに記録して Phase 終了

### GT 再分類基準

secondary_target に追加して良い条件 (両方を満たすこと):

1. **失敗責務テスト**: そのファイルの振る舞いが壊れた場合にテストが失敗する (incidental な型参照や文字列定数は不可)
2. **assertion 対象**: テスト内の assertion が当該ファイル定義の振る舞い・値を直接検証している

不可のケース:
- 型アノテーションのみで import している → FP (コード修正で対応)
- assertion 文字列内に incidental に出現する → FP (コード修正 or フィルタ強化)

### P/R 計算方法

- **Precision**: pair 単位。TP / (TP + FP)。1つの test→prod マッピングペアを1カウント
- **Recall**: test file 単位。「何らかの正しいマッピングを持つ test file 数 / GT に存在する test file 数」

### Files to Change

- `crates/lang-python/src/observe.rs` (FP 修正が必要な場合)
- `docs/dogfooding-results.md` (計測結果更新)
- `docs/observe-ground-truth-python-httpx.md` (GT 再監査で変更がある場合)
- `docs/STATUS.md` (Phase 21 追加)
- `ROADMAP.md` (Next: 完了項目を Completed Recently へ移送、未完項目は残留)

## Environment

### Scope

- Layer: `crates/lang-python/src/observe.rs`, `docs/`
- Plugin: N/A (Rust crate, cargo test)
- Risk: 25/100 (PASS)
- Runtime: Rust 1.88.0
- Dependencies: tree-sitter (既存)

## Context & Dependencies

### Background

Phase 13-20 で Python observe に多数の改善を実施:
- Phase 13: L1 `_` prefix strip, `src/` layout detection
- Phase 14-15: L2 barrel import resolution, bare import handling
- Phase 16-18: Precision improvement (attribute access filter, stem-only fallback, barrel suppression)
- Phase 19: Assertion-referenced import filter
- Phase 20: Test helper exclusion (tests/ directory path segment check)

Phase 12 時点: httpx P=66.7%, R=6.2%
Phase 20 推定: httpx P=~94%, R=96.8%; Requests P=~100%, R=100%

Ship criteria (CONSTITUTION Section 8): P>=98%, R>=90%

### Upstream References

- CONSTITUTION.md Section 8: Ship criteria P>=98%, R>=90%
- ROADMAP.md: Next セクションは古い (P0/P1 完了済み)。完了項目を Completed Recently へ移送

### Review Findings (plan-review)

Socrates, design-reviewer, codex の3レビューで以下を反映:
- GT 再分類基準を明文化 (codex BLOCK #3, socrates #2)
- スコープ停止条件を追加 (codex BLOCK #2)
- Requests を spot-check に格下げ (codex BLOCK #1)
- 回帰テストを Test List に追加 (codex WARN #6)
- ROADMAP 更新は移送方式に変更 (codex WARN #5)

## Approach

### Step 0: ROADMAP 整理 (前処理)

ROADMAP.md Next セクションの完了済み P0/P1 項目を Completed Recently へ移送。Phase 21 の目的を明示。

### Step 1: Re-dogfood 実行

httpx をクローン (or 既存)、`exspec observe --lang python --format json` を実行。
Requests も同様に実行 (spot-check のみ)。

### Step 2: GT 突合 & P/R/F1 計算

httpx 出力を `docs/observe-ground-truth-python-httpx.md` と突合。
- TP: primary_targets + secondary_targets に含まれるマッピングペア
- FP: GT にないマッピングペア
- FN: GT にあるが出力にないマッピングペア (primary_targets のみ)
- Precision = TP / (TP + FP) (pair 単位)
- Recall = matched_test_files / total_gt_test_files (test file 単位)

### Step 3: FP 分析 & 修正方針決定

GT 再分類基準に基づき各 FP を判定:
1. `__version__.py` — 失敗責務テスト + assertion 対象か確認
2. `_types.py` — 型アノテーションのみなら FP → コード修正

**停止条件チェック**: 実測 FP が 7件以上なら Step 4 を別 Phase に分割。

### Step 4: TDD (コード修正が必要な場合)

RED → GREEN → REFACTOR → 再 dogfood で P>=98% 確認。
P<98% なら残課題をバックログに記録して Phase 終了。

### Step 5: ドキュメント更新

dogfooding-results.md (Phase 21 セクション), STATUS.md (Phase 21 追加), ROADMAP.md

## Test List

### 計測

- [ ] httpx re-dogfood: P>=98%, R>=90% (pair/test file 単位)
- [ ] Requests spot-check: 出力のサニティチェック

### FP 修正テスト (必要な場合に追加)

- [ ] `__version__.py` FP 再現テスト (GT 再分類の場合は不要)
- [ ] `_types.py` type-annotation-only FP 再現テスト

### 回帰テスト (既存テストで確認)

- [ ] barrel import 解決が正常動作 (既存 PY-SUP-* テスト)
- [ ] cross-directory stem-only fallback が正常動作 (既存 PY-L1X-* テスト)
- [ ] `src/` layout detection が正常動作 (既存テスト)
- [ ] test helper exclusion が正常動作 (既存 PY-HELPER-* テスト)

## Verification

1. `cargo test` — 全テスト PASS (回帰テスト含む)
2. `cargo clippy -- -D warnings` — 0 errors
3. `cargo run -- --lang rust .` — BLOCK 0件
4. httpx: P>=98%, R>=90% (正式計測)
5. Requests: spot-check PASS

## Progress Log

- 2026-03-22 14:35: Cycle doc created from plan
- 2026-03-22 14:42: Plan-review findings 反映 (socrates + design-reviewer + codex)
