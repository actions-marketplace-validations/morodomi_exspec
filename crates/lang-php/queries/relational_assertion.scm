; PHP relational assertion patterns
; Presence of any of these suppresses T105

; PHPUnit: $this->assertGreaterThan(), assertLessThan(), assertContains(), etc.
(member_call_expression
  object: (variable_name) @_self
  name: (name) @_method
  (#eq? @_self "$this")
  (#match? @_method "^(assertGreaterThan|assertGreaterThanOrEqual|assertLessThan|assertLessThanOrEqual|assertContains|assertStringContainsString|assertInstanceOf|assertNull|assertNotNull|assertTrue|assertFalse|assertMatchesRegularExpression|assertCount)$")) @relational

; Pest: ->toBeGreaterThan(), ->toContain(), ->toBeTrue(), etc.
(member_call_expression
  name: (name) @_pest_method
  (#match? @_pest_method "^(toBeGreaterThan|toBeGreaterThanOrEqual|toBeLessThan|toBeLessThanOrEqual|toContain|toBeTrue|toBeFalse|toBeNull|toBeInstanceOf|toHaveCount|toMatch)$")) @relational
