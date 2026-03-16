import { BarService } from '@app/services';
import { describe, it, expect } from 'vitest';

describe('BarService', () => {
  it('should return bar', () => {
    expect(new BarService().bar()).toBe('bar');
  });
});
