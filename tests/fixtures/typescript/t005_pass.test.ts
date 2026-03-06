import * as fc from 'fast-check';

test('add is commutative', () => {
  fc.assert(
    fc.property(fc.integer(), fc.integer(), (a, b) => {
      expect(a + b).toBe(b + a);
    })
  );
});
