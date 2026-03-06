test('with one mock', () => {
  const mockDb = jest.fn();
  const result = query(mockDb);
  expect(result).toEqual([]);
});
