import { Missing } from '@app/nonexistent';
import { describe, it, expect } from 'vitest';

describe('Nonexistent', () => {
  it('should handle missing', () => {
    expect(Missing).toBeDefined();
  });
});
