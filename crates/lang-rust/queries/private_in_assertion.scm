; Rust private field access: obj._field
(field_expression
  field: (field_identifier) @private_access
  (#match? @private_access "^_[^_]"))
