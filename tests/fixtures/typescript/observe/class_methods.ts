export class UsersController {
  findAll() {
    return this.usersService.findAll();
  }

  create(dto: CreateUserDto) {
    return this.usersService.create(dto);
  }

  validate(id: number) {
    return this.usersService.validate(id);
  }
}

class InternalService {
  process(data: any) {
    return data;
  }
}
