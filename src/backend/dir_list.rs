use std::path::PathBuf;

use super::{File, Folder, Symlink};

use anyhow::{Context, Result};
use ratatui::widgets::{Widget, List, Block, Borders};

#[derive(Clone)]
pub struct DirList {
    folders: Vec<Folder>,
    files: Vec<File>,
    symlinks: Vec<Symlink>,
}

pub enum FileSystemItem {
    Folder(Folder),
    File(File),
    Symlink(Symlink),
}

impl DirList {
    pub fn new(path: &PathBuf) -> Result<DirList> {
        let mut folders = Vec::new();
        let mut files = Vec::new();
        let mut symlinks = Vec::new();

        for entry in path
            .read_dir()
            .context("[app_backend.DirList.new()] Failed to read directory path")?
        {
            let entry = entry.context("[app_backend.DirList.new()] Failed to read DirEntry")?;
            let path = entry.path();
            if path.is_dir() {
                folders.push(
                    Folder::new(path).context(
                        "[app_backend.DirList.new()] Failed to create new Folder struct",
                    )?,
                );
            } else if path.is_file() {
                files.push(
                    File::new(path)
                        .context("[app_backend.DirList.new()] Failed to create new File struct")?,
                );
            } else if path.is_symlink() {
                symlinks.push(
                    Symlink::new(path).context(
                        "[app_backend.DirList.new()] Failed to create new Symlink struct",
                    )?,
                );
            }
        }

        Ok(DirList {
            folders,
            files,
            symlinks,
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
        self.folders.len() + self.files.len() + self.symlinks.len()
    }

    pub fn get(&self, index: usize) -> Option<FileSystemItem> {
        if index < self.folders.len() {
            let folder = self.folders.get(index).unwrap().clone();
            let folder = FileSystemItem::Folder(folder);
            Some(folder)
        } else if index >= self.folders.len() && index < self.folders.len() + self.files.len() {
            let file = self.files.get(index - self.folders.len()).unwrap().clone();
            let file = FileSystemItem::File(file);
            Some(file)
        } else if index >= (self.files.len() + self.folders.len()) {
            let symlink = self.symlinks.get(index - self.files.len() - self.folders.len()).unwrap().clone();
            let symlink = FileSystemItem::Symlink(symlink);
            Some(symlink)
        } else {
            None
        }
    }
}

impl Widget for DirList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let folders: Vec<String> = self.folders().iter().map(|x| x.to_string()).collect();
        let files: Vec<String> = self.files().iter().map(|x| x.to_string()).collect();
        let symlinks: Vec<String> = self.symlinks().iter().map(|x| x.to_string()).collect();

        let items = folders
            .iter()
            .chain(files.iter())
            .chain(symlinks.iter())
            .map(|x| x.as_str().to_string())
            .collect::<Vec<String>>();

        let list = List::new(items).block(Block::default().borders(Borders::ALL));
        list.render(area, buf);
    }
}
