// Parameterized test via test.each → ratio >= 0.1
test.each([
  [1, 1, 2],
  [2, 2, 4],
  [3, 3, 6],
])('add(%i, %i) = %i', (a, b, expected) => {
  expect(a + b).toBe(expected);
});

test('subtract', () => {
  expect(3 - 1).toBe(2);
});
