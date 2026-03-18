;; Django URL conf route extraction
;; Note: call expressions (e.g. include()) are intentionally excluded —
;; only (attribute) and (identifier) handlers are captured.

;; Pattern 1: path/re_path with attribute handler (views.func)
(call
  function: (identifier) @django.func
  arguments: (argument_list
    (string) @django.path
    (attribute
      attribute: (identifier) @django.handler))
  (#match? @django.func "^(path|re_path)$"))

;; Django URL conf: path/re_path with identifier handler (direct import)
(call
  function: (identifier) @django.func
  arguments: (argument_list
    (string) @django.path
    (identifier) @django.handler)
  (#match? @django.func "^(path|re_path)$"))
