<?php

class UserTest extends \PHPUnit\Framework\TestCase
{
    public function test_create_user(): void
    {
        $user = new User("alice");
        $this->assertEquals("alice", $user->getName());
    }

    public function test_delete_user(): void
    {
        $result = User::delete(1);
        $this->assertTrue($result);
    }
}
