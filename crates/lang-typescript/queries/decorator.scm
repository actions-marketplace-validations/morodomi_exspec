;; Class-level decorator with call expression: @Controller('users')
(class_declaration
  decorator: (decorator
    (call_expression
      function: (identifier) @class_dec_name
      arguments: (arguments) @class_dec_args))
  name: (type_identifier) @class_name) @decorated_class

;; Exported class with decorator: export class UsersController { ... }
(export_statement
  (class_declaration
    decorator: (decorator
      (call_expression
        function: (identifier) @exported_class_dec_name
        arguments: (arguments) @exported_class_dec_args))
    name: (type_identifier) @exported_class_name)) @exported_decorated_class
