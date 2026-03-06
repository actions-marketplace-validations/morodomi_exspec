// exspec-ignore: T001
describe('user management', () => {
  test('create user', () => {
    const user = createUser('alice');
    // No assertion -- suppression on describe should NOT apply here
  });

  test('delete user', () => {
    deleteUser('alice');
    // No assertion -- suppression on describe should NOT apply here
  });
});
