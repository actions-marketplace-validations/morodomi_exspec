<?php
// Issue #7: Methods with both attribute and docblock should be counted once

use PHPUnit\Framework\TestCase;
use PHPUnit\Framework\Attributes\Test;

class DocblockDoubleDetectionTest extends TestCase
{
    // Case 1: #[Test] short attribute + /** @test */ docblock
    /**
     * @test
     */
    #[Test]
    public function short_attribute_with_docblock(): void
    {
        $this->assertTrue(true);
    }

    // Case 2: FQCN attribute + /** @test */ docblock
    /**
     * @test
     */
    #[\PHPUnit\Framework\Attributes\Test]
    public function fqcn_attribute_with_docblock(): void
    {
        $this->assertEquals(1, 1);
    }

    // Case 3: /** @test */ docblock only (normal case, should be detected once)
    /**
     * @test
     */
    public function docblock_only(): void
    {
        $this->assertNotNull('hello');
    }
}
