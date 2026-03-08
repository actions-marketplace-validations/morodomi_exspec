# exspec Roadmap v2 (Final)

Gemini / Grok / GPT-5.4 / Claude 4者壁打ち (2026-03-08) で合意。

## 設計原則

1. **exspec は静的解析 lint である**。テンプレート生成器でもドキュメント生成器でもない
2. **1人開発の scope 制約**。並行して2つ以上の大機能を追わない
3. **OSSは出してから育てる**。ただし「lint としての信頼」を壊さない最低限の硬化は必要
4. **AI連携は "exspec が出す → 人間/AI が判断" の分離**。exspec 自体が LLM を呼ばない

## Phase 6: Release Hardening

新ルール追加なし。公開品質への硬化のみ。

### 6-1: Dogfooding

exspec を実プロジェクトで実行し、偽陽性を issue 化。

| 言語 | 対象 | 目的 |
|------|------|------|
| Python | 自分のプロジェクト (Keiba) + fastapi or requests | 大規模テスト群での FP 検出 |
| TypeScript | vitest or zod | モダン TS テストでの FP 検出 |
| PHP | PHPUnit サンプル or Laravel テスト | フレームワーク固有記法の誤爆検出 |
| Rust | exspec 自身 (516テスト) | 自己検証 + token_tree 限界の確認 |

**合格基準**:
- BLOCK/WARN: 偽陽性 0 件 (絶対条件)
- INFO: 全体テスト数の 3% 未満

### 6-2: Severity / Default-on 見直し

各ルールの「精度」と「出力の強さ」の整合性を確認。

| チェック | 基準 |
|---------|------|
| BLOCK は本当に BLOCK か？ | 偽陽性がほぼ 0 でないなら WARN に降格 |
| WARN はヒューリスティックか？ | 精度が 7 割未満なら INFO に降格 |
| default-on にすべきか？ | opinionated なルールは opt-in 検討 |

### 6-3: FP 修正

Dogfooding で見つかった偽陽性を TDD サイクルで修正。

### 6-4: Limitations 明記

README に以下を追加:
- exspec が判定しないものの明示 (意味的正しさ、mock の妥当性、property の質)
- AI 意味的レビューは今後予定
- 既知の制約 (Rust token_tree 等)

### 6-5: 段階的導入の準備

`.exspec.toml` の `[rules] disable` で段階的導入をサポート。
README に推奨設定例を追加:

```toml
# Recommended: Start with Tier 1 only
[rules]
disable = ["T101", "T102", "T103", "T105", "T106", "T107", "T108", "T109"]
```

### 6-6: CI 統合例

README に GitHub Actions の 1 行コピー例を追加。

```yaml
- run: cargo install exspec && exspec --format sarif . > results.sarif
```

---

## Phase 7: OSS Release

### 7-1: LICENSE (MIT)

### 7-2: README 更新

- 冒頭メッセージ: "A static lint for executable-specification-oriented tests"
- 4 性質テーブル (test_architecture.md から)
- 差別化テーブル (vs tsDetect, PyNose, xNose)
- CI 統合例
- Limitations セクション
- 段階的導入ガイド

### 7-3: crates.io 公開

`cargo install exspec` を実現。

### 7-4: GitHub 公開

リポジトリ公開 + SARIF で GitHub Code Scanning 即対応。

### 7-5: Note 記事

「AI 時代のテスト品質を静的解析で守る」+ dogfooding 結果の記事。

---

## Phase 8: Post-Release (フィードバック駆動)

| 優先度 | タスク | トリガー |
|--------|--------|---------|
| P1 | FP 修正 + ルール閾値調整 | ユーザーフィードバック |
| P2 | T201 spec-quality (advisory mode) | 「意味的品質もほしい」の声 |
| P3 | T203 AST similarity 重複検出 | 「重複テスト検出ほしい」の声 |
| P4 | Spec-Export / spec kit 系 | 需要が確認できたら |

---

## test_architecture.md 修正事項 (4者合意)

### 1. SSOT の定義緩和

Before: 1つの仕様に対してテストは1箇所
After: 同一の仕様・同一観点に対してはテストは1箇所に集約する。ただし異なるテストレベル (unit / integration / property) や異なる観点 (機能 / 性能 / セキュリティ) での重なりは許容される。

### 2. Compositional の本質明確化

Before: 小さなspecの組み合わせで全体が構成される
After: 小さなspecの組み合わせで全体が構成される。1テストの責務 (検証対象の概念数) が1つに収まることを優先する。アサート数はあくまで proxy 指標に過ぎない。

### 3. 確率的コードの統計的妥当性追加

以下を新セクションとして追加:
- 固定 seed は再現性のためであって妥当性保証ではない
- 複数 seed での安定性確認
- サンプルサイズ設計 (最低 N=1000 or power analysis)
- flaky 対策を retry でごまかさない
- 信頼区間 (95% CI) での判定例

### 4. What not How の違反例追加 (GPT 提案)

「内部状態の直接 assert (private field access)」を違反例に明記 (T101 連動)

### 5. Contract の限界明記 (GPT 提案)

Contract は必要だが十分ではない。振る舞いは property / example / relation で補う。
