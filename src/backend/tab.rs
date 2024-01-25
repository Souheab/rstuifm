use anyhow::{anyhow, Result};
use std::path::PathBuf;
use crate::ui::widgets::{ThreePaneLayout, DirSelectionList};
use super::{DirList, dir_list::FileSystemItem};

pub struct Tab {
    pub working_directory: PathBuf,
    pub ui: ThreePaneLayout,
}

pub struct Tabs {
    tabs_vec: Vec<Tab>,
    selected_index: usize,
}

impl Tabs {
    pub fn new(tabs_vec: Vec<Tab>) -> Result<Tabs> {

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
        let ui = ThreePaneLayout::new(dir_list);

        Tab {
            working_directory,
            ui,
        }
    }

    pub fn selected_item(&self) -> FileSystemItem {
        let index = self.ui.mid_pane.state;
        self.ui.mid_pane.items.get(index).unwrap()
    }


    pub fn select(&mut self, new_path: PathBuf, new_dir_list: DirList) {
        self.working_directory = new_path;
        self.ui.mid_pane = DirSelectionList::from(new_dir_list);
    }

}
