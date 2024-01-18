use anyhow::{anyhow, Result};
use std::path::PathBuf;

use super::DirList;

pub struct Tab {
    working_directory: PathBuf,
    mid_pane: DirList,
}

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
}

impl Tab {
    pub fn new(working_directory: PathBuf, mid_pane: DirList) -> Tab {
        Tab {
            working_directory,
            mid_pane,
        }
    }
}
