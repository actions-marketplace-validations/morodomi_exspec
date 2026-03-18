;; Next.js App Router: exported HTTP handler functions
;; Pattern 1: export [async] function GET() {}
(export_statement
  (function_declaration
    name: (identifier) @handler_name)) @exported_handler

;; Pattern 2: export const GET = [async] () => {}
(export_statement
  (lexical_declaration
    (variable_declarator
      name: (identifier) @handler_name
      value: [(arrow_function) (function_expression)]))) @exported_arrow_handler
