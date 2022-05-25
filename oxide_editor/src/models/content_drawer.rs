use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub struct ContentDrawerModel {
    pub current: PathBuf,
}

impl ContentDrawerModel {
    pub fn list_current(self: Self) -> Option<Vec<Option<PathBuf>>> {
        match self.current.read_dir() {
            Ok(result) => Some(
                result
                    .map(|res| match res {
                        Ok(result) => Some(result.path()),
                        Err(_) => None,
                    })
                    .collect(),
            ),
            Err(_) => None,
        }
    }
}

impl Default for ContentDrawerModel {
    fn default() -> Self {
        Self {
            current: Path::new("./").to_path_buf(),
        }
    }
}
