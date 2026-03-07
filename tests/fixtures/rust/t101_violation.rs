#[test]
fn test_service_calls_repository() {
    let mut mock = MockRepository::new();
    mock.expect_save()
        .times(1)
        .returning(|_| Ok(()));

    let service = Service::new(mock);
    service.create_user("alice");
}
