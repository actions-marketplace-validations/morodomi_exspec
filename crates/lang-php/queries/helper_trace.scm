; Function calls (free function — helper call in test body)
(function_call_expression function: (name) @call_name)

; Function definitions (helper function with body)
(function_definition
  name: (name) @def_name
  body: (compound_statement) @def_body)
