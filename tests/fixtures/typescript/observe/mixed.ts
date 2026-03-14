export function getUser(id: number): User {
  return db.findById(id);
}

function formatName(name: string): string {
  return name.trim().toLowerCase();
}

export const createUser = (dto: CreateUserDto): User => {
  return db.create(dto);
};

const validateInput = (input: any): boolean => {
  return input != null;
};

export class UserService {
  findAll() {
    return [];
  }

  deleteById(id: number) {
    return db.delete(id);
  }
}

class PrivateHelper {
  transform(data: any) {
    return data;
  }
}
