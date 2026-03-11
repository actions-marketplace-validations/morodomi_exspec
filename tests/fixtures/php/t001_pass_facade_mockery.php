<?php

class FacadeMockeryTest extends TestCase
{
    public function test_log_should_receive(): void
    {
        Log::shouldReceive('error')->once()->with('Something failed');

        $service = new ErrorHandler();
        $service->handleError('Something failed');
    }

    public function test_log_should_have_received(): void
    {
        $service = new AuditService();
        $service->performAction('update');

        Log::shouldHaveReceived('info')->once()->with('update');
    }

    public function test_log_should_not_have_received(): void
    {
        $service = new QuietService();
        $service->run();

        Log::shouldNotHaveReceived('debug');
    }

    public function test_chained_facade_mockery(): void
    {
        Cache::shouldReceive('get')
            ->once()
            ->with('key')
            ->andReturn('value');

        $service = new CacheService();
        $result = $service->fetch('key');
    }
}
