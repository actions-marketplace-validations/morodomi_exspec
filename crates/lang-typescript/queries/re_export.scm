;; Named re-export: export { Foo, Bar } from './module'
(export_statement
  (export_clause
    (export_specifier
      name: (identifier) @symbol_name))
  source: (string
    (string_fragment) @from_specifier))

;; Wildcard re-export: export * from './module'
(export_statement
  "*" @wildcard
  source: (string
    (string_fragment) @from_specifier))
