<?php

class MathTest extends \PHPUnit\Framework\TestCase
{
    public function test_add(): void
    {
        $this->assertEquals(4, Math::add(2, 2));
    }

    public function test_add_is_greater(): void
    {
        $this->assertGreaterThan(0, Math::add(1, 1));
    }

    public function test_list_contains(): void
    {
        $this->assertContains("a", ["a", "b", "c"]);
    }

    public function test_is_valid(): void
    {
        $this->assertTrue(Math::isValid(1));
    }

    public function test_null_check(): void
    {
        $this->assertNull(Math::tryParse("invalid"));
    }
}
