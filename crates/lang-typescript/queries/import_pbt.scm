; Detect property-based testing library imports
; fast-check
(import_statement
  source: (string
    (string_fragment) @source
    (#match? @source "fast-check"))) @pbt_import

; jsverify
(import_statement
  source: (string
    (string_fragment) @source
    (#match? @source "jsverify"))) @pbt_import
