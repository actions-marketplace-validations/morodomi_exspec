import { APP_NAME } from '@app/constants';
import { describe, it, expect } from 'vitest';

describe('Constants', () => {
  it('should have app name', () => {
    expect(APP_NAME).toBe('test-app');
  });
});
