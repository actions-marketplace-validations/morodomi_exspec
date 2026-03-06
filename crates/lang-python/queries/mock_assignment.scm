; mock_xxx = MagicMock() / Mock()
(assignment
  left: (identifier) @var_name
  right: (call
    function: (identifier) @fn
    (#match? @fn "^(MagicMock|Mock)$"))) @mock_assignment
