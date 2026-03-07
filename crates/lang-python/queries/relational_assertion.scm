; Python relational assertion patterns
; Presence of any of these suppresses T105

; assert x > y
(assert_statement
  (comparison_operator
    operators: ">" @relational))

; assert x < y
(assert_statement
  (comparison_operator
    operators: "<" @relational))

; assert x >= y
(assert_statement
  (comparison_operator
    operators: ">=" @relational))

; assert x <= y
(assert_statement
  (comparison_operator
    operators: "<=" @relational))

; assert x in collection
(assert_statement
  (comparison_operator
    operators: "in" @relational))

; assert x not in collection
(assert_statement
  (comparison_operator
    operators: "not in" @relational))

; assert x is y / assert x is not y
(assert_statement
  (comparison_operator
    operators: "is" @relational))

; assert x is not y
(assert_statement
  (comparison_operator
    operators: "is not" @relational))

; pytest.approx()
(call
  function: (attribute
    object: (identifier) @_obj
    attribute: (identifier) @_method)
  (#eq? @_obj "pytest")
  (#eq? @_method "approx")) @relational

; unittest relational methods:
; assertGreater, assertGreaterEqual, assertLess, assertLessEqual
; assertIn, assertNotIn, assertIsInstance
; assertAlmostEqual, assertRegex
; assertIsNone, assertIsNotNone
; assertTrue, assertFalse
(call
  function: (attribute
    object: (identifier) @_self
    attribute: (identifier) @_meth)
  (#eq? @_self "self")
  (#match? @_meth "^(assertGreater|assertGreaterEqual|assertLess|assertLessEqual|assertIn|assertNotIn|assertIsInstance|assertAlmostEqual|assertRegex|assertIsNone|assertIsNotNone|assertTrue|assertFalse)$")) @relational
