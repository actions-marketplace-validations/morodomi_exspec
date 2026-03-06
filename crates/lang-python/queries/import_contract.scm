; Detect contract/schema library imports
; pydantic
(import_statement
  name: (dotted_name
    (identifier) @module
    (#eq? @module "pydantic"))) @contract_import

(import_from_statement
  module_name: (dotted_name
    (identifier) @module
    (#eq? @module "pydantic"))) @contract_import

; pandera
(import_statement
  name: (dotted_name
    (identifier) @module
    (#eq? @module "pandera"))) @contract_import

(import_from_statement
  module_name: (dotted_name
    (identifier) @module
    (#eq? @module "pandera"))) @contract_import

; marshmallow
(import_statement
  name: (dotted_name
    (identifier) @module
    (#eq? @module "marshmallow"))) @contract_import

(import_from_statement
  module_name: (dotted_name
    (identifier) @module
    (#eq? @module "marshmallow"))) @contract_import

; attrs
(import_statement
  name: (dotted_name
    (identifier) @module
    (#eq? @module "attrs"))) @contract_import

(import_from_statement
  module_name: (dotted_name
    (identifier) @module
    (#eq? @module "attrs"))) @contract_import

(import_statement
  name: (dotted_name
    (identifier) @module
    (#eq? @module "attr"))) @contract_import

(import_from_statement
  module_name: (dotted_name
    (identifier) @module
    (#eq? @module "attr"))) @contract_import
