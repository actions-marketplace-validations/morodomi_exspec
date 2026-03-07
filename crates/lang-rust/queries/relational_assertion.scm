; Rust relational assertion patterns
; Presence of any of these suppresses T105
; Note: assert!(x > y) uses token_tree which is not AST-parseable,
; so we detect via method calls only.

; .is_some(), .is_none(), .is_err(), .is_ok()
; .contains(), .starts_with(), .ends_with()
(call_expression
  function: (field_expression
    field: (field_identifier) @_method)
  (#match? @_method "^(is_some|is_none|is_err|is_ok|contains|starts_with|ends_with)$")) @relational
