#[test]
fn test_mixed_lets() {
    // Setup (fixture-like) — should be counted
    let db = MockDb::new();                          // scoped_identifier call
    let config = Config::default();                  // scoped_identifier call
    let service = UserService::new(db, config);      // scoped_identifier call
    let input = CreateUserInput { name: "alice".into() }; // struct_expression
    let data = vec![1, 2, 3];                        // macro_invocation
    let builder_result = Config::builder().timeout(30).build(); // chain root = scoped_identifier

    // Action/prep (not fixture) — should NOT be counted
    let result = service.create(input);              // method_call on local
    let user = result.unwrap();                      // method_call on local
    let name = user.get_name();                      // method_call on local

    assert_eq!(name, "alice");
}
