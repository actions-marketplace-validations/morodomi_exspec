<?php

class OrderServiceTest extends TestCase
{
    public function test_sends_notification_on_order(): void
    {
        $mock = Mockery::mock(Notifier::class);
        $mock->shouldReceive('send')->once()->with('order_created');

        $service = new OrderService($mock);
        $service->createOrder(['item' => 'widget']);
    }

    public function test_verifies_post_execution(): void
    {
        $mock = Mockery::mock(Logger::class);

        $service = new AuditService($mock);
        $service->performAction('delete');

        $mock->shouldHaveReceived('log')->once()->with('delete');
    }

    public function test_negative_verification(): void
    {
        $mock = Mockery::mock(Mailer::class);

        $service = new UserService($mock);
        $service->createGuest();

        $mock->shouldNotHaveReceived('sendWelcome');
    }

    public function test_multiple_mock_expectations(): void
    {
        $mock = Mockery::mock(Gateway::class);
        $mock->shouldReceive('connect')->once();
        $mock->shouldReceive('send')->once()->with('data');
        $mock->shouldReceive('disconnect')->once();

        $gateway = new GatewayWrapper($mock);
        $gateway->transmit('data');
    }

    public function test_chained_mock_expectation(): void
    {
        $mock = Mockery::mock(Cache::class);
        $mock->shouldReceive('get')
            ->once()
            ->with('key')
            ->andReturn('value');

        $service = new CacheService($mock);
        $result = $service->fetch('key');
    }
}
