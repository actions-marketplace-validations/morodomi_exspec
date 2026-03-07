(call
  function: (attribute
    attribute: (identifier) @how_pattern)
  (#match? @how_pattern "^assert_(called|called_once|called_with|called_once_with|any_call|has_calls|not_called)$"))
