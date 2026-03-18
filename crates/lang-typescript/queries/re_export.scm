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

;; Namespace re-export: export * as Ns from './module'
(export_statement
  (namespace_export) @ns_wildcard
  source: (string
    (string_fragment) @from_specifier))
