<?php

class UserTest extends \PHPUnit\Framework\TestCase
{
    public function test_create_user_throws_on_empty_name(): void
    {
        $this->expectException(\InvalidArgumentException::class);
        new User("");
    }

    public function test_create_user(): void
    {
        $user = new User("alice");
        $this->assertEquals("alice", $user->getName());
    }
}
