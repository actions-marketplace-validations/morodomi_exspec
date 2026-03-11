; #64: PHPUnit skip-only test detection
; Broad match on method name only (no $this constraint).
; Same trade-off as Sinon .verify() in #51: false negative > false positive.
; $this->markTestSkipped()
(member_call_expression
  name: (name) @_skip_method
  (#eq? @_skip_method "markTestSkipped")) @skip

; $this->markTestIncomplete()
(member_call_expression
  name: (name) @_skip_method2
  (#eq? @_skip_method2 "markTestIncomplete")) @skip
