; Detect contract/schema library imports
; zod
(import_statement
  source: (string
    (string_fragment) @source
    (#eq? @source "zod"))) @contract_import

; yup
(import_statement
  source: (string
    (string_fragment) @source
    (#eq? @source "yup"))) @contract_import

; io-ts
(import_statement
  source: (string
    (string_fragment) @source
    (#match? @source "io-ts"))) @contract_import

; ajv
(import_statement
  source: (string
    (string_fragment) @source
    (#eq? @source "ajv"))) @contract_import
