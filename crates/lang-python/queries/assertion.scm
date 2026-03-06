; assert statement
(assert_statement) @assertion

; unittest self.assert* methods
(call
  function: (attribute
    object: (identifier) @obj
    attribute: (identifier) @method)
  (#match? @obj "^self$")
  (#match? @method "^assert")) @assertion
