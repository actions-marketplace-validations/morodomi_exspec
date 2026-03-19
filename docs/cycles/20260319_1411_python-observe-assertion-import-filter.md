---
feature: "Phase 19 — Python observe assertion-referenced import filter"
cycle: "20260319_1411"
phase: REVIEW
complexity: standard
test_count: 10
risk_level: medium
codex_session_id: ""
created: 2026-03-19 14:11
updated: 2026-03-19 14:30
---

# Cycle: Phase 19 — Python observe assertion-referenced import filter

## Scope Definition

### In Scope

- `extract_assertion_referenced_imports()` 実装: assertion byte ranges 内の identifier を収集し、import symbol まで逆追跡
- `assignment_mapping.scm` 新規作成: 代入パターン capture (var = ClassName(), var = obj.method() 等)
- assignment tracking + chain tracking (2-hop max) による `asserted_imports` 構築
- L2 `matched_indices` を `asserted_matched` / `all_matched` に分離し safe fallback 実装
- テスト追加 (unit: PY-AF-01〜07, e2e: PY-AF-08〜10)

### Out of Scope

- L3 semantic duplication detection (CONSTITUTION Non-Goals)
- 3-hop 以上の chain tracking (complexity vs. gain 不均衡)
- assertion filter の L1 への適用 (L1 は filename convention ベース、filter 不要)

### Files to Change

- `crates/lang-python/queries/assignment_mapping.scm` (新規)
- `crates/lang-python/src/observe.rs`

## Environment

### Scope

- Layer: `crates/lang-python/queries/`, `crates/lang-python/src/observe.rs`
- Plugin: dev-crew:python-quality は不使用 (Rust crate)
- Risk: 45/100 (WARN)
- Runtime: Rust (cargo test)
- Dependencies: tree-sitter (既存、追加依存なし)

### Risk Interview

(WARN — リスク 45/100)

- `asserted_matched` が空の場合の safe fallback が正しく動作しないと、assertion のないテストファイルが全 L2 マッチを失う FN リスクがある。PY-AF-06 で明示確認。
- assignment tracking は同名変数の再代入 (shadowing) を追跡しない。実用上は問題ないが、複雑なテストでは asserted_imports の過大評価が起きる可能性あり。
- chain tracking 2-hop の実装は `idx_to_symbols` の構築順序に依存する。ループ順序の誤りで chain が未解決になるリスクあり。
- `assertion.scm` の byte ranges 取得は既存クエリを再利用するため、assertion クエリのカバレッジに依存する。未検出 assert は filter に影響しない (保守的)。

## Context & Dependencies

### Background

Phase 18 で stem-only fallback + barrel suppression を実装し P=26.7% → 43.3%、R=90.6% に改善。
残存 FP 35件の大半は L2 import tracing の「incidental import」— テストが import するが assert 対象ではないモジュール (MockTransport, _exceptions, _urls 等)。

CONSTITUTION 整合性: assert 内のシンボル追跡は structural AST analysis であり、semantic validation (テスト名 vs 振る舞い判定) ではない。Non-Goals に抵触しない。

### Design Approach

**assertion-referenced import filter (safe fallback 付き)**

L2 ループで `matched_indices` を収集した後、assertion context で参照されるシンボルのみに絞り込む。

**フロー**:
1. テストソースから assertion byte ranges を取得 (既存 `assertion.scm`)
2. assertion 範囲内の全 identifier を収集 → `assertion_identifiers`
3. assignment tracking: `var = ImportedClass()` → var→class マッピング
4. chain tracking: `response = client.get()` → response→client → client→Class (2-hop max)
5. assertion_identifiers から import symbol まで逆追跡 → `asserted_imports: HashSet<String>`
6. L2 matched_indices を2つに分離:
   - `asserted_matched`: import symbols が asserted_imports と交差する prod indices
   - `all_matched`: 全 L2 マッチ (従来通り)
7. **Safe fallback**: `asserted_matched` が非空なら使用、空なら `all_matched` にフォールバック

**assignment_mapping.scm (新規 query)**

```scheme
;; var = ClassName(...)
(assignment left: (identifier) @var right: (call function: (identifier) @class))
;; var = module.ClassName(...)
(assignment left: (identifier) @var right: (call function: (attribute attribute: (identifier) @class)))
;; var = obj.method(...) → var derives from obj
(assignment left: (identifier) @var right: (call function: (attribute object: (identifier) @source)))
;; var = await ClassName(...)
(assignment left: (identifier) @var right: (await (call function: (identifier) @class)))
```

**assertion 内 identifier 収集**

新規 query 不要。`assertion.scm` で byte ranges を取得し、AST walk で identifier を収集。

**idx_to_symbols tracking**

L2 ループ内で `collect_import_matches` 呼び出し前後の `matched_indices` 差分を取り、新規追加された idx に import symbols を紐付ける。
`idx_to_symbols: HashMap<usize, HashSet<String>>` を構築。

