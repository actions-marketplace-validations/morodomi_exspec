# Known Constraints

exspec uses tree-sitter for static AST analysis. This is fast and language-agnostic, but has inherent limitations.

## Rust macro-generated tests

tree-sitter parses macro bodies as opaque `token_tree` nodes. This affects Rust in two ways:

1. **Test functions inside macros are not detected.** If your test harness generates tests via macros (e.g. `rgtest!` in ripgrep), exspec will not see them at all.
2. **Custom assertion macros**: Macros whose name starts with `assert` (e.g. `assert_pending!`, `assert_ready!`, `assert_data_eq!`) are **auto-detected** as assertions (since v0.4.1). Other custom macros (e.g. `check!`, `verify!`) still need `custom_patterns`.

**Workaround** (for non-`assert_*` macros): Use `[assertions] custom_patterns`:

```toml
[assertions]
custom_patterns = ["check!", "verify!"]
```

This does not help with macro-generated test functions -- those are fundamentally invisible to tree-sitter.

## TypeScript T107 (assertion-roulette)

T107 detects tests with multiple assertions but no descriptive messages. For TypeScript, the assertion message count is always set equal to the assertion count, which means T107 never fires.

This is intentional. TypeScript assertion libraries (Jest, Vitest, Chai) have inconsistent message parameter positions, and false-positive T107 was noisier than useful (36-48% false positive rate in dogfooding).

## Helper delegation

Test functions that delegate assertions to project-local helpers are not recognized as having assertions:

```python
def test_validation(self):
    self.assertValid(data)  # exspec sees no standard assertion
```

```php
public function test_structure(): void {
    $this->assertJsonStructure($response, $expected);  // not recognized
}
```

**Workaround**: Add helper patterns to config:

```toml
[assertions]
custom_patterns = ["assertValid", "assertJsonStructure", "self.assertValid"]
```

**Dogfooding data**: Helper delegation was the primary remaining false positive source across all languages after query-level fixes. In Laravel, 222 remaining BLOCK violations were all helper delegation patterns.

## Benchmark / compile-fail / model-check tests

Some test functions are intentionally assertion-free:

- **Benchmarks**: `benchmark()` functions in pytest-benchmark
- **Compile-fail tests**: Tests that verify compilation failure
- **Model-check tests**: Property-based model checking

These will trigger T001 (assertion-free). Use inline suppression:

```python
# exspec-ignore: T001
def test_benchmark_performance(benchmark):
    benchmark(my_function)
```

## Python observe: src/ layout fallback

Python observe's Layer 2 (import tracing) supports the standard `src/` layout (`src/package/module.py`). When an absolute import like `from package.module import X` cannot be resolved under the scan root directly, exspec falls back to `<scan_root>/src/` as a second candidate.

**Limitations:**

- Only `src/` at depth 1 is supported. Non-standard layouts like `source/`, `lib/`, or nested `src/src/` are not detected.
- The fallback is tried after the direct resolution, so if both `package/module.py` and `src/package/module.py` exist, the direct resolution wins.

## Python observe: shadow variable in attribute-access query

Python observe's `bare_import_attribute.scm` query matches `obj.attr` expressions where `obj` is a bare import name. This is used to trace attribute-access-based imports like `requests.get()` back to the `requests` module.

**Limitation**: The query cannot distinguish module-level names from local variables that shadow the module name:

```python
import requests

def test_it():
    requests = config.requests  # shadows the module
    requests.get("/api")        # incorrectly captured as module attribute access
```

**Mitigation**: The false positive rate is low in practice because:
1. Only identifiers that appear in a `import X` or `from X import ...` statement are tracked
2. Variable shadowing of module names is rare in test code
3. Even when shadowed, the mapping usually points to the correct production module

## Callback / wrapper patterns

Tests that pass assertions through callbacks (e.g. `done()` in Mocha-style async tests) or return assertion wrappers may not be recognized:

```typescript
it('async test', (done) => {
    fetchData().then(data => {
        expect(data).toBe(true);
        done();
    });
});
```

The `expect()` inside the callback **is** counted. But if the test only calls `done()` without any assertion, T001 will fire -- which is typically a true positive.
