<?php

class MathTest extends \PHPUnit\Framework\TestCase
{
    public function test_add(): void
    {
        $this->assertEquals(4, Math::add(2, 2));
    }

    public function test_subtract(): void
    {
        $this->assertEquals(0, Math::subtract(2, 2));
    }

    public function test_multiply(): void
    {
        $this->assertEquals(6, Math::multiply(2, 3));
    }

    public function test_divide(): void
    {
        $this->assertEquals(2, Math::divide(4, 2));
    }

    public function test_modulo(): void
    {
        $this->assertEquals(1, Math::modulo(5, 2));
    }
}
