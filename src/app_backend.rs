use crate::ui::UiContext;
use anyhow::{anyhow, Context, Result};
use std::{path::PathBuf, collections::HashSet};

pub struct AppBackend {
    ui_context: UiContext,
    dirlist_cache: HashSet<DirList>,
    tabs: Vec<Tab>,
}

struct Tab {
    working_directory: PathBuf,
    mid_pane: DirList,
}

//TODO apply .context() to all "?"
impl Tab {
    fn from(working_directory: PathBuf, mid_pane: DirList) -> Tab {
        Tab { working_directory, mid_pane }
    }
}

// TODO: Take initial path to make a new tab when creating backend
impl AppBackend {
    pub fn new(initial_path: PathBuf) -> Result<AppBackend> {
        let main_dir_list = DirList::from(&initial_path)?;
        let ui_context = UiContext::new(main_dir_list.clone())?;
        let dirlist_cache = HashSet::new();
        let tab = Tab::from(initial_path, main_dir_list);
        let mut tabs: Vec<Tab> = Vec::new();
        tabs.push(tab);
        Ok(AppBackend { ui_context , dirlist_cache, tabs})
    }

    pub fn get_ui_context(&self) -> &UiContext {
        &self.ui_context
    }

}

#[derive(Clone)]
pub struct DirList {
    folders: Vec<Folder>,
    files: Vec<File>,
    symlinks: Vec<Symlink>,
}

impl DirList {
    fn from(path: &PathBuf) -> Result<DirList> {
        let mut folders = Vec::new();
        let mut files = Vec::new();
        let mut symlinks = Vec::new();

        for entry in path.read_dir().context("[app_backend.DirList.new()] Failed to read directory path")? {
            let entry = entry.context("[app_backend.DirList.new()] Failed to read DirEntry")?;
            let path = entry.path();
            if path.is_dir() {
                folders.push(Folder::new(path)?);
            } else if path.is_file() {
                files.push(File::new(path)?);
            } else if path.is_symlink() {
                symlinks.push(Symlink::new(path)?);
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
}

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
    fn new(path: PathBuf) -> Result<Folder> {
        let name = path
            .file_name()
            .ok_or_else(|| anyhow!("[app_backend.Folder.new()] failed to unwrap Option<&OsStr>"))?
            .to_str()
            .ok_or_else(|| anyhow!("[app_backend.Folder.new()] failed to unwrap Option<&str>"))?
            .to_string();
        Ok(Folder { path, name })
    }
}

#[derive(Clone)]
pub struct File {
    path: PathBuf,
    name: String,
}

impl ToString for File {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl File {
    fn new(path: PathBuf) -> Result<File> {
        let name = path
            .file_name()
            .ok_or_else(|| anyhow!("[app_backend.File.new()] failed to unwrap Option<&OsStr>"))?
            .to_str()
            .ok_or_else(|| anyhow!("[app_backend.File.new()] failed to unwrap Option<&str>"))?
            .to_string();
        Ok(File { path, name })
    }
}

#[derive(Clone)]
pub struct Symlink {
    path: PathBuf,
    name: String,
}

impl Symlink {
    fn new(path: PathBuf) -> Result<Symlink> {
        let name = path
            .file_name()
            .ok_or_else(|| anyhow!("[app_backend.Symlink.new()] failed to unwrap Option<&OsStr>"))?
            .to_str()
            .ok_or_else(|| anyhow!("[app_backend.Symlink.new()] failed to unwrap Option<&str>"))?
            .to_string();
        Ok(Symlink { path, name })
    }
}

impl ToString for Symlink {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}
