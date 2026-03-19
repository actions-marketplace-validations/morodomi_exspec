;; var = ClassName(...)
(assignment left: (identifier) @var right: (call function: (identifier) @class))
;; var = module.ClassName(...)
(assignment left: (identifier) @var right: (call function: (attribute attribute: (identifier) @class)))
;; var = obj.method(...) -> var derives from obj
(assignment left: (identifier) @var right: (call function: (attribute object: (identifier) @source)))
;; var = await ClassName(...)
(assignment left: (identifier) @var right: (await (call function: (identifier) @class)))
