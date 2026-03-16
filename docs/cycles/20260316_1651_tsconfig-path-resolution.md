---
feature: "Phase 8c-3 — tsconfig path resolution"
cycle: "20260316_1651"
phase: COMMIT
complexity: standard
test_count: 17
risk_level: low
codex_session_id: ""
created: 2026-03-16 16:51
updated: 2026-03-16 16:51
---

# Cycle: Phase 8c-3 — tsconfig path resolution

## Scope Definition

### In Scope

- `tsconfig.json` の `compilerOptions.paths` + `baseUrl` パース
- path alias を絶対パスに変換し、既存の `resolve_import_path` と同等のファイル解決を実行
- `extends` チェーン (ファイルパスのみ、npm パッケージは無視)
- tsconfig 自動発見 (scan_root から上方探索)

### Out of Scope

- B2 (node_modules/workspace resolution) — Phase 対象外
- B4 (context-aware enum filter) — Phase 対象外
- CLI オプション追加
- JSON5 対応 (Phase 1 は標準 JSON のみ)
- extends で npm パッケージを参照するケース (無視)

### Files to Change

| File | Change |
|------|--------|
| `crates/lang-typescript/src/tsconfig.rs` | 新規: TsconfigPaths, PathAlias, discover, parse, resolve |
| `crates/lang-typescript/src/lib.rs` | `pub mod tsconfig;` 追加 |
| `crates/lang-typescript/src/observe.rs` | map_test_files_with_imports にエイリアス import 解決を追加 |
| `crates/lang-typescript/Cargo.toml` | `serde`, `serde_json` 依存追加 |
| `tests/fixtures/typescript/observe/tsconfig_alias/` | 新規: fixture ファイル群 |
| `docs/observe-boundaries.md` | B3 を "Resolved in 8c-3" に更新 |
| `ROADMAP.md` | 8c-3 完了記録 |
| `docs/STATUS.md` | Phase 更新 |

## Environment

| Field | Value |
|-------|-------|
| Layer | Rust / lang-typescript crate |
| Plugin | lang-typescript |
| Risk | low (10/100) |
| Runtime | Rust + serde_json |
| Dependencies | serde, serde_json (新規追加), observe.rs の既存テストインフラ |

### Risk Interview

- Risk type: JSON5 tsconfig への対応漏れ
- JSON5 (コメント付き tsconfig) はどう扱うか: Phase 1 は標準 JSON のみ対応。JSON5 パースエラー時は TsconfigPaths を空として扱い、フォールバック動作を維持する
- extends で npm パッケージを参照するケースは: 無視する (ファイルパスのみ処理)

## Context & Dependencies

### Reference Documents

- `docs/observe-boundaries.md` — B3 の失敗境界定義 (本 Cycle で "Resolved" に変更)
- `ROADMAP.md` — Phase 8c 全体方針と 8c-3 スコープ
- `crates/lang-typescript/src/observe.rs` — 既存の `resolve_import_path`, `map_test_files_with_imports` 実装

### Dependent Features

- Phase 8c-1 (boundary 仕様テスト): `boundary_b3_tsconfig_alias_not_resolved` が本 Cycle で FN → TP に遷移
- Phase 8c-2 (observe MVP): helper_import_filtering と共存する前提

### Related Issues/PRs

- Phase 8c-3: tsconfig path alias (B3 解消)

## Test List

### TODO

#### tsconfig.rs ユニットテスト

