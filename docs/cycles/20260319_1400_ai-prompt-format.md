---
feature: "Phase 17 — ai-prompt output format with fix guidance"
cycle: "20260319_1400"
phase: DONE
complexity: standard
test_count: 8
risk_level: low
codex_session_id: ""
created: 2026-03-19 14:00
updated: 2026-03-19 14:00
---

# Cycle: Phase 17 — ai-prompt output format with fix guidance

## Scope Definition

### In Scope
- `RuleMeta` に `guidance: &'static str` フィールド追加
- 全17ルール (`T001`〜`T110`) の guidance 文字列記述
- `format_ai_prompt()` 実装 (BLOCK/WARN はセクション分け + guidance、INFO は1行表示)
- `OutputFormat::AiPrompt` variant 追加
- CLI `--format` デフォルトを `"terminal"` → `"ai-prompt"` に変更
- `SUPPORTED_FORMATS` に `"ai-prompt"` 追加、match アーム追加

### Out of Scope
- `format_ai_prompt` の国際化 (英語のみ)
- guidance のカスタマイズ (.exspec.toml 経由) — 別 Issue
- `observe` サブコマンドへの ai-prompt 対応 — 別 Issue

### Files to Change
- `crates/core/src/output.rs`
- `crates/cli/src/main.rs`

## Environment

### Scope
- Layer: `crates/core/src/output.rs`, `crates/cli/src/main.rs`
- Plugin: dev-crew:rust-quality (cargo test)
- Risk: 25/100 (WARN)
- Runtime: Rust (cargo test)
- Dependencies: 既存依存のみ (追加なし)

### Risk Interview

(WARN — リスク 25/100)
- `--format` デフォルト変更は breaking change の性質を持つが、`--format terminal` で従来動作を維持可能。後方互換は保たれる。
- `RULE_REGISTRY` への `guidance` フィールド追加はコンパイルエラーを起こす可能性があるが、影響は `output.rs` 内に限定される。
- `format_ai_prompt()` の出力形式はテキスト文字列であり、JSON/SARIF のようなスキーマ制約なし。設計自由度が高い反面、テストでの文字列検証が主となる。
- regression リスク: 既存の `format_terminal`/`format_json`/`format_sarif` に変更なし。SARIF の `RULE_REGISTRY` 参照は `guidance` フィールドを使わないため影響なし。

## Context & Dependencies

### Background

exspec の主要ユーザーは AI agent (Claude Code 等)。現在のデフォルト出力 `terminal` は人間向けの1行メッセージで、AI が「なぜ問題なのか」「どう直すべきか」を理解できない。

AI が自律的にテスト品質を改善できるよう、ルールごとの修正ガイダンス付き `ai-prompt` フォーマットを実装し、デフォルトにする。

### Design Approach

**1. RuleMeta に guidance フィールド追加**

```rust
struct RuleMeta {
    id: &'static str,
    name: &'static str,
    short_description: &'static str,
    guidance: &'static str,  // 追加
}
```

全17ルールに guidance を記述。空文字は AI-FMT-06 でブロックされる。

**2. format_ai_prompt() の出力構造**

```
# exspec ai-prompt report

Score: BLOCK {n} | WARN {n} | INFO {n} | PASS {n}

## BLOCK

### {file}:{line} [{rule}] {message}
{guidance}

## WARN

### {file}:{line} [{rule}] {message}
{guidance}

## INFO

- {file}:{line} [{rule}] {message}
```

- BLOCK / WARN はセクション分けし、guidance を付与
- INFO は1行表示 (guidance なし) — ノイズ削減
- 空 diagnostics の場合はヘッダー + Score のみ

**3. CLI デフォルト変更**

```rust
// before
#[arg(long, default_value = "terminal")]
pub format: String,

// after
#[arg(long, default_value = "ai-prompt")]
pub format: String,
```

`SUPPORTED_FORMATS` (`validate_format` テスト) に `"ai-prompt"` を追加。

**4. 既存コードとの整合性**

- `OutputFormat` enum に `AiPrompt` variant を追加。既存 `Terminal`/`Json`/`Sarif` に並列
- `format_sarif()` は `RULE_REGISTRY` を `short_description` のみで参照 → `guidance` 追加で影響なし
- `format_terminal()`/`format_json()` に変更なし

## Test List

### TODO
(none)

### WIP
- [x] AI-FMT-01: `format_ai_prompt` with BLOCK diagnostic → "## BLOCK" セクションに diagnostic + guidance が含まれる
- [x] AI-FMT-02: `format_ai_prompt` with WARN diagnostic → "## WARN" セクションに diagnostic + guidance が含まれる
- [x] AI-FMT-03: `format_ai_prompt` with INFO diagnostic → 1行表示、guidance なし
- [x] AI-FMT-04: `format_ai_prompt` with mixed severities → BLOCK, WARN, INFO の順にグルーピング
- [x] AI-FMT-05: `format_ai_prompt` with empty diagnostics → ヘッダー + Score のみ
- [x] AI-FMT-06: `RULE_REGISTRY` の全ルールが空でない `guidance` を持つ
- [x] AI-FMT-07: CLI default format が "ai-prompt" (validate_format テスト更新)
- [x] AI-FMT-08: `format_ai_prompt` に Score 行が含まれる

### DISCOVERED
(none)

### DONE
(none)

## Progress Log

### 2026-03-19 — RED phase 完了

全8テストケース (AI-FMT-01〜08) を作成。

- `crates/core/src/output.rs` の mod tests に AI-FMT-01〜06, 08 を追加
  - `format_ai_prompt` 未実装により 7エラー（コンパイルエラー）
  - `RuleMeta.guidance` フィールド未定義によりコンパイルエラー
- `crates/cli/src/main.rs` の mod tests を更新 (AI-FMT-07)
  - `validate_format_ai_prompt_error` → `validate_format_ai_prompt_ok` に書き換え
  - `cli_default_format_is_ai_prompt` テストを追加
  - 両テスト失敗（FAILED）を確認 → RED state verified

### 2026-03-19 14:00 — Cycle doc 作成 (sync-plan)

planファイルから Cycle doc を生成。
AI agent (Claude Code 等) が exspec の出力を直接消費してテスト品質を改善できるよう、
ルールごとの修正ガイダンス付き `ai-prompt` フォーマットを実装するサイクルを開始。
変更対象: `crates/core/src/output.rs` (RuleMeta guidance 追加・format_ai_prompt 実装)、
`crates/cli/src/main.rs` (デフォルト変更)。
test_count: 8 (unit: 8)
Design Review Gate: PASS (スコア 20/100)
