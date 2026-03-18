---
feature: "9a — ObserveExtractor: collect_import_matches 共通化 + CLI 重複排除"
cycle: "20260318_1041"
phase: DONE
complexity: standard
test_count: 8
risk_level: low
codex_session_id: ""
created: 2026-03-18 10:41
updated: 2026-03-18 10:41
---

# Cycle: 9a — ObserveExtractor: collect_import_matches 共通化 + CLI 重複排除

## Scope Definition

### In Scope
- [x] `crates/core/src/observe.rs` — `collect_import_matches` free function 追加
- [x] `crates/lang-typescript/src/observe.rs` — inline closure を core 呼び出しに置換
- [x] `crates/lang-python/src/observe.rs` — local `collect_import_matches` 削除 → core 呼び出し
- [x] `crates/lang-rust/src/observe.rs` — local `collect_import_matches` 削除 → core 呼び出し
- [x] `crates/cli/src/main.rs` — `run_observe()` 共通化 (~200行 → ヘルパー関数抽出)

### Out of Scope
- PHP observe への変更 (barrel 非対応のため `collect_import_matches` 不使用)
- 新機能追加 (純粋リファクタリングのみ)
- 他言語への observe 機能拡張

### Files to Change (target: 10 or less)
- `crates/core/src/observe.rs` (edit)
- `crates/lang-typescript/src/observe.rs` (edit)
- `crates/lang-python/src/observe.rs` (edit)
- `crates/lang-rust/src/observe.rs` (edit)
- `crates/cli/src/main.rs` (edit)

## Environment

### Scope
- Layer: Backend (core + 3 lang crates + cli)
- Plugin: dev-crew:rust-quality (cargo test / clippy / fmt)
- Risk: 25/100 (PASS) — 純粋リファクタリング、既存テスト全通過が検証基準

### Runtime
- Language: Rust (cargo test)

### Dependencies (key packages)
- `crates/core/src/observe.rs` — `ObserveExtractor` trait, `is_barrel_file`, `resolve_barrel_exports`, `is_non_sut_helper`
- `std::collections::{HashMap, HashSet}`
- `std::path::Path`

### Risk Interview (BLOCK only)

(BLOCK なし — リスク 25/100 PASS)

## Context & Dependencies

### Reference Documents
- `crates/core/src/observe.rs` L54-89 — `ObserveExtractor` trait (既存)
- `crates/lang-python/src/observe.rs` L637-666 — Python `collect_import_matches` (削除対象)
- `crates/lang-rust/src/observe.rs` L728-757 — Rust `collect_import_matches` (削除対象)
- `crates/lang-typescript/src/observe.rs` L904-931 — TypeScript inline closure (置換対象)
- `crates/cli/src/main.rs` L333-541 — `run_observe()` 4言語分 (~200行、共通化対象)

### Dependent Features
- observe: `map_test_files_with_imports` (全3言語で `collect_import_matches` を使用)
- observe CLI: `run_observe()` 経由で全言語の observe を起動

### Related Issues/PRs
- (none)

## Test List

### TODO
(none)

### WIP
(none)

### DISCOVERED
(none)

### DONE
- [x] CORE-CIM-01: barrel file 経由の production match
- [x] CORE-CIM-02: 非 barrel file の直接 match
- [x] CORE-CIM-03: helper file はスキップ
- [x] CORE-CIM-04: canonical_to_idx に存在しない file はスキップ
- [x] REFACTOR-TS-01: TypeScript observe 全既存テスト通過
- [x] REFACTOR-PY-01: Python observe 全既存テスト通過
- [x] REFACTOR-RS-01: Rust observe 全既存テスト通過
- [x] REFACTOR-CLI-01: CLI observe 動作維持

## Implementation Notes

### Goal
`collect_import_matches` の3重実装 (Python/Rust/TypeScript) を `crates/core/src/observe.rs` の free function に統一し、CLI `run_observe()` の ~200行重複を共通ヘルパーで削減する。純粋リファクタリングであり、外部動作は変わらない。

