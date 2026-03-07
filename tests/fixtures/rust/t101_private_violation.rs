#[test]
fn test_private_field_in_assertion() {
    let user = User::new("alice");
    let name = user._name;
    let active = user._is_active;
    assert_eq!(name, "alice");
    assert!(active);
}
