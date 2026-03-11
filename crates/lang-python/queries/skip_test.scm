; #64: Python skip-only test detection
; Covers function-body skip calls only.
; Decorator-based skip (@pytest.mark.skip) is not covered (separate detection scope).
; Nested function limitation applies (same as #41 for assertions).
; pytest.skip()
(call
  function: (attribute
    object: (identifier) @_mod
    attribute: (identifier) @_fn)
  (#eq? @_mod "pytest")
  (#eq? @_fn "skip")) @skip

; unittest: self.skipTest()
(call
  function: (attribute
    object: (identifier) @_self
    attribute: (identifier) @_method)
  (#eq? @_self "self")
  (#eq? @_method "skipTest")) @skip