### Background
ROADMAP Phase 9a は「ObserveExtractor trait を TypeScript から抽出」が当初目標だったが、trait は既に `crates/core/src/observe.rs` L54-89 に存在し全4言語が実装済み。残存する重複コードは:
1. `collect_import_matches` が Python/Rust/TypeScript で同一ロジックを3回実装 (PHP は barrel 非対応で不使用)
2. CLI の `run_observe()` が4言語分ほぼ同じ「discover → read → map → report」を繰り返し (L333-541, ~200行)

### Design Approach

**Step 1: `collect_import_matches` を core に抽出**

```rust
pub fn collect_import_matches(
    ext: &dyn ObserveExtractor,
    resolved: &str,
    symbols: &[String],
    canonical_to_idx: &HashMap<String, usize>,
    indices: &mut HashSet<usize>,
    canonical_root: &Path,
)
```

ロジック:
1. `is_barrel_file` → `resolve_barrel_exports` → `is_non_sut_helper` → `canonical_to_idx.get`
2. 非 barrel → `is_non_sut_helper` → `canonical_to_idx.get`

各言語の `collect_import_matches` / inline closure を削除し、core の関数を呼び出す。

**Step 2: CLI `run_observe()` の共通化**

```rust
fn run_observe_common(
    root: &str,
    lang_str: &str,
    lang: Language,
    format: &str,
    config: &Config,
    extractor_fn: impl FnOnce(&[String], &HashMap<String, String>, &Path) -> Vec<FileMapping>,
    route_fn: impl FnOnce(&[String]) -> Vec<ObserveRouteEntry>,
) { ... }
```

共通部分: `discover_files` → test_sources HashMap 構築 → `map_test_files_with_imports` → `build_observe_report` + format 出力

## Progress Log

### 2026-03-18 10:41 - INIT
- Cycle doc created
- Scope definition ready

### 2026-03-18 - SYNC-PLAN
- Cycle doc generated from plan
- Phase completed

### 2026-03-18 - REVIEW (plan)
- Score: 22/100 (PASS)
- Important: TypeScript の collect_matches クロージャは Layer 2b/2c でも使用されている (L940, L957, L980)。core 版への置換時にこれらも忘れずに対応すること
- Important: run_observe_common の route_fn 引数は production_files を受け取る。シグネチャに注釈を追加
- Optional: CORE-CIM-01 の barrel テストは MockExtractor in-memory で完結させる（既存 tc01/tc03 と同方針）
- Phase completed

### 2026-03-18 - RED
- CORE-CIM-01~04: 4テスト作成、全コンパイルエラー (E0425: collect_import_matches not found)
- ConfigurableMockExtractor 追加 (is_non_sut_helper, is_barrel_file が設定可能)
- Phase completed

### 2026-03-18 - GREEN
- Step 1: core/observe.rs に pub fn collect_import_matches 追加
- Step 2: Python ローカル関数削除 → core 呼び出し (3箇所)
- Step 3: Rust ローカル関数削除 → core 呼び出し
- Step 4: TypeScript inline closure 削除 → core 呼び出し (3箇所: L2, L2b, L2c)
- 全955テスト PASS, clippy 0 errors, BLOCK 0
- Phase completed

### 2026-03-18 - REFACTOR
- CLI run_observe() を run_observe_common ヘルパーに抽出 (~200行 → ~70行の共通関数 + ~50行の呼び出し)
- map_fn / route_fn closure 注入で TypeScript 固有の route extraction を分離
- Verification Gate: fmt OK, clippy 0, 955 tests PASS, BLOCK 0
- Phase completed

### 2026-03-18 - REVIEW (code)
- Security: PASS (5/100) — 新規脆弱性なし
- Correctness: PASS (8/100) — 論理等価性確認済み、Layer 2b/2c 置換正しい
- Aggregate: PASS (8/100)
- Phase completed

### 2026-03-18 - COMMIT
- refactor: extract collect_import_matches to core and deduplicate CLI run_observe
- Phase completed
