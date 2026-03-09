<?php

class StaticAssertTest extends TestCase
{
    public function test_self_assert_equals(): void
    {
        $result = Calculator::add(1, 2);
        self::assertEquals(3, $result);
    }

    public function test_static_assert_true(): void
    {
        $flag = FeatureFlag::isEnabled('beta');
        static::assertTrue($flag);
    }

    public function test_parent_assert_same(): void
    {
        $value = $this->computeInherited();
        parent::assertSame('expected', $value);
    }
}
