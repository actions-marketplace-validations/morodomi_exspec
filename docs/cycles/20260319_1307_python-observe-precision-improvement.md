---
feature: "Phase 18 — Python observe precision improvement"
cycle: "20260319_1307"
phase: DONE
complexity: standard
test_count: 9
risk_level: medium
codex_session_id: ""
created: 2026-03-19 13:07
updated: 2026-03-19 13:07
---

# Cycle: Phase 18 — Python observe precision improvement

## Scope Definition

### In Scope

- stem-only fallback (L1 cross-directory): `map_test_files_with_imports()` に stem-only マッチを追加
- L1-priority barrel suppression: L1 マッチ済みテストへの barrel 追加を抑制
- テスト追加 (unit: PY-L1X-01〜05, PY-SUP-01〜03, e2e: PY-SUP-04)

### Out of Scope

- `test_exported_members.py` → `__init__.py` マッピング (L1 miss 維持、barrel 経由で OK)
- `models/test_queryparams.py` → `_models.py` (L1 miss → barrel 維持で OK)
- L3 semantic duplication detection (CONSTITUTION Non-Goals)

### Files to Change

- `crates/lang-python/src/observe.rs`

## Environment

### Scope

- Layer: `crates/lang-python/src/observe.rs`
- Plugin: dev-crew:python-quality は不使用 (Rust crate)
- Risk: 30/100 (WARN)
- Runtime: Rust (cargo test)
- Dependencies: tree-sitter (既存、追加依存なし)

### Risk Interview

(WARN — リスク 30/100)

- stem-only fallback は production_files 全体をスキャンするため、同一 stem を持つ複数ファイルへの ambiguity マッピングが発生する可能性あり。Recall 優先で全てにマップする設計。
- barrel suppression は L1 済みテストの barrel 追加のみを抑制。direct import (`from httpx._utils import X`) は L1 済みでも追加するため、suppress ロジックの分岐が必要。
- barrel 判定ロジック (`__init__.py` 経由かどうか) は `collect_import_matches` の解決パスを参照する必要がある。既存のアーキテクチャに依存。
- 既存 L1 core マッチ済みの場合に fallback が不発火することを PY-L1X-05 で明示確認。

## Context & Dependencies

### Background

Python observe の httpx dogfooding 結果: **P=26.7%, R=84.4%** (GT比較)。
Ship criteria (P>=98%, R>=90%) に対し精度が大幅に未達。

根本原因:
1. **L1 directory mismatch**: core `map_test_files` は `(directory, stem)` ペアでマッチ。`tests/test_client.py` (dir=tests/) と `httpx/_client.py` (dir=httpx/) → L1 hit = 0
2. **Barrel fan-out**: L1 miss → 全テストが L2 barrel のみで解決。`import httpx` → `__init__.py` → 全 re-export 先にマップ → 1テストあたり 3-8 FP (計74 FP)

### Design Approach

**A. stem-only fallback (L1 cross-directory)**

`map_test_files_with_imports()` (observe.rs:600) 内で、core L1 の後に stem-only マッチを追加。

- production_files 全体から `production_stem()` → stem の逆引き map を構築
- L1 未マッチのテストファイルについて stem-only で production file を探す
- 同一 stem が複数 prod file にマッチする場合は全てにマップ (recall 優先)
- L1 core で既にマッチ済みなら fallback しない (重複防止)

**B. L1-priority barrel suppression**

L1 マッチ済みテストの barrel 追加を抑制。

- L1 (core + fallback) でマッチ済みのテストファイル set を構築
- `matched_indices` への追加時 (observe.rs:720-724)、テストが L1 済みかつ import が barrel 経由 (`__init__.py` 解決) なら skip
- direct import (`from httpx._utils import X` → `_utils.py` に直接解決) は L1 済みでも追加

**barrel 判定**: `collect_import_matches` が `__init__.py` を経由して barrel chain を辿った場合 = barrel。specifier が直接 `.py` ファイルに解決された場合 = direct。

**C. 残存 FN**

- `test_exported_members.py` → `__init__.py`: L1 miss (stem不一致) → barrel 維持
- `models/test_queryparams.py` → `_models.py`: L1 miss (stem不一致) → barrel 維持 → OK

### Verification Commands

```bash
cargo test -p exspec-lang-python
cargo test -p exspec
cargo clippy -- -D warnings
cargo fmt --check
cargo run -- --lang rust .
cargo run -- observe --lang python /tmp/httpx
```

## Test List

### TODO

(none)

### WIP

