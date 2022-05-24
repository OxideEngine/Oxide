use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct ContentDrawerModel<'a> {
    pub current: &'a Path,
}

impl<'a> ContentDrawerModel<'a> {}

impl<'a> Default for ContentDrawerModel<'a> {
    fn default() -> Self {
        Self {
            current: Path::new("./"),
        }
    }
}
