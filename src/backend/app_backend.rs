use crate::ui;
use anyhow::{Context, Result};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{collections::HashMap, io::Stdout};
use std::path::PathBuf;
use super::{DirList, Tab, Tabs};

pub struct AppBackend {
    dirlist_cache: HashMap<PathBuf, DirList>,
    pub tabs: Tabs,
    pub terminal: Terminal<CrosstermBackend<Stdout>>
}

impl AppBackend {
    pub fn new(initial_path: PathBuf) -> Result<AppBackend> {
        let mut terminal = crate::ui::functions::setup_terminal()
            .context("[AppBackend.new()] Failed to setup terminal")?;
        let main_dir_list = DirList::new(&initial_path)?;
        let dirlist_cache = HashMap::new();
        let tab = Tab::new(initial_path, main_dir_list);

        terminal.draw(|frame| {
            frame.render_widget(tab.ui.clone(), frame.size());
        }).context("Failed to draw on terminal")?;

        let mut tabs_vec: Vec<Tab> = Vec::new();
        tabs_vec.push(tab);
        let tabs = Tabs::new(tabs_vec, 0).context("Failed to create tabs")?;

        Ok(AppBackend {
            dirlist_cache,
            tabs,
            terminal,
        })
    }

    pub fn exit_app(&mut self) -> Result<()> {
        ui::functions::restore_terminal(&mut self.terminal)
            .context("[app.exit()] Failed to restore terminal")?;
        std::process::exit(0);
    }

    pub fn select_next(&mut self) {
        self.tabs.selected_tab_ref_mut().select_next(&mut self.terminal);
    }

    pub fn select_previous(&mut self) {
        self.tabs.selected_tab_ref_mut().select_previous(&mut self.terminal);
    }

    // Cached way to get dirlist
    pub fn get_dirlist(&mut self, path: &PathBuf) -> Result<&DirList> {
        if !self.dirlist_cache.contains_key(path) {
            let dirlist = DirList::new(path).context("[get_dirlist] Unable to create DirList")?;
            self.dirlist_cache.insert(path.clone(), dirlist);
        }

        Ok(self.dirlist_cache.get(path).unwrap())
    }

}
