#[test]
fn test_create_user() {
    let service = Service::new();
    let user = service.create_user("alice");
    assert_eq!(user.name(), "alice");
}
