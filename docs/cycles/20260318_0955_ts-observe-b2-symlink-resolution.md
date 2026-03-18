---
feature: "B2 — TypeScript observe: monorepo symlink resolution"
cycle: "20260318_0955"
phase: RED
complexity: standard
test_count: 6
risk_level: medium
codex_session_id: ""
created: 2026-03-18 09:55
updated: 2026-03-18 09:55
---

# Cycle: B2 — TypeScript observe: monorepo symlink resolution

## Scope Definition

### In Scope
- [ ] `crates/lang-typescript/src/observe.rs` — `resolve_node_modules_symlink` 新関数追加
- [ ] `crates/lang-typescript/src/observe.rs` — `resolve_base_to_file_unchecked` ヘルパー追加
- [ ] `crates/lang-typescript/src/observe.rs` — Layer 2c ブロック (`map_test_files_with_imports` L955 挿入点)
- [ ] `crates/lang-typescript/src/observe.rs` — boundary_b2 テスト反転
- [ ] `docs/observe-boundaries.md` — B2 ステータス更新

### Out of Scope
- 他言語 observe への影響
- npm (非 symlink) の実ディレクトリ解決
- package.json main/exports パース
- Windows 環境対応 (Unix only)
- B6 境界の変更

### Files to Change (target: 10 or less)
- `crates/lang-typescript/src/observe.rs` (edit)
- `docs/observe-boundaries.md` (edit)

## Environment

### Scope
- Layer: Backend (`crates/lang-typescript` のみ)
- Plugin: dev-crew:rust-quality (cargo test / clippy / fmt)
- Risk: 35/100 (WARN) — ファイルシステム探索あり、テストで symlink 作成が必要

### Runtime
- Language: Rust (cargo test)

### Dependencies (key packages)
- tree-sitter: 既存
- tree-sitter-typescript: 既存
- std::fs: symlink_metadata / canonicalize

### Risk Interview (BLOCK only)

(BLOCK なし — リスク 35/100 WARN)

## Context & Dependencies

### Reference Documents
- `crates/lang-typescript/src/observe.rs` L674 — `extract_imports_impl` (相対パスフィルタ)
- `crates/lang-typescript/src/observe.rs` L941-955 — `map_test_files_with_imports` Layer 2c 挿入点
- `crates/lang-typescript/src/observe.rs` L3046-3116 — boundary_b2 テスト (反転対象)
- `crates/core/src/observe.rs` L165-208 — `resolve_absolute_base_to_file` (参考、変更なし)
- `crates/core/src/observe.rs` L188 — `canonical.starts_with(canonical_root)` チェック (緩和対象)
- `docs/observe-boundaries.md` — B2 境界定義

### Dependent Features
- TypeScript observe import tracing: `map_test_files_with_imports`
- tsconfig path resolution: Layer 2b (tsconfig alias が優先)
- barrel import resolution: 既存コードを再利用

### Related Issues/PRs
- (none)

## Test List

### TODO
- [ ] TS-B2-SYM-01: symlink を follow して実体パスを返す
- [ ] TS-B2-SYM-02: 非 symlink (実ディレクトリ) は None を返す
- [ ] TS-B2-SYM-03: 存在しない specifier は None を返す
- [ ] TS-B2-MAP-01: symlink 経由の cross-package mapping が TP になる
- [ ] TS-B2-MAP-02: tsconfig alias が優先される (symlink fallback しない)
- [ ] TS-B2-CACHE-01: 同じ specifier の2回目はキャッシュヒット

### WIP
(none)

### DISCOVERED
(none)

### DONE
(none)

## Implementation Notes

### Goal
TypeScript observe の NestJS eval における 7/11 FN の主因 B2 (cross-package barrel import) を解決する。yarn/pnpm workspace の node_modules symlink を follow して production file にマッピングする Layer 2c を実装する。

### Background
`import { Foo } from '@nestjs/common'` のような非相対パスは `extract_imports_impl` (L674) でフィルタされ、tsconfig alias にもマッチしない場合は解決不能になる。yarn/pnpm workspace では `node_modules/@org/common` → `../../packages/common` の symlink が作られる。この symlink を follow することで cross-package の production file にマッピングできる。

### Design Approach

既存パイプラインの構造:

```
Layer 2a: extract_imports_impl (相対パスのみ) → resolve_import_path → barrel/canonical lookup
Layer 2b: extract_all_import_specifiers (非相対パスのみ) → tsconfig resolve_alias → resolve_absolute_base_to_file → barrel/canonical lookup
Layer 2c: [NEW] tsconfig で未解決の非相対パス → node_modules symlink follow → barrel/canonical lookup
```

**新関数: `resolve_node_modules_symlink`**

```rust
fn resolve_node_modules_symlink(
    specifier: &str,       // "@org/common"
    scan_root: &Path,      // packages/core
    cache: &mut HashMap<String, Option<PathBuf>>,
) -> Option<PathBuf>
```

1. キャッシュチェック (同じ specifier の2回目以降はスキップ)
2. `scan_root/node_modules/{specifier}` のパスを構築
3. `std::fs::symlink_metadata()` で symlink かチェック
4. symlink なら `std::fs::canonicalize()` で実体パスを取得
5. 非 symlink なら None (npm install の実ディレクトリは対象外)

**新ヘルパー: `resolve_base_to_file_unchecked`**

core の `resolve_absolute_base_to_file` (L188) は `canonical.starts_with(canonical_root)` でフィルタする。symlink 先は scan_root 外になるため、canonical_root チェックなしのローカルヘルパーを追加。`canonical_to_idx` membership で production file かどうかを検証する。

**Layer 2c 統合 (`map_test_files_with_imports` L955 以降)**

```rust
// Layer 2c: node_modules symlink resolution (monorepo)
for (specifier, symbols) in &alias_imports {
    // tsconfig alias で解決済みなら skip
    if tsconfig_paths.as_ref().and_then(|tc| tc.resolve_alias(specifier)).is_some() {
        continue;
    }
    // node_modules symlink follow
    if let Some(resolved_dir) = resolve_node_modules_symlink(specifier, scan_root, &mut nm_cache) {
        if let Some(resolved) = resolve_base_to_file_unchecked(self, &resolved_dir) {
            collect_matches(&resolved, symbols, &mut matched_indices);
        }
    }
}
```

**設計判断**:
- scope 制約: `production_files` に cross-package のファイルが含まれていない場合は解決不可 (B6 と同じ "by design")
- npm (非 symlink) は対象外: symlink_metadata で判別
- package.json main/exports パースは MVP ではスキップ (`src/index.ts` barrel へのフォールバック)
- Unix only: `std::os::unix::fs::symlink` を使うテストは `#[cfg(unix)]` ガード

## Progress Log

### 2026-03-18 09:55 - INIT
- Cycle doc created
- Scope definition ready
