// 2 test functions, 3 total assertions → density = 1.5 (>= 1.0)
test('first test', () => {
  const x = 1;
  expect(x).toBe(1);
  expect(x).toBeGreaterThan(0);
});

test('second test', () => {
  const x = 2;
  expect(x).toBe(2);
});
