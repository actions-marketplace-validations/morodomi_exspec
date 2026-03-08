describe("custom helpers", () => {
  it("uses custom assertion", () => {
    const result = compute(42);
    myAssert(result === 42);
  });

  it("uses standard expect", () => {
    const result = compute(1);
    expect(result).toBe(1);
  });

  it("has no assertion", () => {
    const result = compute(0);
    console.log(result);
  });
});
