<?php

class ArtisanCommandTest extends TestCase
{
    public function test_artisan_expects_output(): void
    {
        $this->artisan('inspire')
            ->expectsOutput('Be yourself; everyone else is already taken.')
            ->assertExitCode(0);
    }

    public function test_artisan_expects_question(): void
    {
        $this->artisan('question:ask')
            ->expectsQuestion('What is your name?', 'Taylor');
    }

    public function test_expect_not_to_perform_assertions(): void
    {
        $this->expectNotToPerformAssertions();
        $service = new NoOpService();
        $service->doNothing();
    }

    public function test_expect_output_string(): void
    {
        $this->expectOutputString('Hello World');
        echo 'Hello World';
    }

    public function test_artisan_expects_no_output(): void
    {
        $this->artisan('silent:command')
            ->expectsNoOutput();
    }
}
