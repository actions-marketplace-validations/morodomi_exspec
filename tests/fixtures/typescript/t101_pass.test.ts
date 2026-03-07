describe('UserService', () => {
  test('creates user with correct name', () => {
    const service = new UserService();
    const user = service.createUser('alice');
    expect(user.name).toBe('alice');
    expect(user.active).toBe(true);
  });
});
