<?php

class ContainerTest extends TestCase
{
    public function test_add_to_assertion_count(): void
    {
        $container = new ContainerBuilder();
        (new CheckTypeDeclarationsPass(true))->process($container);
        $this->addToAssertionCount(1);
    }
}
