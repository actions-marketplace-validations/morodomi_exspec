# Phase 3C: SARIF Output + ProjectMetrics

## Status: DONE

## Goal
SARIF v2.1.0出力とProjectMetrics実装。MVP最後のフェーズ。
GitHub Code Scanning互換のSARIF出力でOSS公開に必要な出力形式を完備。

## Changes

| Cycle | Content | Tests Added |
|-------|---------|-------------|
| 1 | serde-sarif dep + ProjectMetrics + compute_metrics | 8 |
| 2 | Terminal + JSON metrics display + signature changes | 6 |
| 3 | --format validation (terminal/json/sarif) | 5 |
| 4 | SARIF output (serde-sarif builder API) | 14 |
| 5 | CLI integration (compute_metrics + sarif match arm) | 2 |

## Key Decisions

| # | Decision | Rationale |
|---|----------|-----------|
| 1 | serde-sarif over hand-crafted json! | 型安全、スキーマ自動生成、OSS保守性 (Grok指摘受入) |
| 2 | Old Metrics struct deleted | デッドコード。ProjectMetricsに一本化 |
| 3 | PBT/Contract分母はfile count | has_pbt_importはfile単位bool、関数単位データなし |
| 4 | SARIF内にmetricsなし | SARIF標準外 (Grok指摘受入) |
| 5 | RuleMeta.default_level削除 | 未使用フィールド、clippy dead_code warning回避 |
| 6 | ai-prompt format未サポート | Tier 3 (Phase 6)。validate_formatでreject |

## Architecture

### ProjectMetrics (metrics.rs)
- 7 fields: mock_density_avg, mock_class_max, parameterized_ratio, pbt_ratio, assertion_density_avg, contract_coverage, test_source_ratio
- `compute_metrics(analyses, source_file_count)` with zero-division guards
- `Default` derive for test convenience

### SARIF Output (output.rs)
- serde-sarif 0.8 with opt-builder feature
- RULE_REGISTRY: 8 RuleMeta (id, name, short_description)
- Level mapping: Block->error, Warn->warning, Info->note
- File-level diagnostics (line=None) -> startLine: 1
- Invocations with executionSuccessful: true

### CLI (main.rs)
- validate_format() + SUPPORTED_FORMATS const
- compute_metrics() call before output dispatch
- sarif match arm in format dispatch

## Files Changed

| File | Action |
|------|--------|
| crates/core/Cargo.toml | serde-sarif 0.8 dependency |
| crates/core/src/metrics.rs | Old Metrics deleted, ProjectMetrics + compute_metrics |
| crates/core/src/output.rs | format_sarif + RULE_REGISTRY + metrics display |
| crates/cli/Cargo.toml | serde_json dev-dep |
| crates/cli/src/main.rs | validate_format + compute_metrics + sarif arm |

## Quality

| Metric | Value |
|--------|-------|
| Tests | 203 passing (168 + 35 new) |
| Clippy | 0 errors |
| Format | No diff |

## Review

- Security: PASS (score 5) - 入力検証OK、依存安全、panic safety確認
- Correctness: PASS (score 15) - SARIF準拠、zero-division guard、aggregation正確
- Discovered items -> GitHub issue #5
