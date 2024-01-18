use super::events::{event_handler, RstuifmEvent};
use crate::ui::{self, UiContext};
use anyhow::{Context, Result};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::collections::HashMap;
use std::io::Stdout;
use std::rc::Rc;
use std::sync::mpsc;
use std::{collections::HashSet, path::PathBuf};

use super::events::event_handler::EventHandler;
use super::{DirList, Tab, Tabs};

pub struct AppBackend {
    ui_context: UiContext,
    dirlist_cache: HashMap<PathBuf, DirList>,
    tabs: Tabs,
}

impl AppBackend {
    pub fn new(initial_path: PathBuf) -> Result<AppBackend> {
        let main_dir_list = DirList::new(&initial_path)?;
        let ui_context =
            UiContext::new(main_dir_list.clone()).context("Failed to make UiContext")?;
        let dirlist_cache = HashMap::new();
        let tab = Tab::new(initial_path, main_dir_list);
        let mut tabs_vec: Vec<Tab> = Vec::new();
        tabs_vec.push(tab);
        let tabs = Tabs::new(tabs_vec, 0).context("Failed to create tabs")?;

        Ok(AppBackend {
            ui_context,
            dirlist_cache,
            tabs,
        })
    }

    pub fn ui_context_ref(&self) -> &UiContext {
        &self.ui_context
    }

    pub fn ui_context_mut_ref(&mut self) -> &mut UiContext {
        &mut self.ui_context
    }

    pub fn exit_app(&mut self) -> Result<()> {
        ui::functions::restore_terminal(&mut self.ui_context.terminal)
            .context("[app.exit()] Failed to restore terminal")?;
        std::process::exit(0);
    }

}
