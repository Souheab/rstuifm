use std::path::PathBuf;
use anyhow::{anyhow, Result};

#[derive(Clone)]
pub struct Folder {
    path: PathBuf,
    name: String,
}

impl ToString for Folder {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl Folder {
    pub fn new(path: PathBuf) -> Result<Folder> {
        let name = path
            .file_name()
            .ok_or_else(|| anyhow!("[app_backend.Folder.new()] failed to unwrap Option<&OsStr>"))?
            .to_str()
            .ok_or_else(|| anyhow!("[app_backend.Folder.new()] failed to unwrap Option<&str>"))?
            .to_string();
        Ok(Folder { path, name })
    }
}
