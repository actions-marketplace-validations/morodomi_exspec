;; pkg.Symbol -- attribute access on a module
;;
;; Matches ALL `obj.attr` expressions where `obj` is a simple identifier.
;; Does not distinguish module-level names from local variables that shadow
;; the module name (static analysis limitation). False positives are filtered
;; downstream by matching @module_name against known bare-import names.
;;
;; For dotted bare imports (`import os.path`), the import_name_parts[0] is
;; "os.path" which won't match a single identifier node, so attrs=[] (safe
;; fallback to match-all).
(attribute
  object: (identifier) @module_name
  attribute: (identifier) @attribute_name)
