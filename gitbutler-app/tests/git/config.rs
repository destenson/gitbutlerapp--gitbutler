use crate::test_repository;

#[test]
pub fn test_set_str() {
    let repo = test_repository();
    let mut config = repo.config().unwrap();
    config.set_str("test.key", "test.value").unwrap();
    assert_eq!(
        config.get_string("test.key").unwrap().unwrap(),
        "test.value"
    );
}

#[test]
pub fn test_set_bool() {
    let repo = test_repository();
    let mut config = repo.config().unwrap();
    config.set_bool("test.key", true).unwrap();
    assert!(config.get_bool("test.key").unwrap().unwrap());
}

#[test]
pub fn test_get_string_none() {
    let repo = test_repository();
    let config = repo.config().unwrap();
    assert_eq!(config.get_string("test.key").unwrap(), None);
}

#[test]
pub fn test_get_bool_none() {
    let repo = test_repository();
    let config = repo.config().unwrap();
    assert_eq!(config.get_bool("test.key").unwrap(), None);
}
