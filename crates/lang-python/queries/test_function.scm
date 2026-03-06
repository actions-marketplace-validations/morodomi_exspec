; Test function without decorator
(function_definition
  name: (identifier) @name
  (#match? @name "^test_")) @function

; Test function with decorator (@patch etc.)
(decorated_definition
  definition: (function_definition
    name: (identifier) @name
    (#match? @name "^test_"))) @decorated
