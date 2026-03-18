;; FastAPI route decorator: @app.get("/path") def handler(): ...
;; or: @app.get("/path") async def handler(): ...
;; Captures: route.object, route.method, route.path (optional), route.handler
;;
;; Pattern 1: path is a string literal, sync or async def
(decorated_definition
  (decorator
    (call
      function: (attribute
        object: (identifier) @route.object
        attribute: (identifier) @route.method)
      arguments: (argument_list
        (string) @route.path)))
  definition: (_
    name: (identifier) @route.handler))

;; Pattern 2: path is not a string literal (dynamic), sync or async def
(decorated_definition
  (decorator
    (call
      function: (attribute
        object: (identifier) @route.object
        attribute: (identifier) @route.method)
      arguments: (argument_list
        (identifier) @route.path)))
  definition: (_
    name: (identifier) @route.handler))
