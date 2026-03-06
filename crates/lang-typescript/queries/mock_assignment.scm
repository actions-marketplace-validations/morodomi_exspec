;; Match const mockDb = jest.fn() / vi.fn()
(variable_declarator
  name: (identifier) @var_name
  value: (call_expression
    function: (member_expression
      object: (identifier) @obj
      property: (property_identifier) @method)
    (#match? @obj "^(jest|vi)$")
    (#match? @method "^fn$"))) @mock_assignment
