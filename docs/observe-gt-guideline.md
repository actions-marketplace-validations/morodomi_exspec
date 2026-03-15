# Ground Truth Annotation Guideline

## Purpose

exspec observeの精度評価に使うground truth（正解データ）の判定基準を定義する。
全てのラベリングはこのガイドラインに従う。判断に迷う場合はuncertainとする。

## Definition

テストファイル T に対する正解は、**T が直接的に仕様を検証している production file の集合 G(T)**。

「直接的に仕様を検証している」とは:
- テスト名・describe名がそのファイルのexportを対象としている
- そのファイルを差し替えるとテストの意図が崩れる
- テスト内でそのファイルのexportに対してアサーションしている

## Target Classification

### primary_target

テストが主として検証しているproduction file。

判定基準:
1. describe/it名がそのファイルのexport名と一致 or 強く関連
2. テスト内でそのシンボルに対して直接assertしている
3. そのファイルを削除するとテストの目的が消失する

原則: 1テストにつきprimary_targetは1-2件に絞る。

### secondary_target

テストが間接的に依存しているが、テストの主目的ではないproduction file。

例:
- 定数ファイル（constants.ts）をimportして期待値に使っている
- 型定義をimportしてtype annotationに使っている
- テスト対象のコンストラクタに渡すための依存クラス

### non_target（除外）

以下はtargetに含めない:

| カテゴリ | 例 | 理由 |
|---------|---|------|
| テストユーティリティ | sinon, chai, expect | テスト基盤 |
| mock/stub定義 | test内の.mock.ts | テスト補助 |
| barrel file (index.ts) | decorators/index.ts | 流通経路であり仕様の主体ではない |
| 型のみのimport | `import type { Foo }` | 実行時依存なし |
| importされたが未使用 | import後にテスト内で参照なし | 死にimport |
| helper/utils（テスト補助） | テスト内でのみ使うユーティリティ | テスト基盤 |

### barrel fileの扱い

**原則: barrelはnon_target。実体ファイルをtargetにする。**

例:
- `from '../../decorators'` → `decorators/index.ts` は non_target
- 実際にテストしているのが `inject.decorator.ts` → これが primary_target
- ただし、public APIのentrypointとしてbarrelをテストしている場合は `public_api_target` として別途記録

### public_api_target

テストがpublic API surface（re-export entrypoint）を検証している場合に記録。
primary/secondaryとは独立した軸。

## Evidence Types

各判定に対して証拠を記録する:

| Evidence | 説明 | 強度 |
|----------|------|------|
| symbol_assertion | テスト内でそのシンボルに対してassert | Strong |
| test_name_match | describe/it名がシンボル名・ファイル名と一致 | Strong |
| filename_match | spec名と本番ファイル名が対応 (foo.spec.ts → foo.ts) | Strong |
| direct_import | テストが直接importしている | Medium |
| constructor_usage | new Foo() で直接インスタンス化 | Medium |
| provider_registration | Test.createTestingModule({ providers: [Foo] }) | Medium |
| indirect_import | barrel経由でimport | Weak |
| type_only_import | `import type` のみ | Very Weak |

## Confidence Levels

| Level | 条件 |
|-------|------|
| high | Strong evidence 2つ以上 |
| medium | Strong 1つ + Medium 1つ以上 |
| low | Medium のみ、またはWeak多数 |
| uncertain | 判断困難。再レビュー対象 |

## Edge Cases

### 1テストが複数のproduction fileをテストしている場合
許容する。NestJSのinjector/scanner等のテストでは5件のprimaryが正当なケースがある。
ただし、各primaryに対して「このファイルを削除するとテストの目的が消失するか？」を確認する。

### production fileがテストされていない場合
unmapped_production_filesとして記録。これ自体がobserveのrecall評価に使える。

### cross-package import
packages/core/test/のテストがpackages/common/のコードをimportしている場合、
decorators（@Injectable, @Controller等）は**primaryとして許容**する。
テスト対象のクラスを定義するために必須のデコレータだからである。
定数やenumのimportはsecondary。

### integration test（primary無し）
テストが複数コンポーネントの統合動作を検証し、単一のSUTを特定できない場合、
primary_targets: [] で uncertain とするのが正当。無理にprimaryを1つに絞らない。
例: nested-transient-isolation.spec.ts（Injector + InstanceWrapper + Module + Scopeの統合テスト）

### inline定義のテスト対象
テストファイル自体がproduction classを定義している場合（test utility）、
primary_targets: [] とする。外部production fileをテストしていないため。
例: noop-adapter.spec.ts（NoopHttpAdapterがspec内にinline定義）

### ファイル名のtypo/リネーム
production fileに typo がある場合（select-relp-fn.ts の "relp"）や、
テストとproductionでファイル名が歴史的に不一致の場合（application-ref-host vs http-adapter-host）、
describe名・symbol名を優先してprimaryを判定する。filename_matchは補助的証拠。

### 依存オブジェクトの誤primary
テストのbeforeEachでインスタンス化されるが、テスト自体がその振る舞いを検証していない
オブジェクトはsecondary。NestContainer等の「テスト環境のセットアップ用依存」が典型。
基準: そのオブジェクトのメソッドに対してassertしているか？

### filename_matchの偽陽性
同名の別ファイルが存在する場合（errors/messages.ts vs helpers/messages.ts）、
filename_matchだけでprimaryにしない。必ずimportまたはsymbol usageで裏付ける。

## Data Schema (JSONL)

```json
{
  "test_file": "packages/common/test/decorators/inject.decorator.spec.ts",
  "primary_targets": ["packages/common/decorators/core/inject.decorator.ts"],
  "secondary_targets": ["packages/common/constants.ts"],
  "public_api_targets": [],
  "non_targets": ["packages/common/decorators/index.ts"],
  "confidence": "high",
  "evidence": {
    "symbol_assertion": ["Inject"],
    "test_name_match": ["Inject decorator"],
    "filename_match": "inject.decorator",
    "direct_import": ["../../decorators/core/inject.decorator"],
    "indirect_import": []
  },
  "notes": ""
}
```
