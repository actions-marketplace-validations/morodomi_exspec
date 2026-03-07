describe('UserService', () => {
  test('calls repository on create', () => {
    const mockRepo = jest.fn();
    const service = new UserService(mockRepo);
    service.createUser('alice');
    expect(mockRepo.save).toHaveBeenCalledWith('alice');
    expect(mockRepo.save).toHaveBeenCalledTimes(1);
    expect(service).toBeDefined();
  });

  test('sends notification on order', () => {
    const mockNotifier = jest.fn();
    const service = new OrderService(mockNotifier);
    service.placeOrder({ item: 'book' });
    expect(mockNotifier.send).toHaveBeenCalled();
    expect(true).toBe(true);
  });
});
