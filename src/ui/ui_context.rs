use std::io::Stdout;

use anyhow::{Context, Result};
use ratatui::{backend::CrosstermBackend, Terminal};

use super::widgets::ThreePaneLayout;
use crate::backend::DirList;

pub struct UiContext {
    pub main_ui: ThreePaneLayout,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl UiContext {
    pub fn new(dir_list: DirList) -> Result<UiContext> {
        let mut terminal = crate::ui::functions::setup_terminal()
            .context("[app.run()] Failed to setup terminal")?;

        let main_ui = ThreePaneLayout::from(dir_list);

        // Initial drawing of he screen when the program starts
        terminal
            .draw(|frame| frame.render_widget(main_ui.clone(), frame.size()))
            .context("Failed to draw initial TUI on terminal")?;

        Ok(UiContext { main_ui, terminal })
    }

    pub fn get_main_ui_ref(&self) -> &ThreePaneLayout {
        &self.main_ui
    }

    pub fn process_terminal_resize(&mut self) -> Result<()> {
        self.terminal
            .draw(|frame| {
                frame.render_widget(self.main_ui.clone(), frame.size());
            })
            .context("[app.run()] Failed to draw on terminal during resize")?;

        Ok(())
    }
}
