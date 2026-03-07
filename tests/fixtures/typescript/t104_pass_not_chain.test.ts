it('uses not chain with variable', () => {
  const result = compute(42);
  expect(result).not.toBe(0);
});
