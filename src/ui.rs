use std::io;

use anyhow::{Context, Result};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListState, StatefulWidget, Widget},
    Terminal,
};

//TODO maybe organize stuff to seperate widgets into different files and other stuff throughtout the app

use crate::app_backend::DirList;

// Can store widgets here!
#[derive(Clone)]
pub struct ThreePaneLayoutWidget {
    dir_selection_list: DirSelectionListWidget,
}

#[derive(Clone)]
pub struct DirSelectionListWidget {
    pub state: ListState,
    pub items: DirList,
}

impl DirSelectionListWidget {
    pub fn from(items: DirList) -> DirSelectionListWidget {
        let mut state = ListState::default();
        state.select(Some(0));
        DirSelectionListWidget { state, items }
    }
}

impl StatefulWidget for DirSelectionListWidget {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Turn DirList into a list of strings
        let folders: Vec<String> = self.items.folders().iter().map(|x| x.to_string()).collect();
        let files: Vec<String> = self.items.files().iter().map(|x| x.to_string()).collect();
        let symlinks: Vec<String> = self
            .items
            .symlinks()
            .iter()
            .map(|x| x.to_string())
            .collect();

        let items = folders
            .iter()
            .chain(files.iter())
            .chain(symlinks.iter())
            .map(|x| x.as_str().to_string())
            .collect::<Vec<String>>();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        StatefulWidget::render(list, area, buf, state)
    }
}

impl ThreePaneLayoutWidget {
    pub fn from(dir_list: DirList) -> Result<ThreePaneLayoutWidget> {
        Ok(ThreePaneLayoutWidget {
            dir_selection_list: DirSelectionListWidget::from(dir_list),
        })
    }
}

// TODO: UI Idea the widget renders everything but I will write seperate functions to render different things
impl Widget for ThreePaneLayoutWidget {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ]
                .as_ref(),
            )
            .split(area);

        for chunk in chunks.iter() {
            block.clone().render(*chunk, buf);
        }

        let state = ListState::default();

        self.dir_selection_list
            .clone()
            .render(chunks[1], buf, &mut self.dir_selection_list.state);
    }
}

pub struct UiContext {
    pub main_ui: ThreePaneLayoutWidget,
}

impl UiContext {
    pub fn new(dir_list: DirList) -> Result<UiContext> {
        Ok(UiContext {
            main_ui: ThreePaneLayoutWidget::from(dir_list)?,
        })
    }

    pub fn get_main_ui_ref(&self) -> &ThreePaneLayoutWidget {
        &self.main_ui
    }
}

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("[ui.setup_terminal()] Failed to enable crossterm raw mode")?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))
        .context("[ui.setup_terminal()] Failed to create new ratatui terminal")?)
}

pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode().context("[ui.restore_terminal()] Failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)
        .context("Failed to leave crossterm alternate screen")?;
    Ok(terminal
        .show_cursor()
        .context("Failed to show crossterm cursor")?)
}
