describe('PrivateAccess', () => {
  test('checks internal count via dot notation', () => {
    const service = new Service();
    service.process();
    expect(service._count).toBe(1);
    expect(service._processed).toBe(true);
  });

  test('checks internal via bracket notation', () => {
    const service = new Service();
    service.process();
    expect(service['_count']).toBe(1);
    expect(service['_processed']).toBe(true);
  });

  test('mixed private and mock verification', () => {
    const mockRepo = jest.fn();
    const service = new UserService(mockRepo);
    service.createUser('alice');
    expect(mockRepo.save).toHaveBeenCalledWith('alice');
    expect(service._lastCreated).toBe('alice');
  });

  test('private outside expect not counted', () => {
    const service = new Service();
    const value = service._internal;
    expect(value).toBe(42);
  });
});