### Verification Commands

```bash
cargo test -p exspec-lang-python
cargo test -p exspec
cargo clippy -- -D warnings
cargo fmt --check
cargo run -- --lang rust .
cargo run -- observe --lang python /tmp/httpx   # P/R 計測
```

## Test List

### TODO

(none)

### DONE (RED phase)

- [x] PY-AF-01: assertion filter: `client = Client(); assert client.ok` → Client in asserted_imports
- [x] PY-AF-02: assertion filter: `transport = MockTransport()` (not in assert) → MockTransport NOT in asserted_imports
- [x] PY-AF-03: assertion filter: direct usage `assert A() == B()` → both A, B in asserted_imports
- [x] PY-AF-04: assertion filter: `pytest.raises(HTTPError)` → HTTPError in asserted_imports
- [x] PY-AF-05: chain tracking: `response = client.get(); assert response.ok` → Client reachable (2-hop)
- [x] PY-AF-06a: no assertions → asserted_imports empty → fallback to all_matched (safe)
- [x] PY-AF-06b: assertions exist but no asserted imports intersect with L2 → fallback to all_matched
- [x] PY-AF-07: unittest: `self.assertEqual(result.value, 42)` → result's import captured
- [x] PY-AF-08: E2E integration: primary import kept, incidental filtered
- [x] PY-AF-09: E2E: ALL imports incidental → fallback, no regression
- [x] PY-AF-10: E2E: third_party_http_client fixture, FP 削減確認

### WIP

(none)

### DISCOVERED

- [ ] Performance: `extract_assertion_referenced_imports` が毎回 Parser を再生成。L2 ループ先頭で tree を共有すべき
- [ ] Performance: `all_matched.clone()` が import ごとに発生。`collect_import_matches` の戻り値拡張で回避可能
- [ ] Maintainability: observe.rs が 2600+ 行の God File。routes 抽出部を `routes.rs` に分離候補
- [ ] Correctness: `assignment_mapping.scm` Pattern 2/3 が `var = module.Class()` で二重マッチ。実害は限定的だが排他制御を検討
- [ ] Test: PY-AF-03 の識別子 `A`/`B` が1文字で将来の誤マッチリスク。`ModelA`/`ModelB` に改名推奨

### DONE

(none)

## Progress Log

### 2026-03-19 — REVIEW phase 完了

- Mode: code, Risk: HIGH (90)
- Panel: security(8) + correctness(22) + test(18) + maintainability(32) + performance(45) = max 45 → PASS
- Correctness fix: bare relative import loop で `&import.symbols` → `&[sym.clone()]` に修正 (symbols tracking 精度向上)
- PY-AF-10 fixture コメント修正 (pytest.raises の asserted_imports 挙動を正確に記述)
- DISCOVERED: parser re-instantiation, all_matched.clone() per import, observe.rs God File, assignment_mapping.scm 二重マッチ
- Phase completed

### 2026-03-19 — REFACTOR phase 完了

- `track_new_matches` ヘルパー関数抽出 (3箇所の重複パターン解消)
- `cargo fmt` 適用
- Verification Gate: テスト 217+122 全PASS, clippy 0, fmt OK, BLOCK 0
- Phase completed

### 2026-03-19 14:30 — RED phase 完了

テスト10件 (unit: 7 + 06分割で実質8, e2e: 3) を `crates/lang-python/src/observe.rs` に追加。
`extract_assertion_referenced_imports` 関数が未実装のため8件の E0425 コンパイルエラーで RED 状態を確認。

変更ファイル:
- `crates/lang-python/src/observe.rs` — PY-AF-01〜10 テスト追加
- `tests/fixtures/python/observe/af_pkg/` — PY-AF-08 fixture (Client + MockTransport)
- `tests/fixtures/python/observe/af_e2e_fallback/` — PY-AF-09 fixture (all incidental fallback)
- `tests/fixtures/python/observe/af_e2e_http/` — PY-AF-10 fixture (third_party_http_client)

設計レビューフィードバック反映:
- PY-AF-06 を 06a (no assertions) + 06b (no intersection) に分割
- PY-AF-10 fixture 名を汎用名 `third_party_http_client` / `af_e2e_http` に変更 (OSS 公開対応)
- `extract_assertion_referenced_imports` は standalone 関数として設計 (source: &str -> HashSet<String>)

### 2026-03-19 14:11 — Cycle doc 作成 (sync-plan)

planファイルから Cycle doc を生成。
Phase 18 で P=43.3%, R=90.6% に改善後、残存 FP 35件の大半が incidental import であることを特定。
assertion-referenced import filter + safe fallback で L2 精度をさらに向上させるサイクルを開始。
新規ファイル: `assignment_mapping.scm`
変更ファイル: `observe.rs`
test_count: 10 (unit: 7, e2e: 3)
Risk Score: 45 (WARN)
