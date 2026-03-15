import { Controller, Get } from '@nestjs/common';

const BASE_PATH = 'api/v1';

@Controller(BASE_PATH)
export class DynamicController {
  @Get()
  index() {
    return 'hello';
  }
}
