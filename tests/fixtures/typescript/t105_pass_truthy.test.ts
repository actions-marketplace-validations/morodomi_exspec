describe('math', () => {
  it('adds', () => {
    expect(add(1, 2)).toBe(3);
  });

  it('subtracts', () => {
    expect(subtract(5, 3)).toEqual(2);
  });

  it('is truthy', () => {
    expect(compute(10)).toBeTruthy();
  });

  it('divides', () => {
    expect(divide(10, 2)).toBe(5);
  });

  it('powers', () => {
    expect(power(2, 3)).toBe(8);
  });
});
