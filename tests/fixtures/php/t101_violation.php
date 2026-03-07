<?php

class UserServiceTest extends \PHPUnit\Framework\TestCase
{
    public function test_create_user_calls_repository(): void
    {
        $repo = $this->createMock(UserRepository::class);
        $repo->expects($this->once())
            ->method('save');

        $service = new UserService($repo);
        $service->createUser("alice");
    }

    public function test_notify_uses_mockery(): void
    {
        $notifier = \Mockery::mock(Notifier::class);
        $notifier->shouldReceive('send')
            ->once()
            ->with('hello');

        $service = new UserService($notifier);
        $service->notify("hello");
    }
}
