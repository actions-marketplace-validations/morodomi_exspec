test('create user', () => {
  const user = createUser('Alice');
  expect(user.name).toBe('Alice');
});
