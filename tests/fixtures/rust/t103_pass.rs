#[test]
#[should_panic(expected = "empty name")]
fn test_create_user_panics_on_empty() {
    User::new("");
}

#[test]
fn test_create_user() {
    let user = User::new("alice");
    assert_eq!(user.name(), "alice");
}
