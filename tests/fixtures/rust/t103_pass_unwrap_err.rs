#[test]
fn test_create_user_error() {
    let err = User::new("").unwrap_err();
    assert_eq!(err.to_string(), "empty name");
}

#[test]
fn test_create_user() {
    let user = User::new("alice").unwrap();
    assert_eq!(user.name(), "alice");
}
