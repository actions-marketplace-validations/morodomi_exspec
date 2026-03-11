<?php

class SkipWithLogicTest extends TestCase
{
    public function testSkipNoAssertion(): void
    {
        $result = doSomething();
        if ($result === null) {
            $this->markTestSkipped('Skip reason');
        }
        // No assertion after skip — T001 should NOT fire (has_skip_call=true)
    }
}
