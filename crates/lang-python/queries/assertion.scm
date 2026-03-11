; assert statement
(assert_statement) @assertion

; unittest self.assert* methods
(call
  function: (attribute
    object: (identifier) @obj
    attribute: (identifier) @method)
  (#match? @obj "^self$")
  (#match? @method "^assert")) @assertion

; pytest.raises() — exception verification counts as assertion
; (also matched in error_test.scm for T103)
(call
  function: (attribute
    object: (identifier) @_pytest_obj
    attribute: (identifier) @_pytest_attr)
  (#eq? @_pytest_obj "pytest")
  (#eq? @_pytest_attr "raises")) @assertion

; arbitrary object: obj.assert*() methods (mock, reprec, custom test helpers)
; self.assert* is handled above (lines 4-10), so exclude self here
(call
  function: (attribute
    object: (identifier) @_assert_obj
    attribute: (identifier) @_mock_method)
  (#match? @_mock_method "^assert")
  (#not-match? @_assert_obj "^self$")) @assertion

; chained call: expr.something.assert_*() (e.g., mock.return_value.assert_called_once())
; object is not an identifier, so no self overlap concern
(call
  function: (attribute
    object: (attribute)
    attribute: (identifier) @_chained_assert_method)
  (#match? @_chained_assert_method "^assert_")) @assertion

; pytest.warns() — warning verification counts as assertion
; (also matched in error_test.scm for T103)
(call
  function: (attribute
    object: (identifier) @_pytest_warns_obj
    attribute: (identifier) @_pytest_warns_attr)
  (#eq? @_pytest_warns_obj "pytest")
  (#eq? @_pytest_warns_attr "warns")) @assertion

; pytest.fail() — explicit failure oracle counts as assertion
; unconditionally fails the test with a message (functionally equivalent to `assert False, msg`)
(call
  function: (attribute
    object: (identifier) @_pytest_fail_obj
    attribute: (identifier) @_pytest_fail_attr)
  (#eq? @_pytest_fail_obj "pytest")
  (#eq? @_pytest_fail_attr "fail")) @assertion
