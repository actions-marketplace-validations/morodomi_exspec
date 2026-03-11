# TypeScript (Jest / Vitest)

Since v0.1.0. Supports `.ts` and `.tsx` files (TSX parser used for `.tsx`).

## Test Detection

- `test()` and `it()` calls

## Assertions

- `expect()` chains (Jest/Vitest matchers)
- Chai property/method chains (`.to.be.true`, `.to.have.been.called`, etc.)
- Chai chain depth supported up to depth 7 (e.g. `.rejected.and.be.an.instanceof()`)
- `expect.assertions()` / `expect.hasAssertions()` counted
- `expect.soft()` modifier chains counted
- Sinon `.verify()` broad match (any `<expr>.verify()`)

## T107 (assertion-roulette)

Always set to `assertion_count` rather than independently counting message arguments. T107 never fires for TypeScript. This is intentional -- Jest/Vitest `expect()` has no message argument, and independent counting had 36-48% FP rate.

## Inline Suppression

`// exspec-ignore` applies to the **next** `test()`/`it()` call only. It does **not** propagate through a `describe()` block. Suppress each test individually.

## Known Gaps

- **Return-wrapped Chai property assertions** (#52): `return expect(x).to.be.true` in arrow functions
- **Helper delegation**: Project-local helpers need `[assertions] custom_patterns`

## Dogfooding Results

| Project | Tests | BLOCK | Progression | Notes |
|---------|-------|-------|-------------|-------|
| vitest | 3,120 | 326 | 432->350->326 | Remaining = project-local helpers |
| nestjs | 2,675 | 17 | 90->34->17 | 0% FP. All 17 are TP |
