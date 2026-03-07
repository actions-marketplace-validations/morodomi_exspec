; Rust mockall verification patterns (how-not-what)

; mock.expect_*() - mockall patterns
(call_expression
  function: (field_expression
    field: (field_identifier) @how_pattern)
  (#match? @how_pattern "^expect_"))