- [ ] TP-01: `{"compilerOptions":{"baseUrl":".","paths":{"@app/*":["src/*"]}}}` を `TsconfigPaths::from_str()` でパースすると 1 alias, prefix=`@app/`, targets=[("src/","")] が得られる
- [ ] TP-02: paths に複数ターゲット `["src/*","lib/*"]` をパースすると targets.len() == 2 になる
- [ ] TP-03: baseUrl 省略時にパースすると base_url == tsconfig_dir になる
- [ ] TP-04: ワイルドカードなし `{"@config":["src/config/index"]}` で `resolve_alias("@config")` を呼ぶと exact match で解決される
- [ ] TP-05: extends チェーン (base に paths あり) をパースすると base の paths が継承される
- [ ] TP-06: extends で子が上書きする場合、子の paths が有効になる
- [ ] TP-07: scan_root の親に tsconfig.json がある場合 `discover_tsconfig()` で親の tsconfig を発見できる
- [ ] TP-08: tsconfig が不存在の場合 `discover_tsconfig()` が None を返す
- [ ] TP-09: `resolve_alias("lodash")` を alias `@app/*` のみの設定で呼ぶと None が返る
- [ ] TP-10: `resolve_alias("@app/services/foo")` を alias `@app/*->src/*` で呼ぶと `base_url/src/services/foo` が返る

#### observe.rs 統合テスト

- [ ] OB-01: tsconfig `@app/*->src/*` があり test が `@app/foo.service` を import し src/foo.service.ts が存在する場合、map_test_files_with_imports で foo.service.ts にマッピングされる
- [ ] OB-02: tsconfig なしで test が `@app/foo.service` を import する場合、map_test_files_with_imports でマッピングなし (既存動作維持)
- [ ] OB-03: tsconfig `@app/*->src/*` があり test が `@app/services` (barrel) を import し src/services/index.ts が存在する場合、barrel 経由で解決される
- [ ] OB-04: 相対 import と alias import が共存する場合、map_test_files_with_imports で両方解決される
- [ ] OB-05: tsconfig `@app/*->src/*` があり test が `@app/constants` を import する場合、src/constants.ts は is_non_sut_helper でフィルタされる
- [ ] OB-06: tsconfig `@app/*->src/*` があり test が `@app/nonexistent` を import する場合、map_test_files_with_imports でマッピングなし (エラーなし)

#### 境界テスト更新

- [ ] B3-update: boundary_b3_tsconfig_alias_not_resolved テストを tsconfig ありの環境でマッピング成功 (FN → TP に遷移) するよう更新

### WIP

(none)

### DISCOVERED

(none)

### DONE

(none)

## Implementation Notes

### Goal

observe Layer 2 の `extract_imports()` が非相対パスをフィルタしていることで生じる B3 (tsconfig path alias) の FN を解消する。`@app/*` 等のパスエイリアスを import tracing で解決可能にし、NestJS 評価で 7/11 FN の内 B3 起因分を TP に移行させる。

### Background

observe Layer 2 は `extract_imports()` (observe.rs:709) で非相対パス (`./`/`../` 以外) をフィルタする。これが B2 (cross-package barrel) と B3 (tsconfig path alias) の FN の根本原因。NestJS 評価で 7/11 FN がこのフィルタに起因する。8c-3 では B3 を解消対象とし、B2 (node_modules/workspace) は引き続き対象外とする。

### Design Approach

- `crates/lang-typescript/src/tsconfig.rs` を新規作成し、`TsconfigPaths` 構造体に `discover`, `from_file`, `from_str`, `resolve_alias` を実装
- `resolve_alias` は prefix マッチ (ワイルドカードあり) と exact マッチ (ワイルドカードなし) の両方に対応
- `extends` チェーンはファイルパスのみ再帰的に解決し、npm パッケージ参照は無視
- `map_test_files_with_imports` の import 解決ループで alias import を検出し、`resolve_alias` を呼んで絶対パスに変換してから既存の `resolve_import_path` に渡す
- JSON パースエラー (JSON5 等) は `None` として扱い、フォールバックで既存動作を維持

## Progress Log

### 2026-03-16 16:51 — Cycle doc 作成 (sync-plan)

Phase 8c-3 plan から Cycle doc を生成。17件のテスト (TP-01〜TP-10, OB-01〜OB-06, B3-update) を RED phase で実装予定。

---

## Next Steps

1. [Done] INIT
2. [Done] PLAN
3. [Done] RED
4. [Done] GREEN
5. [Done] REFACTOR
6. [Done] REVIEW
7. [Done] COMMIT
