; assert!, assert_eq!, assert_ne!, prop_assert! macros
(macro_invocation
  macro: (identifier) @_name
  (#match? @_name "^(assert|assert_eq|assert_ne|debug_assert|debug_assert_eq|debug_assert_ne|prop_assert|prop_assert_eq|prop_assert_ne)$")) @assertion

; Rust-specific policy refinement:
; treat free-function assert_*() helpers as assertion oracles.
; ^assert_ avoids matching assertion_* and does not overlap with assert! macros.
(call_expression
  function: (identifier) @_fn
  (#match? @_fn "^assert_")) @assertion

; Scoped helper delegation: module::assert_*()
(call_expression
  function: (scoped_identifier
    name: (identifier) @_scoped_fn)
  (#match? @_scoped_fn "^assert_")) @assertion