- [ ] PY-L1X-01: stem-only fallback: `tests/test_client.py` → `pkg/_client.py` (cross-directory)
- [ ] PY-L1X-02: stem-only: `tests/test_decoders.py` → `pkg/_decoders.py` (_ prefix prod)
- [ ] PY-L1X-03: stem-only: `tests/test_asgi.py` → `pkg/transports/asgi.py` (サブディレクトリ)
- [ ] PY-L1X-04: stem ambiguity: 同一 stem の prod が複数 → 全てにマップ
- [ ] PY-SUP-01: barrel suppression: L1 済みテストに barrel 追加なし
- [ ] PY-SUP-04: E2E: httpx fixture で FP 大幅削減 (P >= 80%, 中間目標)

### DISCOVERED

- [x] stem collision risk: `models.py`, `utils.py` 等の common name が複数ディレクトリにある場合、stem-only fallback が全てにマップし precision 低下リスク。`prod_indices.len() == 1` ガード検討。→ issue #126
- [x] barrel suppression scope: L1マッチ済みテストが2つの無関係パッケージをテストする場合、2つ目のbarrel importが抑制される FN リスク。per-(test, prod) スコープ検討。→ issue #127

### DONE

- [x] PY-L1X-05: L1 core マッチ済みなら fallback 不発火 (既存動作で PASS)
- [x] PY-SUP-02: barrel suppression: L1 済みでも direct import は追加 (既存動作で PASS)
- [x] PY-SUP-03: barrel suppression: L1 未マッチテストは barrel 通常通り (既存動作で PASS)

## Progress Log

### 2026-03-19 13:07 — Cycle doc 作成 (sync-plan)

planファイルから Cycle doc を生成。
httpx dogfooding で P=26.7% に留まる根本原因として L1 directory mismatch と barrel fan-out を特定。
stem-only fallback と L1-priority barrel suppression の2アプローチで精度改善を図るサイクルを開始。
test_count: 9 (unit: 8, e2e: 1)
Risk Score: 30 (WARN)

### 2026-03-19 13:08 — Plan Review (design-reviewer)

判定: **WARN** (blocking_score: 35)

指摘事項:
1. (important/risk) barrel判定のAPIインターフェース未定義 → 対処: `self.is_barrel_file(&resolved)` を `collect_import_matches` 呼び出し前に判定。core変更不要。
2. (important/upstream) E2E基準 P>=80% と Ship criteria P>=98% の乖離 → 対処: PY-SUP-04 は中間目標。stable ship 判定ではない旨を明記。
3. (optional/scope) stem-only fallback の挿入位置 → L1後、L2ループ前。
4. (optional/over-engineering) PY-L1X-04 stem ambiguity の実例確認 → recall優先の仕様確認テスト。

全指摘対処可能。BLOCK なし → RED phase へ進行。

### 2026-03-19 13:15 — RED phase 完了

9件のテスト作成完了。
- FAIL (RED) 6件: PY-L1X-01〜04, PY-SUP-01, PY-SUP-04 → 実装待ち
- PASS 3件: PY-L1X-05, PY-SUP-02, PY-SUP-03 → 既存動作で満たされる
- 既存200件: 全PASS (リグレッションなし)

設計判断: PY-L1X-01〜03 はimport文なし (L2で解決されない純粋なstem-only fallbackシナリオ)。

### 2026-03-19 13:20 — GREEN phase 完了

全206件テスト PASS。実装内容:
- A. stem-only fallback: L1後、L2前に production_stem の逆引きマップで cross-directory マッチ
- B. barrel suppression: L2の3箇所で `is_barrel_file(&resolved) && l1_matched_tests.contains(test_file)` ガード

### 2026-03-19 13:22 — REFACTOR phase 完了

- `std::collections::HashSet` フルパス使用を import に統一 (DRY)
- barrel suppression 3箇所の重複は AGENTS.md 方針に従い維持 (premature abstraction 回避)
- Verification Gate: 206 tests PASS, clippy 0, fmt clean, BLOCK 0
- Phase completed

### 2026-03-19 13:30 — REVIEW phase 完了

Specialist Panel:
- Security: PASS (score 0) — 新規脆弱性なし
- Performance: PASS (score 22) — optional のみ、O(P+T) 維持
- Correctness: WARN (score 55) — 2件 important

Finding 1 (strategy mislabeling) 修正: `layer1_extended_tests_per_prod` を stem-only fallback 後にスナップショットし strategy 更新で使用。PY-SRCLAYOUT-01/02 のテスト期待値を `FileNameConvention` に更新 (stem-only fallback は L1 の一種)。

Finding 2 (stem collision): Plan で recall 優先と判断済み (PY-L1X-04)。DISCOVERED に記録。
Finding 3 (barrel suppression scope): DISCOVERED に記録。

Verification Gate (修正後): 206 tests PASS, 122 integration tests PASS, clippy 0, fmt clean, BLOCK 0
- Phase completed
