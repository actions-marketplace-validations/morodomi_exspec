;; Match jest.fn(), jest.mock(), jest.spyOn(), vi.fn(), vi.mock(), vi.spyOn()
;; sinon.stub(), sinon.spy(), sinon.mock()
(call_expression
  function: (member_expression
    object: (identifier) @obj
    property: (property_identifier) @method)
  (#match? @obj "^(jest|vi|sinon)$")
  (#match? @method "^(fn|mock|spyOn|stub|spy)$")) @mock
