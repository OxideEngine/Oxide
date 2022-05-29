#[cfg(test)]
mod content_drawer {
    use std::path::{Path, PathBuf};

    use crate::{
        models::filesystem::Filesystem, viewmodels::content_drawer::ContentDrawerViewModel,
    };

    #[test]
    fn test_default_constructor() {
        let viewmodel = ContentDrawerViewModel::default();

        assert_eq!(viewmodel.current, Path::new("./"));
    }

    #[test]
    fn test_list_current() {
        let path = "./";

        let mock_result = Some(vec![
            Some(PathBuf::from("1")),
            Some(PathBuf::from("2")),
            Some(PathBuf::from("3")),
        ]);
        let mut filesystem = Filesystem::faux();

        faux::when!(filesystem.list_dir(PathBuf::from(path))).then_return(mock_result.clone());
        let viewmodel = ContentDrawerViewModel::new(PathBuf::from(path), filesystem);

        assert_eq!(mock_result, viewmodel.list_current());
    }
}
