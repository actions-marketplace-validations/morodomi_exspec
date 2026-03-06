// exspec-ignore: T002
test('suppressed mocks', () => {
  const mockDb = jest.fn();
  const mockCache = jest.fn();
  const mockLogger = jest.fn();
  const mockAuth = jest.fn();
  const mockPaymentService = jest.fn();
  const mockNotifier = jest.fn();
  const result = doSomething(mockDb, mockCache, mockLogger, mockAuth, mockPaymentService, mockNotifier);
  expect(result).toBeDefined();
});
