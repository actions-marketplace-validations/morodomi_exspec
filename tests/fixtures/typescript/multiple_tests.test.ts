function helper() {
  return 42;
}

test('adds numbers', () => {
  expect(1 + 2).toBe(3);
});

it('subtracts numbers', () => {
  expect(5 - 3).toBe(2);
});

describe('math operations', () => {
  it('multiplies numbers', () => {
    expect(2 * 3).toBe(6);
  });
});
