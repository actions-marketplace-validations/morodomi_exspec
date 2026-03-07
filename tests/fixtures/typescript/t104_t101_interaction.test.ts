it('mock and hardcoded both fire', () => {
  expect(jest.fn()).toHaveBeenCalled();
  expect(add(1, 2)).toBe(3);
});
