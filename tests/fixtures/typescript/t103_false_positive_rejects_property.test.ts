describe('ServiceWithRejects', () => {
  it('has a rejects property', () => {
    const service = createService();
    expect(service.rejects).toEqual([]);
  });
});
