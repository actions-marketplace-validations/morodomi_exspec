; Function calls (free function — helper call in test body)
(call_expression function: (identifier) @call_name)

; Function declarations
(function_declaration
  name: (identifier) @def_name
  body: (statement_block) @def_body)

; Arrow function helpers assigned to const/let/var
; e.g. const assertValid = (x) => { expect(x)... }
(lexical_declaration
  (variable_declarator
    name: (identifier) @def_name
    value: (arrow_function
      body: (statement_block) @def_body)))
