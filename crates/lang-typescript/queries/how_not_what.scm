(call_expression
  function: (member_expression
    property: (property_identifier) @how_pattern)
  (#match? @how_pattern "^(toHaveBeenCalled|toHaveBeenCalledWith|toHaveBeenCalledTimes|toHaveBeenLastCalledWith|toHaveBeenNthCalledWith|toHaveReturned|toHaveReturnedWith|toHaveReturnedTimes|toHaveLastReturnedWith|toHaveNthReturnedWith)$"))
