import { expect } from 'chai';

// Chai BDD method-call chain assertions (with trailing parentheses).
// These are valid test oracles and should NOT trigger T001.

describe('chai method-call chain assertions', () => {
  // TC-01: depth 2 — expect(x).to.equal(y)
  it('should detect to.equal (depth 2)', () => {
    expect(value).to.equal(42);
  });

  // TC-02: depth 3 — expect(x).to.be.a('string')
  it('should detect to.be.a (depth 3)', () => {
    expect(value).to.be.a('string');
  });

  // TC-03: depth 3 — expect(spy).to.have.callCount(3)
  it('should detect to.have.callCount (depth 3)', () => {
    expect(spy).to.have.callCount(3);
  });

  // TC-04: depth 4 — expect(spy).to.have.been.calledWith(arg)
  it('should detect to.have.been.calledWith (depth 4)', () => {
    expect(spy).to.have.been.calledWith('arg');
  });

  // TC-05: depth 5 — expect(x).to.not.have.been.calledWith(arg)
  it('should detect to.not.have.been.calledWith (depth 5)', () => {
    expect(spy).to.not.have.been.calledWith('arg');
  });

  // TC-06: mixed property + method in same test
  it('should count both property and method assertions', () => {
    expect(value).to.be.true;
    expect(value).to.equal(42);
  });

  // TC-07: multiple method assertions in same test
  it('should count multiple method assertions', () => {
    expect(a).to.equal(1);
    expect(b).to.include('x');
  });

  // TC-08: no assertion (regression guard)
  it('should have no assertions', () => {
    const x = 1 + 1;
  });

  // TC-09: expect(x).to.customHelper() — NOT in terminal vocabulary
  it('should not detect custom helper as assertion', () => {
    expect(x).to.customHelper();
  });

  // TC-10: depth 3 — expect(x).not.to.equal(y) — not at position 1
  it('should detect not.to.equal (depth 3)', () => {
    expect(value).not.to.equal(42);
  });
});
