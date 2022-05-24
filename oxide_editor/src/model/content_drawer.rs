use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub struct ContentDrawerModel<'a> {
    pub current: &'a Path,
}

impl<'a> ContentDrawerModel<'a> {
    pub fn list_current(self: Self) -> Option<Vec<Option<PathBuf>>> {
        match self.current.read_dir() {
            Ok(result) => Some(result.map(|res| match res {
                Ok(result) => Some(result.path()),
                Err(_) => None,
            }).collect()),
            Err(_) => None,
        }
    }
}

impl<'a> Default for ContentDrawerModel<'a> {
    fn default() -> Self {
        Self {
            current: Path::new("./"),
        }
    }
}
