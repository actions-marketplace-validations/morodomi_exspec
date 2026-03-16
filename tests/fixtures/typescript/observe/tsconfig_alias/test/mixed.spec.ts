import { FooService } from '@app/foo.service';
import { BarService } from '../src/bar.service';
import { describe, it, expect } from 'vitest';

describe('Mixed imports', () => {
  it('should work with both import styles', () => {
    expect(new FooService().findAll()).toEqual([]);
    expect(new BarService().bar()).toBe('bar');
  });
});
