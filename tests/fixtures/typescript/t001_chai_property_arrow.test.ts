import { expect } from 'chai';

// Chai property-style assertions in arrow function concise body.
// expression_statement does not exist in concise arrow body,
// so arrow_function body: patterns are needed.

describe('chai property in arrow body', () => {
  // TC-01: depth 2 arrow body — expect(x).to.be.ok
  it('should detect property in forEach arrow', () => {
    items.forEach(x => expect(x).to.be.ok);
  });

  // TC-02: depth 4 arrow body — expect(obj).to.not.be.undefined
  it('should detect property in then arrow (depth 4)', () => {
    promise.then(obj => expect(obj).to.not.be.undefined);
  });

  // TC-03: depth 4 arrow body — expect(obj).to.have.been.calledOnce
  it('should detect sinon property in then arrow (depth 4)', () => {
    promise.then(obj => expect(obj).to.have.been.calledOnce);
  });

  // TC-04: depth 5 arrow body — expect(obj).to.not.have.been.calledOnce
  it('should detect sinon property in arrow (depth 5)', () => {
    promise.then(obj => expect(obj).to.not.have.been.calledOnce);
  });

  // TC-05: regression — block body still works
  it('should detect property in block body arrow', () => {
    promise.then(obj => { expect(obj).to.be.ok; });
  });

  // TC-06: regression — method-call in arrow (already works)
  it('should detect method-call in arrow', () => {
    promise.then(obj => expect(obj).to.equal(42));
  });

  // TC-07: #48 vocabulary + arrow body
  it('should detect rejected property in arrow', () => {
    promises.forEach(p => expect(p).to.be.rejected);
  });

  // TC-08: negative — no assertion
  it('should have no assertions in arrow', () => {
    items.map(x => x + 1);
  });
});
