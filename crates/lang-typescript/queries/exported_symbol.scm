;; Exported function declarations: export function Foo() {}
(export_statement
  (function_declaration
    name: (identifier) @symbol_name))

;; Exported variable/const declarations: export const Foo = ...
(export_statement
  (lexical_declaration
    (variable_declarator
      name: (identifier) @symbol_name)))

;; Exported class declarations: export class Foo {}
(export_statement
  (class_declaration
    name: (type_identifier) @symbol_name))

;; Exported enum declarations: export enum Foo {}
(export_statement
  (enum_declaration
    name: (identifier) @symbol_name))

;; Exported interface declarations: export interface Foo {}
(export_statement
  (interface_declaration
    name: (type_identifier) @symbol_name))

;; Exported type alias: export type Foo = ...
(export_statement
  (type_alias_declaration
    name: (type_identifier) @symbol_name))
