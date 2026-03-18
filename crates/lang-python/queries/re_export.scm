;; Re-export patterns in __init__.py:
;;   from .module import Foo
;;   from .module import Foo, Bar
;;   from .module import *

;; Named re-export: from .module import Symbol
(import_from_statement
  module_name: (_) @from_specifier
  name: (dotted_name
    (identifier) @symbol_name))

;; Wildcard re-export: from .module import *
(import_from_statement
  module_name: (_) @from_specifier
  (wildcard_import) @wildcard)
