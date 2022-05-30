use crate::models::filesystem::Filesystem;
use std::path::PathBuf;

#[test]
fn test_list_dir() {
    let mut target_path = PathBuf::new();
    target_path.push("./");
    let filesystem = Filesystem::default();

    let expected: Vec<Option<PathBuf>> = target_path
        .as_path()
        .read_dir()
        .unwrap()
        .map(|content| match content {
            Ok(content) => Some(content.path()),
            Err(_) => None,
        })
        .collect();
    assert_eq!(expected, filesystem.list_dir(&target_path).unwrap())
}
