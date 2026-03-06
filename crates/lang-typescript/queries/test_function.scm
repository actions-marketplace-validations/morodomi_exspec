;; Match test('name', fn) and it('name', fn)
(call_expression
  function: (identifier) @fn_name
  arguments: (arguments
    (string
      (string_fragment) @name)
    [(arrow_function) (function_expression)])
  (#match? @fn_name "^(test|it)$")) @function
