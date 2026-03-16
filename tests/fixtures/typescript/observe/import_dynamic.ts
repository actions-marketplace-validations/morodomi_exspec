// Boundary B5: dynamic import is not captured by static import extraction
describe('UserService', () => {
  it('should load dynamically', async () => {
    const m = await import('./user.service');
    expect(m.UserService).toBeDefined();
  });
});
