---
feature: "Phase 8c-4 — context-aware enum/interface filter (B4)"
cycle: "20260316_1736"
phase: DONE
complexity: standard
test_count: 10
risk_level: low
codex_session_id: ""
created: 2026-03-16 17:36
updated: 2026-03-16 18:10
---

# Cycle: Phase 8c-4 — context-aware enum/interface filter (B4)

## Scope Definition

### In Scope

- `is_non_sut_helper` に `is_known_production` パラメータ追加
- suffix filter (enum/interface/exception) を known production file では bypass
- exact-match filter (constants, index) と test-path filter は変更なし
- `is_type_definition_file` として suffix check を独立関数に抽出
- 3 call sites の signature 更新
- B4 boundary テストを FN → TP に反転
- 新規ユニットテスト 8 件追加

### Out of Scope

- barrel 解決パス (line 1247) の production-aware 化
- B2 (cross-package barrel)
- B1 (namespace re-export)

### Files to Change

| File | Change |
|------|--------|
| `crates/lang-typescript/src/observe.rs` | `is_type_definition_file` 抽出、`is_non_sut_helper` signature 変更、3 call sites 更新、既存テスト signature 更新 (~20箇所)、B4 テスト assertion 反転、新規テスト 8 件 |
| `docs/observe-boundaries.md` | B4 を "Resolved in 8c-4" に更新 |
| `ROADMAP.md` | 8c-4 完了記録 |
| `docs/STATUS.md` | Phase 更新 |

## Environment

| Field | Value |
|-------|-------|
| Layer | Rust / lang-typescript crate |
| Plugin | lang-typescript |
| Risk | low (5/100) |
| Runtime | Rust + tree-sitter |
| Dependencies | observe.rs の既存テストインフラ、crates/lang-typescript |

### Risk Interview

- Risk type: OB-05 regression (constants.ts)
- constants.ts は exact-match フィルタで処理されるため: suffix check とは独立コードパスであり、`is_known_production=true` を渡しても exact-match による true は変わらない
- BARREL-08 regression は: barrel パス (line 1247) は `false` を渡す設計。動作変更なし
- barrel + enum edge case: 文書化して defer

## Context & Dependencies

### Reference Documents

- `docs/observe-boundaries.md` — B4 の失敗境界定義 (本 Cycle で "Resolved" に変更)
- `ROADMAP.md` — Phase 8c 全体方針と 8c-4 スコープ
- `crates/lang-typescript/src/observe.rs` — `is_non_sut_helper` の既存実装 (line 1073)

### Dependent Features

- Phase 8c-1 (boundary 仕様テスト): `boundary_b4_*` テストが本 Cycle で FN → TP に遷移
- Phase 8c-2 (observe MVP): helper_import_filtering と共存する前提
- Phase 8c-3 (tsconfig path resolution): `collect_matches` の call sites に新パラメータを追加

### Related Issues/PRs

- Phase 8c-4: context-aware enum/interface filter (B4 解消)

## Test List

### TODO

(none)

### WIP

(none)

### DISCOVERED

(none)

### DONE

#### is_type_definition_file ユニットテスト

- [x] TD-01: `src/foo.enum.ts` を `is_type_definition_file()` に渡すと true が返る
- [x] TD-02: `src/bar.interface.ts` を `is_type_definition_file()` に渡すと true が返る
- [x] TD-03: `src/baz.exception.ts` を `is_type_definition_file()` に渡すと true が返る
- [x] TD-04: `src/service.ts` を `is_type_definition_file()` に渡すと false が返る
- [x] TD-05: `src/constants.ts` を `is_type_definition_file()` に渡すと false が返る

#### is_non_sut_helper (production-aware) ユニットテスト

- [x] PA-01: `src/foo.enum.ts` かつ `known_production=true` で `is_non_sut_helper()` が false を返す (bypass)
- [x] PA-02: `src/foo.enum.ts` かつ `known_production=false` で `is_non_sut_helper()` が true を返す (filtered)
- [x] PA-03: `src/constants.ts` かつ `known_production=true` で `is_non_sut_helper()` が true を返す (still filtered)

#### B4 boundary テスト (FN → TP 反転)

- [x] B4-enum: `route-paramtypes.enum.ts` が production_files に含まれており、`route.spec.ts` がそれを import する場合、`map_test_files_with_imports` でマッピング成功 (FN → TP)
- [x] B4-interface: `user.interface.ts` が production_files に含まれており、`user.spec.ts` がそれを import する場合、`map_test_files_with_imports` でマッピング成功 (FN → TP)

