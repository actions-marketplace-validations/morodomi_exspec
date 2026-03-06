; Detect property-based testing library imports
; hypothesis
(import_statement
  name: (dotted_name
    (identifier) @module
    (#eq? @module "hypothesis"))) @pbt_import

(import_from_statement
  module_name: (dotted_name
    (identifier) @module
    (#eq? @module "hypothesis"))) @pbt_import

; schemathesis
(import_statement
  name: (dotted_name
    (identifier) @module
    (#eq? @module "schemathesis"))) @pbt_import

(import_from_statement
  module_name: (dotted_name
    (identifier) @module
    (#eq? @module "schemathesis"))) @pbt_import
