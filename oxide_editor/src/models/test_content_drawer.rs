#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::models::content_drawer::ContentDrawerModel;

    #[test]
    fn test_default_constructor() {
        let content_drawer_model = ContentDrawerModel::default();

        assert_eq!(content_drawer_model.current, Path::new("./"));
    }

    #[test]
    fn test_list_current() {
        let content_drawer_model = ContentDrawerModel::default();
        let list = content_drawer_model.list_current().unwrap();

        let expected: Vec<Option<PathBuf>> = Path::new("./")
            .read_dir()
            .unwrap()
            .map(|content| match content {
                Ok(content) => Some(content.path()),
                Err(_) => None,
            })
            .collect();

        assert_eq!(expected, list);
    }
}
