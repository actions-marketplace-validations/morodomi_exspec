; obj._private
(member_expression
  property: (property_identifier) @private_access
  (#match? @private_access "^_[^_]"))

; obj['_private']
(subscript_expression
  index: (string
    (string_fragment) @private_access
    (#match? @private_access "^_[^_]")))
