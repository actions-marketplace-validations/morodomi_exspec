// 3 test functions, only 1 total assertion → density < 1.0
test('first test', () => {
  const x = 1;
  expect(x).toBe(1);
});

test('second test', () => {
  const x = 2;
  const y = x + 1;
});

test('third test', () => {
  const x = 3;
  const y = x + 2;
});
