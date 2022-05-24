#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::model::content_drawer::ContentDrawerModel;

    #[test]
    fn test_default_constructor() {
        let content_drawer_model = ContentDrawerModel::default();

        assert_eq!(content_drawer_model.current, Path::new("./"))
    }
}
