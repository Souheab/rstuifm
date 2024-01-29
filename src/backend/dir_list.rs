use std::path::PathBuf;

use super::{File, Folder, Symlink};

use crate::ui::widgets::DirListUI;
use anyhow::{Context, Result};
use ratatui::widgets::{Block, Borders, List, Widget};

#[derive(Clone)]
pub struct DirList {
    folders: Vec<Folder>,
    files: Vec<File>,
    symlinks: Vec<Symlink>,
    dotfile_folders: Vec<Folder>,
    dotfile_files: Vec<File>,
    dotfile_symlinks: Vec<Symlink>,
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
        let mut dotfile_folders = Vec::new();
        let mut dotfile_files = Vec::new();
        let mut dotfile_symlinks = Vec::new();

        for entry in path
            .read_dir()
            .context("[app_backend.DirList.new()] Failed to read directory path")?
        {
            let entry = entry.context("[app_backend.DirList.new()] Failed to read DirEntry")?;
            let path = entry.path();
            let is_dotfile = path
                .file_name()
                .and_then(|name| name.to_str())
                .map_or(false, |name| name.starts_with('.'));

            if path.is_dir() {
                if is_dotfile {
                    dotfile_folders.push(Folder::new(path).unwrap());
                } else {
                    folders.push(Folder::new(path).context(
                        "[app_backend.DirList.new()] Failed to create new Folder struct",
                    )?);
                }
            } else if path.is_file() {
                if is_dotfile {
                    dotfile_files.push(File::new(path).unwrap());
                } else {
                    files.push(
                        File::new(path).context(
                            "[app_backend.DirList.new()] Failed to create new File struct",
                        )?,
                    );
                }
            } else if path.is_symlink() {
                if is_dotfile {
                    dotfile_symlinks.push(Symlink::new(path).unwrap());
                } else {
                    symlinks.push(Symlink::new(path).context(
                        "[app_backend.DirList.new()] Failed to create new Symlink struct",
                    )?);
                }
            }
        }

        Ok(DirList {
            folders,
            files,
            symlinks,
            dotfile_folders,
            dotfile_files,
            dotfile_symlinks,
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
        } else if index >= (self.files.len() + self.folders.len()) && index < self.len() {
            let symlink = self
                .symlinks
                .get(index - self.files.len() - self.folders.len())
                .unwrap()
                .clone();
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
            .map(|x| x.as_str().to_string());

        let list = List::new(items).block(Block::default().borders(Borders::ALL));
        list.render(area, buf);
    }
}
