; assert*!, debug_assert*!, prop_assert*! macros (prefix match — auto-detects custom assert macros)
; (_|$) ensures "assert_pending!" matches but "assertion!" does not.
(macro_invocation
  macro: (identifier) @_name
  (#match? @_name "^(assert(_|$)|debug_assert(_|$)|prop_assert(_|$))")) @assertion

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
