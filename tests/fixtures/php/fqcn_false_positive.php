<?php
// Issue #8: FQCN pattern should only match PHPUnit attributes, not custom ones

use PHPUnit\Framework\TestCase;

class FqcnFalsePositiveTest extends TestCase
{
    // This should NOT be detected as a test - custom attribute ending in "Test"
    #[\MyApp\Attributes\Test]
    public function custom_attribute_method(): void
    {
        $this->assertTrue(true);
    }

    // This SHOULD be detected as a test - real PHPUnit FQCN attribute
    #[\PHPUnit\Framework\Attributes\Test]
    public function real_phpunit_attribute(): void
    {
        $this->assertEquals(1, 1);
    }
}
