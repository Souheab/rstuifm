use anyhow::{anyhow, Result};
use ratatui::{widgets::ListState, Terminal, backend::CrosstermBackend};
use std::{path::PathBuf, io::Stdout};

use crate::ui::widgets::ThreePaneLayout;

use super::DirList;

pub struct Tab {
    working_directory: PathBuf,
    pub ui: ThreePaneLayout,
}
// TODO: Perhaps add UI stuff to Tab struct?
pub struct Tabs {
    tabs_vec: Vec<Tab>,
    selected_index: usize,
}

impl Tabs {
    pub fn new(tabs_vec: Vec<Tab>, selected_index: usize) -> Result<Tabs> {

        if tabs_vec.is_empty() {
            Err(anyhow!("Tabs Vec is empty, there must be at least one tab"))
        } else {
            Ok(Tabs {
                tabs_vec,
                selected_index: 0,
            })
        }
    }

    pub fn selected_tab_ref(&self) -> &Tab {
        self.tabs_vec.get(self.selected_index).unwrap()
    }

    pub fn selected_tab_ref_mut(&mut self) -> &mut Tab {
        self.tabs_vec.get_mut(self.selected_index).unwrap()
    }

}

impl Tab {
    pub fn new(working_directory: PathBuf, dir_list: DirList) -> Tab {
        let selected_item = ListState::default();
        let ui = ThreePaneLayout::new(dir_list);

        Tab {
            working_directory,
            ui,
        }
    }

    pub fn select_next(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        self.ui.select_next(terminal);
    }

    pub fn select_previous(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        self.ui.select_previous(terminal);
    }
}
