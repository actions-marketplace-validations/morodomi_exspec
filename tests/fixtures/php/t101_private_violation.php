<?php

class UserTest extends \PHPUnit\Framework\TestCase
{
    public function test_private_access_in_assertion(): void
    {
        $user = new User("alice");
        $this->assertEquals("alice", $user->_name);
        $this->assertTrue($user->_isActive);
    }
}
