---
name: release-exec
description: exspec (Cargo workspace) のリリース実行スキル。workspace.package のバージョン更新、CHANGELOG自動生成、docs/STATUS.md更新、crates.io publish、GitHub Releaseを一括実行。「リリース」「release」「publish」「バージョン上げて」で起動。Do NOT use for コード変更を伴う開発作業（dev-crewを使う）。
---

# release-exec

exspec Cargo workspace のバージョンリリースを実行する。

## Progress Checklist

```
- [ ] Pre-flight (test/clippy/fmt/dogfood)
- [ ] Version decided
- [ ] CHANGELOG generated
- [ ] STATUS.md updated
- [ ] Cargo.toml version bumped
- [ ] Verification (test + dry-run all 6 crates)
- [ ] Commit + Push
- [ ] crates.io publish (6 crates)
- [ ] Tag + GitHub Release
```

## Pre-flight (自動実行、1つでも失敗したら停止)

1. `git status` -- working tree clean
2. `cargo test` -- 全テスト通過 (テスト数を記録)
3. `cargo clippy -- -D warnings` -- 静的解析0件
4. `cargo fmt --check` -- フォーマット差分なし
5. `cargo run -- --lang rust .` -- self-dogfooding BLOCK 0件

## Step 1: バージョン決定

AskUserQuestion で確認:
- リリースバージョン (semver: `MAJOR.MINOR.PATCH`)
- 前バージョンは `git describe --tags --abbrev=0` で自動取得

## Step 2: CHANGELOG 自動生成

1. `git log <前バージョンタグ>..HEAD --oneline` でコミット一覧取得
2. conventional commit type で分類 (reference.md 参照)
3. `## v{VERSION} ({YYYY-MM-DD})` エントリをドラフト生成
4. AskUserQuestion でユーザーに確認・編集依頼
5. CHANGELOG.md の先頭 (`# Changelog` の次) に挿入

## Step 3: docs/STATUS.md 更新

Pre-flight で記録した `cargo test` のテスト数で `Tests` 行を更新。

## Step 4: Cargo.toml バージョン更新

root `Cargo.toml` の `[workspace.package] version` を更新 (1箇所のみ)。
各クレートは `version.workspace = true` で自動追従。reference.md 参照。

## Step 5: Verification

全6クレートの dry-run を依存順に実行 (reference.md の publish 順):
```bash
cargo test
cargo publish --dry-run --manifest-path crates/core/Cargo.toml
# ... (全6クレート、reference.md 参照)
```

## Step 6: Commit + Push

```bash
git add Cargo.toml CHANGELOG.md docs/STATUS.md
git commit -m "chore: prepare v{VERSION} release"
```

AskUserQuestion で push 確認後 main に直接 push (リリースコミットは例外):
```bash
git push origin main
```

## Step 7: crates.io Publish

AskUserQuestion で確認 (不可逆操作)。依存順に publish、各30秒待機。
失敗時のリカバリ手順は reference.md 参照。

## Step 8: Tag + GitHub Release

全 publish 成功後に tag 作成・push + GitHub Release。
```bash
git tag -a v{VERSION} -m "Release v{VERSION}"
git push origin v{VERSION}
gh release create v{VERSION} --title "v{VERSION}" --notes-file <changelog-excerpt>
```

完了後にバージョン、crates.io URL、GitHub Release URL を報告。
