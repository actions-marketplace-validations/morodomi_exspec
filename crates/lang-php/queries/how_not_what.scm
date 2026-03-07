; PHPUnit mock verification patterns (how-not-what)

; ->expects(...) (PHPUnit mock verification)
(member_call_expression
  name: (name) @how_pattern
  (#eq? @how_pattern "expects"))

; ->shouldReceive(...) (Mockery)
(member_call_expression
  name: (name) @how_pattern
  (#eq? @how_pattern "shouldReceive"))

; ->shouldHaveReceived(...) (Mockery)
(member_call_expression
  name: (name) @how_pattern
  (#eq? @how_pattern "shouldHaveReceived"))
