use std::path::PathBuf;

use super::{Folder, File, Symlink};

use anyhow::{Context, Result};

#[derive(Clone)]
pub struct DirList {
    folders: Vec<Folder>,
    files: Vec<File>,
    symlinks: Vec<Symlink>,
    len: usize,
}

impl DirList {
    pub fn new(path: &PathBuf) -> Result<DirList> {
        let mut folders = Vec::new();
        let mut files = Vec::new();
        let mut symlinks = Vec::new();


        for entry in path.read_dir().context("[app_backend.DirList.new()] Failed to read directory path")? {
            let entry = entry.context("[app_backend.DirList.new()] Failed to read DirEntry")?;
            let path = entry.path();
            if path.is_dir() {
                folders.push(Folder::new(path).context("[app_backend.DirList.new()] Failed to create new Folder struct")?);
            } else if path.is_file() {
                files.push(File::new(path).context("[app_backend.DirList.new()] Failed to create new File struct")?);
            } else if path.is_symlink() {
                symlinks.push(Symlink::new(path).context("[app_backend.DirList.new()] Failed to create new Symlink struct")?);
            }
        }

        let len = folders.len() + files.len() + symlinks.len();

        Ok(DirList {
            folders,
            files,
            symlinks,
            len
        })
    }

    pub fn folders(&self) -> &Vec<Folder> {
        &self.folders
    }

    pub fn files(&self) -> &Vec<File> {
        &self.files
    }

    pub fn symlinks(&self) -> &Vec<Symlink> {
        &self.symlinks
    }

    pub fn len(&self) -> usize {
        self.len.clone()
    }
}
