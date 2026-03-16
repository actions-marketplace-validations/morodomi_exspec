import { FooService } from '@app/foo.service';
import { describe, it, expect } from 'vitest';

describe('FooService', () => {
  it('should return empty array', () => {
    const service = new FooService();
    expect(service.findAll()).toEqual([]);
  });
});
