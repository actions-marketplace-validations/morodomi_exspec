---
feature: NestJS Route/Decorator Extractor
phase: DONE
complexity: medium
test_count: 14
risk_level: low
created: 2026-03-15
updated: 2026-03-15
---

# NestJS Route/Decorator Extractor

Phase 8b Task 2: NestJS コントローラーからルート情報とギャップ検出に必要なデコレータを静的抽出する。

## Test List

- [x] RT1: basic_controller_routes - GET /users, POST /users, DELETE /users/:id
- [x] RT2: route_path_combination - Controller base + method sub path
- [x] RT3: controller_no_path - @Controller() + @Get('health') -> GET /health
- [x] RT4: method_without_route_decorator - plain method not in routes
- [x] RT5: all_http_methods - Get, Post, Put, Patch, Delete, Head, Options
- [x] RT6: use_guards_decorator - UseGuards(AuthGuard)
- [x] RT7: multiple_decorators_on_method - UseGuards only, not Delete
- [x] RT8: class_validator_on_dto - IsEmail, IsNotEmpty on DTO fields
- [x] RT9: use_pipes_decorator - UsePipes(ValidationPipe)
- [x] RT10: empty_source_returns_empty - both routes and decorators empty
- [x] RT11: non_nestjs_class_ignored - plain class returns no routes
- [x] RT12: route_handler_and_class_name - correct handler/class names
- [x] RT13: class_level_use_guards - class-level @UseGuards extracted
- [x] RT14: dynamic_controller_path - non-literal path produces &lt;dynamic&gt;

## Progress Log

### 2026-03-15 - RED
- Route/DecoratorInfo 型定義 + スタブメソッド (空 Vec 返却)
- テスト12個作成、全て失敗確認 (10個失敗、2個はスタブで偶然 pass)
- Fixture 4ファイル作成
- Phase completed

### 2026-03-15 - GREEN
- extract_routes: class_body 子ノードウォーク + accumulator パターン
- extract_decorators: gap-relevant フィルタ + DTO field decorator 対応
- AST デバッグで発見: @Controller デコレータは export_statement の子 (class_declaration ではない)
- AST デバッグで発見: DTO のデコレータは public_field_definition の子 (class_body の直接子ではない)
- 全23テスト pass (既存11 + 新規12)
- Phase completed

### 2026-03-15 - REFACTOR
- デバッグテスト削除
- 未使用 DECORATOR_QUERY/DECORATOR_QUERY_CACHE 削除
- collect_gap_decorators ヘルパー抽出で重複コード削減
- cargo fmt 適用
- Quality Gate: cargo test 23 passed, clippy 0, fmt clean, self-dogfooding BLOCK 0
- Phase completed

### 2026-03-15 - REVIEW (codex exec)
- BLOCK 1: クラスレベル UseGuards/UsePipes が抽出されない -> 修正 (find_decorators_on_node)
- BLOCK 2: 非リテラルパスで誤パス生成 -> 修正 (&lt;dynamic&gt; プレースホルダー)
- WARN: DecoratorInfo.target_name の型区別なし -> MVP許容 (Task 4bで必要なら拡張)
- RT13, RT14 テスト追加
- Quality Gate: cargo test 741 passed (全crate), clippy 0, fmt clean, self-dogfooding BLOCK 0
- Phase completed

### 2026-03-15 - COMMIT
- NestJS Route/Decorator Extractor 実装完了
- Phase completed
