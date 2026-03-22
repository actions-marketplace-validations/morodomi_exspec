---
feature: "Phase 20 — Python observe test helper exclusion"
cycle: "20260322_2243"
phase: RED-DONE
complexity: trivial
test_count: 7
risk_level: low
codex_session_id: ""
created: 2026-03-22 22:43
updated: 2026-03-22 23:10
---

# Cycle: Phase 20 — Python observe test helper exclusion

## Scope Definition

### In Scope

- `is_non_sut_helper()` にパスセグメント判定を追加: `tests/` or `test/` ディレクトリ内の非テストファイルを無条件でヘルパー扱い
- `is_known_production` バイパスより先にパスセグメントチェックを実行 (TypeScript と同パターン)
- unit テスト PY-HELPER-06〜10 + PY-HELPER-04 regression + E2E PY-E2E-HELPER (計7件) 追加
- E2E fixture `tests/fixtures/python/observe/` にテストヘルパー除外シナリオ追加

### Out of Scope

- `discover_files()` (cli/main.rs) の変更 — 言語非依存汎用ロジックへの副作用リスクがある
- `test/` セグメント以外のヘルパーパターン拡張 (Phase 21 以降)
- L1 filename convention の変更

### Files to Change

- `crates/lang-python/src/observe.rs` (is_non_sut_helper 修正 + テスト追加)
- `tests/fixtures/python/observe/` (E2E fixture 追加)

## Environment

### Scope

- Layer: `crates/lang-python/src/observe.rs`
- Plugin: dev-crew:python-quality は不使用 (Rust crate)
- Risk: 15/100 (low)
- Runtime: Rust (cargo test)
- Dependencies: tree-sitter (既存、追加依存なし)

### Risk Interview

(low — リスク 15/100)

- 変更は `is_non_sut_helper` 関数の冒頭に早期リターン1ブロックを追加するのみ。既存ロジックは一切変更しない。
- TypeScript observe に同一パターンの動作実績あり (`crates/lang-typescript/src/observe.rs:1074-1112`)。
- 既存テスト PY-HELPER-01〜05 は全て `tests/` 外のパスを扱うため、本変更では影響なし。regression テスト (PY-HELPER-04) で明示確認する。
- `src/tests.py` のような「テストディレクトリ外に tests を含むファイル名」はパスセグメントチェックで誤除外されないことを PY-HELPER-10 で明示確認する。

## Context & Dependencies

### Background

Phase 18/19 後の re-dogfooding で Recall は目標達成 (httpx 96.8%, Requests 100%) だが、Precision が未達 (httpx ~92%, Requests ~81%)。最大の FP 原因は**テストディレクトリ内の非テストファイル** (`tests/common.py`, `tests/compat.py`, `tests/testserver/server.py` 等) が production file に誤分類されること。

`discover_files()` はファイルを `test_*.py` / `*_test.py` → test file、それ以外の `.py` → production file と分類する。結果、`tests/common.py` 等が production file リストに入り `canonical_to_idx` に登録される。`is_non_sut_helper(path, is_known_production=true)` が呼ばれると即 `false` → ヘルパー判定バイパス。

TypeScript は `is_non_sut_helper` 内でパスセグメントチェックを `is_known_production` に関係なく行っており、この問題が発生しない。

CONSTITUTION 整合性: Section 6 "observe uses multi-layer matching: filename convention (L1) + import tracing (L2)"。本修正は L2 の精度向上であり Non-Goals に抵触しない。

### Design Approach

**`is_non_sut_helper` にパスセグメント判定を冒頭追加**

```rust
pub fn is_non_sut_helper(file_path: &str, is_known_production: bool) -> bool {
    // Phase 20: Path-segment check BEFORE is_known_production bypass.
    // Files inside tests/ or test/ directories that are NOT test files
    // are always helpers, even if they appear in production_files list.
    // (Same pattern as TypeScript observe.)
    let in_test_dir = file_path
        .split('/')
        .any(|seg| seg == "tests" || seg == "test");

    if in_test_dir {
        let file_name = Path::new(file_path)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("");
        // If it's NOT a test file pattern, it's a helper
        if !file_name.starts_with("test_") && !file_name.ends_with("_test.py") {
            return true;
        }
        return false;
    }

    // Original logic below (unchanged)
    if is_known_production {
        return false;
    }
    // ... rest of existing logic
}
```

### Upstream References

- CONSTITUTION.md Section 6: "observe uses multi-layer matching: filename convention (L1) + import tracing (L2)"
- ROADMAP.md: Ship criteria Precision >= 98%, Recall >= 90%
- TypeScript observe `is_non_sut_helper` (`crates/lang-typescript/src/observe.rs:1074-1112`): 同じパスセグメントパターン

### Verification Commands

```bash
cargo test -p exspec-lang-python
cargo test -p exspec
cargo clippy -- -D warnings
cargo fmt --check
cargo run -- --lang rust .
```

## Test List

### TODO

(none)

### WIP

- [x] PY-HELPER-06: tests/common.py -> helper (is_known_production=true でもパスセグメント判定で除外) — FAIL as expected
- [x] PY-HELPER-07: tests/testserver/server.py -> helper (tests/ 配下のサブディレクトリ) — FAIL as expected
- [x] PY-HELPER-08: tests/compat.py -> helper (is_known_production=false) — PASS with current logic (expected)
- [x] PY-HELPER-09: tests/fixtures/data.py -> helper (深いネスト) — FAIL as expected
- [x] PY-HELPER-10: src/tests.py -> NOT helper (ファイル名に tests を含むが tests/ ディレクトリ外) — PASS with current logic (expected)
- [x] PY-HELPER-04 regression: tests/utils.py still helper — verified existing test present
- [x] PY-E2E-HELPER: test helper in tests/ excluded from mappings — FAIL as expected

### DISCOVERED

(none)

### DONE

(none)

## Progress Log

### 2026-03-22 22:47 — Plan Review (Socrates + Codex)

Socrates WARN x3:
1. 既存 `parent_is_test_dir` ブロック (L106-116) が dead code → 削除する
2. `tests/__init__.py` barrel ケース → `__init__.py` は L99-104 で先にマッチ済み。barrel resolution は `is_barrel_file` 経由で `is_non_sut_helper` に来ない。影響なし
3. 到達不能 `false` 分岐 → 除去する。test dir 内の test file は discover_files で振り分け済み

対応: 選択肢2 (既存ロジック統合)。新セグメントチェック追加と同時に冗長な `parent_is_test_dir` ブロックを削除。

### 2026-03-22 22:43 — Cycle doc 作成 (sync-plan)

planファイルから Cycle doc を生成。
Phase 18/19 後の re-dogfooding で Precision 未達の最大原因を特定: `tests/` ディレクトリ内の非テストファイルが production file に誤分類される。
`is_non_sut_helper` にパスセグメント判定を `is_known_production` バイパスより先に実行することで修正 (TypeScript と同パターン)。
変更ファイル: `crates/lang-python/src/observe.rs`
新規 fixture: `tests/fixtures/python/observe/` 内の E2E fixture
test_count: 7 (unit: 6, e2e: 1)
Risk Score: 15 (low)
