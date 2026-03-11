<?php

class SkipOnlyTest extends TestCase
{
    public function testSkippedFeature(): void
    {
        $this->markTestSkipped('Not supported.');
    }

    public function testIncompleteFeature(): void
    {
        $this->markTestIncomplete('Not implemented yet.');
    }
}
