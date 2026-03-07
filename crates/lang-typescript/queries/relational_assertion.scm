; TypeScript relational assertion patterns
; Presence of any of these suppresses T105

; expect(...).toBeGreaterThan(), .toBeGreaterThanOrEqual(), .toBeLessThan(), .toBeLessThanOrEqual()
; .toContain(), .toContainEqual(), .toMatch(), .toMatchObject()
; .toBeInstanceOf(), .toBeCloseTo(), .toHaveLength()
; .toBeNull(), .toBeUndefined(), .toBeDefined()
; .toBeTruthy(), .toBeFalsy(), .toBeNaN()
(call_expression
  function: (member_expression
    property: (property_identifier) @_method)
  (#match? @_method "^(toBeGreaterThan|toBeGreaterThanOrEqual|toBeLessThan|toBeLessThanOrEqual|toContain|toContainEqual|toMatch|toMatchObject|toBeInstanceOf|toBeCloseTo|toHaveLength|toBeNull|toBeUndefined|toBeDefined|toBeTruthy|toBeFalsy|toBeNaN)$")) @relational
