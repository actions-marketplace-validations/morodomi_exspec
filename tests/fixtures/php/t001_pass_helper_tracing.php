<?php
use PHPUnit\Framework\TestCase;

// Helper WITH assertion (exactly 1 assert — free function at file level)
function check_result($value) {
    \PHPUnit\Framework\Assert::assertTrue($value > 0);
}

// Helper WITHOUT assertion
function no_assert_helper($value) {
    echo $value;
}

// 2-hop chain
function intermediate($value) {
    check_result($value);
}

class HelperTracingTest extends TestCase {
    // TC-01
    public function test_calls_helper_with_assert() {
        check_result(42);
    }
    // TC-02
    public function test_calls_helper_without_assert() {
        no_assert_helper(42);
    }
    // TC-03
    public function test_has_own_assert_plus_helper() {
        $this->assertTrue(true);
        check_result(42);
    }
    // TC-04
    public function test_calls_undefined_function() {
        undefined_function(42);
    }
    // TC-05
    public function test_two_hop_tracing() {
        intermediate(42);
    }
    // TC-06
    public function test_with_assertion_early_return() {
        $this->assertTrue(true);
    }
    // TC-07
    public function test_multiple_calls_same_helper() {
        check_result(1);
        check_result(2);
    }
}
