# PHP (PHPUnit / Pest)

Since v0.1.0. Mockery assertions supported.

## Test Detection

- Methods starting with `test`
- Methods annotated with `@test` (docblock) or `#[Test]` (attribute, including FQCN)
- Pest `test()` / `it()` calls (including arrow function bodies)

## Assertions

- `$this->assert*()` method calls
- `self::assert*()` static calls
- `Assert::assert*()` named-class static calls (without `Assert$` constraint)
- `$this->expect*()` calls
- `$this->expects()` (constrained to `$this` only for precision)
- Mockery: `shouldReceive`, `shouldHaveReceived`, `shouldNotHaveReceived`, `expects`
- Facade assertions: `ClassName::assert*()` (Event, Sleep, Bus, Queue, Notification, Mail, etc.)

## Known Gaps

- **Helper delegation**: Patterns like `$this->fails()`, `$assert->has()`, AssertableJson fluent chains, Laravel validation/route helpers
- **Workaround**: `[assertions] custom_patterns`

## Dogfooding Results

| Project | Tests | BLOCK | Progression | Notes |
|---------|-------|-------|-------------|-------|
| laravel | 10,790 | 222 | 1305->776->224->222 | Remaining = helper delegation |

### Laravel BLOCK Breakdown (222 remaining)

| Category | Count |
|----------|-------|
| AssertableJson fluent | 49 |
| Validation helpers | 58 |
| Route helpers | 21 |
| Misc helpers | 94 |

All are helper delegation patterns. Addressable via `[assertions] custom_patterns`.
