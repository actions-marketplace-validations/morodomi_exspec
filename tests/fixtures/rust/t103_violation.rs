#[test]
fn test_create_user() {
    let user = User::new("alice");
    assert_eq!(user.name(), "alice");
}

#[test]
fn test_delete_user() {
    let result = User::delete(1);
    assert!(result);
}
