import { Controller, Get, Post, Put, Patch, Delete, Head, Options } from '@nestjs/common';

@Controller('api/v1/users')
export class UsersController {
  @Get()
  findAll() {
    return this.usersService.findAll();
  }

  @Get('active')
  findActive() {
    return this.usersService.findActive();
  }

  @Get(':id')
  findOne() {
    return this.usersService.findOne();
  }

  @Post()
  create() {
    return this.usersService.create();
  }

  @Put(':id')
  update() {
    return this.usersService.update();
  }

  @Patch(':id')
  partialUpdate() {
    return this.usersService.partialUpdate();
  }

  @Delete(':id')
  remove() {
    return this.usersService.remove();
  }

  @Head()
  head() {
    return;
  }

  @Options()
  options() {
    return;
  }
}
