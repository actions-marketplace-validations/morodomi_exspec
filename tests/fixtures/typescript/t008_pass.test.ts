import { z } from 'zod';

const UserSchema = z.object({
  name: z.string(),
  age: z.number(),
});

test('user validation', () => {
  const user = UserSchema.parse({ name: 'Alice', age: 30 });
  expect(user.name).toBe('Alice');
});
