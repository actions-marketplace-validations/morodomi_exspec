// Helper WITH assertion (exactly 1 expect)
function checkResult(value: number) {
  expect(value).toBeGreaterThan(0);
}

// Helper WITHOUT assertion
function noAssertHelper(value: number) {
  console.log(value);
}

// Calls checkResult (2-hop chain) but has NO assertion itself
function intermediate(value: number) {
  checkResult(value);
}

// Arrow function helper with assertion
const assertPositive = (value: number) => {
  expect(value).toBeGreaterThan(0);
};

// TC-01: calls helper with assertion → assertion_count >= 1
it('TC-01 calls helper with assert', () => {
  checkResult(42);
});

// TC-02: calls helper without assertion → assertion_count == 0
it('TC-02 calls helper without assert', () => {
  noAssertHelper(42);
});

// TC-03: has own expect + calls helper → assertion_count >= 1 (no extra tracing)
it('TC-03 has own assert plus helper', () => {
  expect(1).toBe(1);
  checkResult(42);
});

// TC-04: calls undefined function → no crash, assertion_count == 0
it('TC-04 calls undefined function', () => {
  undefinedFunction(42);
});

// TC-05: 2-hop: test → intermediate → checkResult — only 1-hop traced.
// intermediate() itself has NO assertion → assertion_count stays 0
it('TC-05 two hop tracing', () => {
  intermediate(42);
});

// TC-06: test with own assertion → early return path, assertion_count unchanged
it('TC-06 with assertion early return', () => {
  expect(true).toBe(true);
});

// TC-07: calls checkResult() twice → dedup, assertion_count == 1 (not 2)
it('TC-07 multiple calls same helper', () => {
  checkResult(1);
  checkResult(2);
});

// TC-08: arrow function helper with assertion → assertion_count >= 1
it('TC-08 arrow function helper', () => {
  assertPositive(42);
});
