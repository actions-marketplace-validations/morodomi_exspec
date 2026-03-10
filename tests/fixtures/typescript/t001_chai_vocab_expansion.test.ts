import { expect } from 'chai';
import * as sinon from 'sinon';

// Chai/Sinon vocabulary expansion tests.
// All TC-01 through TC-16 should be detected as assertions.
// TC-17 and TC-18 should NOT be detected.

describe('chai/sinon vocab expansion', () => {
  // TC-01: eq terminal (depth 2)
  it('should detect eq (depth 2)', () => {
    expect(x).to.eq(y);
  });

  // TC-02: eq + not (depth 3)
  it('should detect not.eq (depth 3)', () => {
    expect(x).to.not.eq(y);
  });

  // TC-03: rejectedWith (depth 3)
  it('should detect rejectedWith (depth 3)', () => {
    expect(promise).to.be.rejectedWith(Error);
  });

  // TC-04: rejectedWith + not (depth 4)
  it('should detect not.rejectedWith (depth 4)', () => {
    expect(promise).to.not.be.rejectedWith(TypeError);
  });

  // TC-05: instanceOf (depth 4) — Chai uses camelCase instanceOf, not keyword instanceof
  it('should detect instanceOf (depth 4)', () => {
    expect(x).to.be.an.instanceOf(Foo);
  });

  // TC-06: instanceOf (depth 3)
  it('should detect instanceOf (depth 3)', () => {
    expect(x).to.be.instanceOf(Foo);
  });

  // TC-07: eventually intermediate (depth 3)
  it('should detect eventually.equal (depth 3)', () => {
    expect(promise).to.eventually.equal(42);
  });

  // TC-08: eventually + rejectedWith (depth 4)
  it('should detect eventually.rejectedWith (depth 4)', () => {
    expect(promise).to.eventually.be.rejectedWith(Error);
  });

  // TC-09: rejected property terminal (depth 3)
  it('should detect rejected property (depth 3)', () => {
    expect(promise).to.be.rejected;
  });

  // TC-10: fulfilled property terminal (depth 3)
  it('should detect fulfilled property (depth 3)', () => {
    expect(promise).to.be.fulfilled;
  });

  // TC-11: eventually + rejected property (depth 4)
  it('should detect eventually.rejected property (depth 4)', () => {
    expect(promise).to.eventually.be.rejected;
  });

  // TC-12: eventually + fulfilled property (depth 4)
  it('should detect eventually.fulfilled property (depth 4)', () => {
    expect(promise).to.eventually.be.fulfilled;
  });

  // TC-13: sinon.assert.callOrder (depth-2)
  it('should detect sinon.assert.callOrder', () => {
    sinon.assert.callOrder(spy1, spy2);
  });

  // TC-14: sinon.assert.calledOnce (depth-2)
  it('should detect sinon.assert.calledOnce', () => {
    sinon.assert.calledOnce(spy);
  });

  // TC-15: Sinon.assert.calledWith — capital S
  it('should detect Sinon.assert.calledWith', () => {
    Sinon.assert.calledWith(spy, 'a');
  });

  // TC-16: regression — existing depth-1 assert.equal
  it('should detect assert.equal (regression)', () => {
    assert.equal(a, b);
  });

  // TC-17: negative — sinon.stub() is NOT an assertion
  it('should not count sinon.stub', () => {
    sinon.stub();
  });

  // TC-18: negative — no assertion
  it('should have no assertions', () => {
    const x = 1 + 1;
  });
});
