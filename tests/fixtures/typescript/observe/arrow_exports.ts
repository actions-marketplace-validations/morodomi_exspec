export const findAll = (): User[] => {
  return db.query('SELECT * FROM users');
};

export const findById = (id: number): User => {
  return db.query('SELECT * FROM users WHERE id = ?', [id]);
};

const internalFn = (): void => {
  console.log('internal');
};
