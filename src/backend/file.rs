use std::path::PathBuf;
use anyhow::{anyhow, Result};

#[derive(Clone)]
pub struct File {
    _path: PathBuf,
    name: String,
}

impl ToString for File {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl File {
    pub fn new(path: PathBuf) -> Result<File> {
        let name = path
            .file_name()
            .ok_or_else(|| anyhow!("[app_backend.File.new()] failed to unwrap Option<&OsStr>"))?
            .to_str()
            .ok_or_else(|| anyhow!("[app_backend.File.new()] failed to unwrap Option<&str>"))?
            .to_string();
        Ok(File { _path: path, name })
    }
}
