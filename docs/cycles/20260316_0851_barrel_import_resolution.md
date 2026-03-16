---
feature: "Task 8b — Barrel Import Resolution (同一パッケージ内)"
cycle: "20260316_0851"
phase: DONE
complexity: standard
test_count: 10
risk_level: low
codex_mode: "no"
codex_session_id: ""
created: "2026-03-16 08:51"
updated: "2026-03-16 08:51"
---

# Cycle: Barrel Import Resolution (同一パッケージ内)

## Objective

Task 7.5 で observe Layer 2 の helper フィルタを強化し Precision 90.3% を達成した。
残る課題は barrel import (`index.ts` re-export) 経由の import がプロダクションファイルにマッチしない問題。
本サイクルでは同一パッケージ内の相対パス barrel import を解決し、Recall の改善を図る。

## Scope Definition

### In Scope

- `resolve_import_path()` への `index.ts` フォールバック追加 (ディレクトリ指定を index.ts に解決)
- `re_export.scm` クエリ新規作成 (named re-export / wildcard re-export)
- `resolve_barrel_exports()` 関数実装 (最大3ホップ、循環防止付き)
- Layer 2 ループへの barrel 解決統合
- `extract_imports()` の symbol 情報保持

### Out of Scope

- node_modules / 外部パッケージの barrel 解決
- TypeScript path alias (`@/`, `~/`) の解決
- 3ホップ超の深い barrel チェーン
- `.js` 拡張子 barrel ファイル (`.ts` のみ対象)

### Files to Change

- `crates/lang-typescript/src/observe.rs` — 主な変更対象 (resolve_import_path, resolve_barrel_exports, extract_imports, Layer 2 ループ)
- `crates/lang-typescript/queries/re_export.scm` — 新規クエリ
- `crates/lang-typescript/queries/import_mapping.scm` — 参照 (変更なし)
- `scripts/evaluate_observe.py` — 評価実行
- `docs/observe-eval-results.md` — 結果記録

## Environment

| Item | Value |
|------|-------|
| Layer | lang-typescript crate |
| Plugin | observe.rs |
| Risk | 30/100 (LOW) |
| Runtime | Rust / cargo test |
| Dependencies | tree-sitter, existing OnceLock query cache |

## Context & Dependencies

- **前サイクル**: `20260316_2005_helper_filter_extension.md` (Task 7.5, is_non_sut_helper 拡張)
- **Layer 2 実装**: `20260315_1555_import_tracing_layer2.md` (extract_imports, resolve_import_path 既存実装)
- **評価基盤**: `docs/observe-eval-results.md`, `scripts/evaluate_observe.py`
- **参照クエリ**: `import_mapping.scm` (既存 import 抽出パターン)

## Implementation Notes

### Goal

barrel import (`import { Foo } from './services'` → `./services/index.ts` → `./services/foo.service.ts`) を
Layer 2 import tracing で解決し、observe の Recall を改善する。

### Design Approach

| 決定事項 | 方針 |
|---------|------|
| re-export 優先 | named re-export 優先。wildcard は保守的（全 re-export 先を解決対象に含める） |
| 循環防止 | `HashSet<PathBuf>` で visited 管理 |
| 深さ制限 | 最大3ホップ |
| フィルタ分離 | barrel 解決パスでは中間の index.ts に `is_non_sut_helper` フィルタを適用しない |
| index.ts フォールバック | `resolve_import_path()` でディレクトリ指定時に `<dir>/index.ts` を試みる |

### Implementation Steps

1. `resolve_import_path()` に index.ts フォールバック追加
2. `re_export.scm` クエリ作成 (named re-export / wildcard re-export)
3. `resolve_barrel_exports()` 実装 (visited HashSet + 最大3ホップ)
4. Layer 2 ループ修正 (barrel 解決を統合)
5. `extract_imports()` の symbol 情報保持

## Test List

### TODO

- [x] EVAL: nestjs/nest で Recall 改善を検証 → Recall 93.4% (目標達成), Precision 15.5% (wildcard FP爆増)

### WIP

(none)

### DISCOVERED

- [x] namespace re-export (`export * as Ns from './module'`) が re_export.scm で未対応 → issue #85

### DONE