## Implementation Notes

### Goal

observe の `is_non_sut_helper` が `*.enum.*`, `*.interface.*`, `*.exception.*` を無条件フィルタしている問題 (B4) を解消する。NestJS 評価で 4/11 FN がこのフィルタに起因しており、production_files に含まれる enum/interface ファイルへのマッピングを許可することで TP に移行させる。

### Background

observe.rs line 1073 の `is_non_sut_helper` は suffix check で enum/interface/exception ファイルを helper とみなし、mapping から除外する。これは型定義系ファイルが多くの場合テスト対象でないという経験則に基づくが、NestJS 評価では `route-paramtypes.enum.ts` や `user.interface.ts` が本来の SUT (production_files に列挙済み) であるケースで FN を引き起こしている。8c-4 では production_files に含まれる場合のみ suffix filter を bypass し、最小限の変更で FN を解消する。

### Design Approach

- `is_type_definition_file(file_path: &str) -> bool`: 現在の suffix check ロジック (lines 1103-1113) を独立関数として抽出
- `is_non_sut_helper(file_path: &str, is_known_production: bool) -> bool`: `is_known_production=true` のとき suffix check をスキップ
- 3 call sites:
  - line 933 (barrel branch): `canonical_to_idx.contains_key(&prod_str)` で production 判定
  - line 939 (direct branch): `canonical_to_idx.contains_key(resolved)` で production 判定
  - line 1247 (resolve_barrel_exports_inner): `false` を渡す (動作変更なし)
- 既存テスト (~20箇所) の signature を更新し、デフォルト `false` で回帰なしを確認

## Progress Log

### 2026-03-16 17:36 — Cycle doc 作成 (sync-plan)

Phase 8c-4 plan から Cycle doc を生成。10件のテスト (TD-01〜TD-05, PA-01〜PA-03, B4-enum, B4-interface) を RED phase で実装予定。

### 2026-03-16 — RED phase 完了

テストコード記述完了。RED状態確認済み。

- TD-01〜TD-05: `is_type_definition_file` 未実装のため E0425 (5件)
- PA-01〜PA-03 + HELPER-01〜11 (既存): `is_non_sut_helper` シグネチャ不一致 E0061 (29件)
- B4-enum, B4-interface: assertion 反転済み（GREEN phase でシグネチャ更新後に検証）
- 合計 34 コンパイルエラー。意図通りの RED 状態。

### 2026-03-16 18:00 — GREEN phase 完了

最小限の実装で全テストをパス。

- `is_type_definition_file` 独立関数として抽出 (line 1066 前に追加)
- `is_non_sut_helper` に `is_known_production: bool` パラメータ追加、suffix check を `is_type_definition_file` 呼び出しに置換
- 3 call sites 更新: barrel branch (line 933) と direct branch (line 939) は `canonical_to_idx.contains_key` で production 判定、line 1247 は `false` を渡す
- `cargo test`: 全クレート PASS (835 件)
- `cargo clippy -- -D warnings`: エラー 0
- `cargo fmt --check`: 差分なし
- `cargo run -- --lang rust .`: BLOCK 0 | WARN 0 | INFO 7

### 2026-03-16 — REFACTOR phase

コード品質レビュー実施。重大なリファクタリング不要と判断。`is_type_definition_file` / `is_non_sut_helper` は既存パターン (`is_barrel_file` 等) と一貫しており、単一責務を維持。

### 2026-03-16 — REVIEW phase 完了

Design Review: PASS (blocking_score: 12)。

- スコープ妥当性: B4 解消のみに限定。YAGNI 違反なし
- アーキテクチャ整合性: 既存の純粋関数 + bool フラグパターンと一貫
- barrel path `false` 判断: canonical_to_idx アクセス不可のため妥当。edge case は文書化済み
- 指摘事項: barrel branch のキー形式整合性コメント追加は optional (defer)

### 2026-03-16 — COMMIT phase 完了

コミット `3985520` で実装済み。全 835 テスト PASS、clippy/fmt/self-dogfooding 全クリア。

---

## Next Steps

1. [Done] INIT
2. [Done] PLAN
3. [Done] RED
4. [Done] GREEN
5. [Done] REFACTOR
6. [Done] REVIEW
7. [Done] COMMIT
