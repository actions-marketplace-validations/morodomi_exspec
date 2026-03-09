<?php

class ResponseTest extends TestCase
{
    public function test_response_assert_status(): void
    {
        $response = $this->get('/api/users');
        $response->assertStatus(200);
    }

    public function test_chained_response_assertions(): void
    {
        $response = $this->postJson('/api/users', ['name' => 'John']);
        $response->assertStatus(201)
            ->assertJsonCount(1, 'data')
            ->assertJsonPath('data.name', 'John');
    }

    public function test_bare_assert_call(): void
    {
        $result = compute();
        $result->assert();
    }

    public function test_assertion_helper_not_counted(): void
    {
        // assertionHelper should NOT be counted as assertion
        $helper->assertionHelper();
    }
}