- [x] BARREL-01: resolve_import_path がディレクトリの index.ts にフォールバックする
- [x] BARREL-02: re_export.scm が named re-export をキャプチャする
- [x] BARREL-03: re_export.scm が wildcard re-export をキャプチャする
- [x] BARREL-04: resolve_barrel_exports が 1ホップのバレルを解決する
- [x] BARREL-05: resolve_barrel_exports が 2ホップのバレルを解決する
- [x] BARREL-06: 循環バレルで無限ループしない
- [x] BARREL-07: Layer 2 で barrel 経由の import が production file にマッチする
- [x] BARREL-08: is_non_sut_helper フィルタが barrel 解決後のファイルに適用される
- [x] BARREL-09: extract_imports が symbol 名を保持する

## Progress Log (追記)

### EVAL (2026-03-16)

nestjs/nest で observe 精度評価を実施。

| Metric | Before (7.5) | After (8b) | Target |
|--------|-------------|------------|--------|
| Recall | 78.3% | 93.4% | 90%+ |
| Precision | 90.3% | 15.5% | 90%+ |
| FN | 36 | 11 | 14以下 |
| FP | 14 | 847 | -- |

**Recall 目標達成**。FN 36→11 (25件解消)。barrel_import 32件中25件を TP に転換。

**Precision 急落の原因**: wildcard re-export (`export * from '...'`) が barrel 内の全ファイルを
解決対象に含め、1テストファイルが数十のプロダクションファイルにマッチ。
例: `apply-decorators.spec.ts` → `../../decorators` barrel → 全 decorator.ts に展開 (20+ FP)。

**次タスク**: wildcard re-export の FP 対策。symbol フィルタ強化または barrel ファンアウト上限の導入。

### REVIEW Phase (2026-03-16)

Specialist Panel: security (8/PASS), correctness (42/PASS), performance (62/WARN)。
Codex competitive review: 使用量上限で失敗、Claude レビューのみで続行。

**Accept (修正済み)**:
- resolve_barrel_exports_inner に extractor と canonical_root を外から渡す (Performance)
- BARREL-09 テストの symbols 内容アサーション強化 (Correctness)
- symbols.is_empty() の意図をコメントで明示 (Correctness)

**DISCOVERED**: namespace re-export 未対応 (re_export.scm)

修正後 Verification Gate: 163 tests PASS, clippy 0, fmt clean, BLOCK 0。

### REFACTOR Phase (2026-03-16)

- `is_barrel_file()` ヘルパー抽出: index.ts/index.tsx 判定を3箇所から集約
- `resolve_barrel_exports_inner()` の wildcard/named 分岐の重複ロジックを統合 (130行→60行)
- `MAX_BARREL_DEPTH` 定数化
- `BTreeMap` → `HashMap` 統一
- Verification Gate PASS: 163 tests, clippy 0, fmt clean, BLOCK 0

### RED Phase (2026-03-16)

BARREL-01〜09 のテストを `observe.rs` の `#[cfg(test)] mod tests` に追加。

**RED 確認結果**: `cargo test -p exspec-lang-typescript` で 6件のコンパイルエラー。

```
error[E0599]: no method named `extract_barrel_re_exports` (BARREL-02, 03)
error[E0425]: cannot find function `resolve_barrel_exports` (BARREL-04, 05, 06)
error[E0609]: no field `symbols` on type `&observe::ImportMapping` (BARREL-09)
```

BARREL-01, 07, 08 は論理的に失敗（バレル解決未実装）。全テスト RED 状態確認済み。

**作成ファイル:**
- `crates/lang-typescript/queries/re_export.scm` (空stub、GREEN で実装)
- `crates/lang-typescript/src/observe.rs` (テスト9件追加)

## Progress Log

### sync-plan (2026-03-16 08:51)

Cycle doc 作成完了。planファイルから Test List 10件を転記。
次フェーズ: plan-review → RED。

### Plan Review (2026-03-16 08:52)

Design review 完了。verdict=WARN (score 35)。
- important #1: ImportMapping に symbols フィールド追加で後方互換維持
- important #2: Layer 2 ループの barrel 解決分岐を明示
- important #3: resolve_barrel_exports のシグネチャ確定
全て RED フェーズで対応可能。BLOCK なし。
