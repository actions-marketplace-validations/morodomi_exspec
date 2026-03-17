# release-exspec Reference

## Version Management

全クレートは `[workspace.package] version` で一元管理。各クレートは `version.workspace = true`。
更新箇所は root `Cargo.toml` の1箇所のみ。

workspace root `Cargo.toml`:
```toml
[workspace.package]
version = "X.Y.Z"
```

## Crate Dependency Order (publish順)

```
core (依存なし)
  <- lang-python (core)
  <- lang-typescript (core)
  <- lang-php (core)
  <- lang-rust (core)
  <- cli (core + lang-python + lang-typescript + lang-php + lang-rust)
```

## Publish コマンド全容

```bash
cargo publish --manifest-path crates/core/Cargo.toml
sleep 30
cargo publish --manifest-path crates/lang-python/Cargo.toml
sleep 30
cargo publish --manifest-path crates/lang-typescript/Cargo.toml
sleep 30
cargo publish --manifest-path crates/lang-php/Cargo.toml
sleep 30
cargo publish --manifest-path crates/lang-rust/Cargo.toml
sleep 30
cargo publish --manifest-path crates/cli/Cargo.toml
```

## Dry-run コマンド全容

```bash
cargo publish --dry-run --manifest-path crates/core/Cargo.toml
cargo publish --dry-run --manifest-path crates/lang-python/Cargo.toml
cargo publish --dry-run --manifest-path crates/lang-typescript/Cargo.toml
cargo publish --dry-run --manifest-path crates/lang-php/Cargo.toml
cargo publish --dry-run --manifest-path crates/lang-rust/Cargo.toml
cargo publish --dry-run --manifest-path crates/cli/Cargo.toml
```

## CHANGELOG Format

```markdown
## v{VERSION} ({YYYY-MM-DD})

### Features

- **Feature name**: Description (#XX)

### Bug Fixes

- **Fix name**: Description (#XX)

### Internal

- Description
```

- セクション内にエントリがない場合、そのセクションは省略
- commit message の `(#XX)` はそのまま保持 (GitHub issue リンクになる)
- conventional commit type での分類:
  - `feat:` -> Features
  - `fix:` -> Bug Fixes
  - `docs:` -> Documentation (Features に統合してもよい)
  - `refactor:`/`test:`/`chore:` -> Internal

## docs/STATUS.md テスト数更新

`cargo test` 出力の最終行 `test result: ok. XXX passed` から数値を抽出し、
`| Tests | XXX passing | -- |` の行を更新する。

## Git Flow 例外

リリースコミット (`chore: prepare vX.Y.Z release`) は main 直接 push を許可。
根拠: リリースはバージョン番号変更のみで、コード変更を含まないため。

## Publish 途中失敗時のリカバリ

crates.io は unpublish 不可 (yank のみ)。

| 状況 | 対応 |
|------|------|
| core 成功、lang-* で失敗 | エラー修正後、失敗したクレート以降を手動 publish |
| 全て成功、tag push 失敗 | `git tag -a vX.Y.Z` → `git push origin vX.Y.Z` を手動実行 |
| index 反映待ちで dep not found | 60秒待って再試行 (最大3回) |

yank は「セキュリティ問題が発見された場合」のみ使用。publish 順序ミスでは使わない。

## 漏れやすいポイント (このスキルが防ぐもの)

| 漏れ | 防止方法 |
|------|---------|
| CHANGELOG に日付が入っていない | Step 2 で自動挿入 |
| STATUS.md のテスト数が古い | Step 3 で cargo test 結果から自動更新 |
| dry-run 忘れ / 一部クレートだけ | Step 5 で全6クレート dry-run |
| publish 順序間違い | reference.md の依存順を固定参照 |
| tag を publish 前に push して orphan tag 発生 | Step 8 で publish 成功後に tag |
| GitHub Release 作成忘れ | Step 8 で tag と同時実行 |

## GitHub Release Body

CHANGELOG の当該バージョンエントリ (`## v{VERSION}` から次の `## v` まで) を抽出して使用。
追加情報は不要。
