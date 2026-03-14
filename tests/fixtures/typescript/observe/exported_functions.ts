export function findAll(): User[] {
  return db.query('SELECT * FROM users');
}

export function findById(id: number): User {
  return db.query('SELECT * FROM users WHERE id = ?', [id]);
}

function internalHelper(data: any): string {
  return JSON.stringify(data);
}
