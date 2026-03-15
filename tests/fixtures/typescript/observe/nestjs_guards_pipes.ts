import { Controller, Get, Post, UseGuards, UsePipes, ValidationPipe } from '@nestjs/common';
import { AuthGuard } from './auth.guard';

@Controller('admin')
export class AdminController {
  @Get()
  @UseGuards(AuthGuard)
  dashboard() {
    return this.adminService.getDashboard();
  }

  @Post('settings')
  @UseGuards(AuthGuard, RoleGuard)
  @UsePipes(ValidationPipe)
  updateSettings() {
    return this.adminService.updateSettings();
  }
}

// Class-level guard applies to all methods
@UseGuards(JwtAuthGuard)
@Controller('protected')
export class ProtectedController {
  @Get()
  getData() {
    return this.service.getData();
  }

  @Post()
  createData() {
    return this.service.createData();
  }
}
