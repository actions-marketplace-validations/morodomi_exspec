---
feature: "Phase 8c-1 — observe failure boundary definition"
cycle: "20260316_1558"
phase: DONE
complexity: standard
test_count: 9
risk_level: low
codex_session_id: ""
created: 2026-03-16 15:58
updated: 2026-03-16 17:00
---

# Cycle: Phase 8c-1 — observe failure boundary definition

## Scope Definition

### In Scope

- 6つの failure boundary (B1〜B6) の境界仕様テストを `observe.rs` に追加
- B5用 dynamic import fixture (`tests/fixtures/typescript/observe/import_dynamic.ts`) の新規作成
- `docs/observe-boundaries.md` — failure boundary の体系的ドキュメント新規作成
- `ROADMAP.md` — Phase 8c-1 結果セクション追加

### Out of Scope

- boundary の修正・改善 (現在の挙動を記録するフェーズ)
- B1〜B4, B6 用の新規 fixture (既存コードで境界をテスト可能)
- observe 本体ロジックの変更

### Files to Change

| File | Change |
|------|--------|
| `crates/lang-typescript/src/observe.rs` | boundary_* テスト 9件追加 |
| `tests/fixtures/typescript/observe/import_dynamic.ts` | 新規: dynamic import fixture |
| `docs/observe-boundaries.md` | 新規: failure boundary ドキュメント |
| `ROADMAP.md` | Phase 8c-1 結果セクション追加 |

## Environment

| Field | Value |
|-------|-------|
| Layer | Rust / test |
| Plugin | lang-typescript |
| Risk | low (5/100) |
| Runtime | Rust + tree-sitter |
| Dependencies | observe.rs の既存テストインフラ |

## Context & Dependencies

- Phase 8b (observe PoC) 完了: NestJS F1 96.3%, typeorm Precision 100%
- observe.rs の既存テスト構造 (`#[cfg(test)] mod tests`) を踏襲
- B1: Issue #85 — re_export.scm が `export * as Ns from` に未対応
- B2: 7/11 FN の主因 — 非相対パスの barrel import は解決不可
- B4: 4/11 FN — 意図的トレードオフ (enum/interface は SUT でない)
- 参照: ROADMAP.md Phase 8c, docs/observe-eval-results.md

## Implementation Notes

### Goal

observe の failure boundary を体系的に定義し、出荷前の applicability scope を確立する研究フェーズ。

### Background

Phase 8b では NestJS monorepo で barrel import resolution を実装し高精度を達成。一方で 6種類の failure boundary が存在することが判明。8c-1 でこれらを fixture + テストで記録し、将来の修正時にアサーション期待値を変更するだけで GREEN に移行可能にする。

### Design Approach

テストは「境界仕様テスト」として設計する:
- 境界の現在の挙動 (FN = false negative, 未解決) を `assert_eq!` で明示的に記録
- テスト名 `boundary_b{N}_*` で統一し、将来のレビュー時に識別しやすくする
- fixture は最小限 (B5用1ファイルのみ) — 既存コードで他境界はテスト可能

## Test List

### TODO

(none)

### WIP

(none)

### DISCOVERED

(none)

### DONE

- [x] TC-01: boundary_b1_ns_reexport_not_captured — `export * as Ns from` は re_export で capture されない
- [x] TC-02: boundary_b1_ns_reexport_mapping_miss — Namespace re-export 経由の import は test-to-code mapping に含まれない
- [x] TC-03: boundary_b2_non_relative_import_skipped — 非相対パス import は resolve 対象外 (skip される)
- [x] TC-04: boundary_b2_cross_pkg_barrel_unresolvable — cross-package barrel は解決不可 (FN として記録)
- [x] TC-05: boundary_b3_tsconfig_alias_not_resolved — `@app/*` path alias は未解決 (FN として記録)
- [x] TC-06: boundary_b4_enum_primary_target_filtered — `.enum.ts` は is_non_sut_helper でフィルタされる
- [x] TC-07: boundary_b4_interface_primary_target_filtered — `.interface.ts` は is_non_sut_helper でフィルタされる
- [x] TC-08: boundary_b5_dynamic_import_not_extracted — dynamic import (`import()`) はテスト関数の import mapping に含まれない
- [x] TC-09: boundary_b6_import_outside_scan_root — scan_root 外のファイルへの import は不可視

## Progress Log

### 2026-03-16 15:58 — Cycle doc 作成 (sync-plan)

Phase 8c-1 plan から Cycle doc を生成。9件の boundary 仕様テストを RED phase で実装予定。

### 2026-03-16 16:30 — GREEN phase 完了

TC-01〜TC-09 全9件を `observe.rs` に実装。全テスト PASS。

**実装上の注意点**: `map_test_files_with_imports` は `FileMapping` エントリ自体は返す（production_files があれば Layer 1 で必ずエントリが生成される）。FN のアサーションは `mappings.is_empty()` ではなく `mappings.iter().flat_map(|m| m.test_files.iter()).collect()` で test_files の空を検証する形に修正。

- fixture 新規作成: `tests/fixtures/typescript/observe/import_dynamic.ts`
- `cargo test`: 全テスト PASS
- `cargo clippy -- -D warnings`: エラー 0
- `cargo fmt --check`: 差分なし
- 自己 dogfooding: BLOCK 0件

### 2026-03-16 17:00 — DONE

docs/observe-boundaries.md 作成、ROADMAP.md に 8c-1 結果と 8c-2 スコープ決定を追記。全品質ゲートクリア。
