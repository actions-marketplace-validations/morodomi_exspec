<?php

class UserServiceTest extends \PHPUnit\Framework\TestCase
{
    public function test_create_user(): void
    {
        $service = new UserService();
        $user = $service->createUser("alice");
        $this->assertEquals("alice", $user->getName());
    }
}
