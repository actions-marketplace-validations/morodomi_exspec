;; 1. Exported function declarations
(export_statement
  (function_declaration
    name: (identifier) @name)) @exported_function

;; 2. Non-exported function declarations
(function_declaration
  name: (identifier) @name) @function

;; 3. Class methods
(method_definition
  name: (property_identifier) @name) @method

;; 4. Exported arrow function / function expression
(export_statement
  (lexical_declaration
    (variable_declarator
      name: (identifier) @name
      value: [(arrow_function) (function_expression)]))) @exported_arrow

;; 5. Non-exported arrow function / function expression
(lexical_declaration
  (variable_declarator
    name: (identifier) @name
    value: [(arrow_function) (function_expression)])) @arrow
