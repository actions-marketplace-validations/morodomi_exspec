; Detect error/exception testing patterns in Rust

; #[should_panic] attribute
(attribute_item
  (attribute
    (identifier) @_attr
    (#eq? @_attr "should_panic"))) @error_test

; .unwrap_err() call
(call_expression
  function: (field_expression
    field: (field_identifier) @_method)
  (#eq? @_method "unwrap_err")) @error_test

; .is_err() call
(call_expression
  function: (field_expression
    field: (field_identifier) @_method2)
  (#eq? @_method2 "is_err")) @error_test
