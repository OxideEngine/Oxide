use std::path::{Path, PathBuf};

use crate::models::filesystem::Filesystem;

pub struct ContentDrawerViewModel {
    pub current: PathBuf,
    filesystem: Filesystem,
}

impl ContentDrawerViewModel {
    pub fn new(current: PathBuf, filesystem: Filesystem) -> Self {
        Self {
            current,
            filesystem,
        }
    }

    pub fn list_current(&self) -> Option<Vec<Option<PathBuf>>> {
        self.filesystem.list_dir(&self.current)
    }
}

impl Default for ContentDrawerViewModel {
    fn default() -> Self {
        Self {
            current: Path::new("./").to_path_buf(),
            filesystem: Filesystem::default(),
        }
    }
}
