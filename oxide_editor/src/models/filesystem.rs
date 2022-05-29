use std::path::PathBuf;

#[cfg_attr(test, faux::create)]
#[derive(Debug, Default)]
pub struct Filesystem;

#[cfg_attr(test, faux::methods)]
impl Filesystem {
    pub fn list_dir(&self, target: &PathBuf) -> Option<Vec<Option<PathBuf>>> {
        match target.read_dir() {
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
