#[test]
fn test_add() {
    assert_eq!(add(2, 2), 4);
}

#[test]
fn test_contains() {
    let v = vec![1, 2, 3];
    let result = v.contains(&2);
    assert!(result);
}

#[test]
fn test_starts_with() {
    let s = String::from("hello world");
    let result = s.starts_with("hello");
    assert!(result);
}

#[test]
fn test_is_some() {
    let opt = Some(42);
    let check = opt.is_some();
    assert!(check);
}

#[test]
fn test_is_ok() {
    let result: Result<i32, String> = Ok(42);
    let check = result.is_ok();
    assert!(check);
}
