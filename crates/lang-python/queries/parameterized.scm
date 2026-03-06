; Detect @pytest.mark.parametrize decorator
(decorated_definition
  (decorator
    (call
      function: (attribute
        object: (attribute
          object: (identifier) @_pytest
          attribute: (identifier) @_mark)
        attribute: (identifier) @_param)
      (#eq? @_pytest "pytest")
      (#eq? @_mark "mark")
      (#eq? @_param "parametrize")))
  definition: (function_definition
    name: (identifier) @name)) @parameterized
