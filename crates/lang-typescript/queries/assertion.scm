;; Match expect(...).toBe(...) and similar
(call_expression
  function: (member_expression
    object: (call_expression
      function: (identifier) @fn
      (#match? @fn "^expect$")))) @assertion

;; Match assert.* (Node assert module)
(call_expression
  function: (member_expression
    object: (identifier) @obj
    (#match? @obj "^assert$"))) @assertion
