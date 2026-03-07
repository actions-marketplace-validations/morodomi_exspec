it('round-trips data', () => {
  const data = 'hello';
  expect(decode(encode(data))).toBe(data);
});
