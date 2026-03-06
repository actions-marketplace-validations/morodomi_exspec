; MagicMock() / Mock() call
(call
  function: (identifier) @fn
  (#match? @fn "^(MagicMock|Mock)$")) @mock

; @patch decorator (attribute form: unittest.mock.patch)
(decorator
  (call
    function: (attribute
      attribute: (identifier) @attr)
    (#match? @attr "^patch"))) @mock

; @patch decorator (identifier form: from unittest.mock import patch)
(decorator
  (call
    function: (identifier) @fn2
    (#match? @fn2 "^patch$"))) @mock
