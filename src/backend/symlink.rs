use std::path::PathBuf;
use anyhow::{anyhow, Result};

#[derive(Clone)]
pub struct Symlink {
    _path: PathBuf,
    name: String,
}

impl Symlink {
    pub fn new(path: PathBuf) -> Result<Symlink> {
        let name = path
            .file_name()
            .ok_or_else(|| anyhow!("[app_backend.Symlink.new()] failed to unwrap Option<&OsStr>"))?
            .to_str()
            .ok_or_else(|| anyhow!("[app_backend.Symlink.new()] failed to unwrap Option<&str>"))?
            .to_string();
        Ok(Symlink { _path: path, name })
    }
}

impl ToString for Symlink {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}
