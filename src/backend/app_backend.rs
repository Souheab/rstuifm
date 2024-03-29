use super::dir_list::FileSystemItem;
use super::{DirList, Tab, Tabs};
use crate::helper_functions;
use crate::ui;
use crate::ui::widgets::{RightPane, ThreePaneLayoutState};
use anyhow::{Context, Result};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::path::PathBuf;
use std::{collections::HashMap, io::Stdout};

pub struct AppBackend {
    dirlist_cache: HashMap<PathBuf, DirList>,
    pub tabs: Tabs,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl AppBackend {
    pub fn new(initial_path: PathBuf) -> Result<AppBackend> {
        let terminal = crate::ui::functions::setup_terminal()
            .context("[AppBackend.new()] Failed to setup terminal")?;
        let main_dir_list = DirList::new(&initial_path)?;
        let dirlist_cache = HashMap::new();
        let tab = Tab::new(initial_path, main_dir_list);

        let mut tabs_vec: Vec<Tab> = Vec::new();
        tabs_vec.push(tab);
        let tabs = Tabs::new(tabs_vec).context("Failed to create tabs")?;

        let mut backend = AppBackend {
            dirlist_cache,
            tabs,
            terminal,
        };

        let mut state = backend.get_new_state();

        backend
            .terminal
            .draw(|frame| {
                frame.render_stateful_widget(
                    backend.tabs.selected_tab_ref().ui.clone(),
                    frame.size(),
                    &mut state,
                );
            })
            .context("Failed to draw on terminal")?;
        Ok(backend)
    }

    pub fn exit_app(&mut self) -> Result<()> {
        ui::functions::restore_terminal(&mut self.terminal)
            .context("[app.exit()] Failed to restore terminal")?;
        std::process::exit(0);
    }

    // TODO go down to Tab and fix the selection functions
    pub fn select_next(&mut self) {
        self.tabs.selected_tab_ref_mut().ui.select_next();
        let three_pane_layout_state = self.get_new_state();

        self.draw(three_pane_layout_state);
    }

    pub fn select_previous(&mut self) {
        self.tabs.selected_tab_ref_mut().ui.select_previous();
        let three_pane_layout_state = self.get_new_state();

        self.draw(three_pane_layout_state);
    }

    // Cached way to get dirlist
    pub fn get_dirlist(&mut self, path: &PathBuf) -> Result<&DirList> {
        if !self.dirlist_cache.contains_key(path) {
            let dirlist = DirList::new(path).context("[get_dirlist] Unable to create DirList")?;
            self.dirlist_cache.insert(path.clone(), dirlist);
        }

        Ok(self.dirlist_cache.get(path).unwrap())
    }

    pub fn select_right(&mut self) {
        let selected_tab = self.tabs.selected_tab_ref_mut();
        let selected_item = selected_tab.selected_item();

        match selected_item {
            Some(selected_item) => {
                let new_path = match selected_item {
                    FileSystemItem::Folder(folder) => Some(folder.path),
                    _ => None,
                };

                if let Some(path) = new_path {
                    let new_dir_list = self.get_dirlist(&path);
                    match new_dir_list {
                        Ok(dir_list_ref) => {
                            let new_dir_list = dir_list_ref.clone();
                            let selected_tab = self.tabs.selected_tab_ref_mut();
                            selected_tab.select(path, new_dir_list)
                        }

                        Err(_) => {
                            return;
                        }
                    }
                }

                let new_state = self.get_new_state();
                self.draw(new_state);
            }
            None => (),
        }
    }

    pub fn select_left(&mut self) {
        let selected_tab = self.tabs.selected_tab_ref_mut();
        let working_dir = selected_tab.working_directory.clone();
        //TODO: Deal with the error of working dir being root
        let new_path = working_dir.parent();

        match new_path {
            Some(path) => {
                let new_path = path.to_path_buf();

                let new_dir_list = self.get_dirlist(&new_path).unwrap().clone();

                let selected_tab = self.tabs.selected_tab_ref_mut();
                selected_tab.select(new_path, new_dir_list);

                let new_state = self.get_new_state();
                self.draw(new_state);
            }

            None => {
                return;
            }
        }
    }

    pub fn get_new_state(&mut self) -> ThreePaneLayoutState {
        let working_dir = {
            let selected_tab_mut = self.tabs.selected_tab_ref_mut();
            selected_tab_mut.working_directory.clone()
        };
        let parent_dir = match working_dir.parent() {
            Some(path) => Some(path.to_path_buf()),
            None => None,
        };

        let left_pane = match parent_dir {
            Some(path) => Some(self.get_dirlist(&path).unwrap().clone()),
            None => None,
        };

        let fs_item = self.tabs.selected_tab_ref().selected_item();
        let mut right_pane = RightPane::DirList(None);

        if let Some(fs_item) = fs_item {
            right_pane = match fs_item {
                FileSystemItem::Folder(folder) => {
                    if helper_functions::can_read_directory(&folder.path) {
                        RightPane::DirList(Some(self.get_dirlist(&folder.path).unwrap().clone()))
                    } else {
                        RightPane::PermissionDenied
                    }
                }
                _ => RightPane::DirList(None),
            };
        }

        ThreePaneLayoutState::new(left_pane, right_pane)
    }

    pub fn draw(&mut self, mut state: ThreePaneLayoutState) {
        let terminal = &mut self.terminal;
        let ui = self.tabs.selected_tab_ref().ui.clone();

        let _ = terminal.draw(|f| f.render_stateful_widget(ui, f.size(), &mut state));
    }
}
