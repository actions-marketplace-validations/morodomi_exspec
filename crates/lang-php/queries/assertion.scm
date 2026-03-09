; PHPUnit: $this->assert*()
(member_call_expression
  object: (variable_name (name) @_obj)
  name: (name) @_method
  (#eq? @_obj "this")
  (#match? @_method "^assert")) @assertion

; PHPUnit: $this->expectException() — exception verification counts as assertion
; (also matched in error_test.scm for T103)
(member_call_expression
  object: (variable_name (name) @_expect_obj)
  name: (name) @_expect_method
  (#eq? @_expect_obj "this")
  (#eq? @_expect_method "expectException")) @assertion

; PHPUnit: $this->expectExceptionMessage()
; (also matched in error_test.scm for T103)
(member_call_expression
  object: (variable_name (name) @_expect_obj2)
  name: (name) @_expect_method2
  (#eq? @_expect_obj2 "this")
  (#eq? @_expect_method2 "expectExceptionMessage")) @assertion

; PHPUnit: $this->expectExceptionCode()
; (also matched in error_test.scm for T103)
(member_call_expression
  object: (variable_name (name) @_expect_obj3)
  name: (name) @_expect_method3
  (#eq? @_expect_obj3 "this")
  (#eq? @_expect_method3 "expectExceptionCode")) @assertion

; Mockery: ->shouldReceive(...) — sets mock expectation (verified at teardown)
(member_call_expression
  name: (name) @_m1 (#eq? @_m1 "shouldReceive")) @assertion

; Mockery: ->shouldHaveReceived(...) — post-execution mock verification
(member_call_expression
  name: (name) @_m2 (#eq? @_m2 "shouldHaveReceived")) @assertion

; Mockery: ->shouldNotHaveReceived(...) — negative mock verification
(member_call_expression
  name: (name) @_m3 (#eq? @_m3 "shouldNotHaveReceived")) @assertion

; PHPUnit mock: ->expects(...) — mock expectation
(member_call_expression
  name: (name) @_e (#eq? @_e "expects")) @assertion

; Pest: expect(...)->toBe(...) and similar
(member_call_expression
  object: (function_call_expression
    function: (name) @_fn
    (#eq? @_fn "expect"))
  name: (name) @_method
  (#match? @_method "^to")) @assertion
