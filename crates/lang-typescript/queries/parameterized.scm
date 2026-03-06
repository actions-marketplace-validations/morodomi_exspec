; Detect test.each(), it.each(), describe.each() patterns
(call_expression
  function: (call_expression
    function: (member_expression
      object: (identifier) @fn_name
      property: (property_identifier) @method)
    (#match? @fn_name "^(test|it|describe)$")
    (#eq? @method "each"))) @parameterized
