import { Controller, Get } from '@nestjs/common';

@Controller()
export class HealthController {
  @Get('health')
  check() {
    return { status: 'ok' };
  }

  // No route decorator - should not appear in routes
  helperMethod() {
    return 'internal';
  }
}
