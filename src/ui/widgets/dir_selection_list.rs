use std::io::Stdout;

use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListState, StatefulWidget},
    Terminal,
};

use crate::backend::DirList;

#[derive(Clone)]
pub struct DirSelectionList {
    pub state: usize,
    pub items: DirList,
}

impl DirSelectionList {
    pub fn from(items: DirList) -> DirSelectionList {
        DirSelectionList { state: 0, items }
    }

    pub fn select(&mut self, index: usize) {
        self.state = index;
    }

    pub fn select_next(&mut self) {
        if self.items.len() != 0 && self.state >= self.items.len() - 1 {
            self.select(0);
        } else {
            self.select(self.state + 1);
        }
    }

    pub fn select_previous(&mut self) {
        if self.state != 0 {
            self.select(self.state - 1);
        } else {
            self.select(self.items.len() - 1)
        }
    }
}

impl StatefulWidget for DirSelectionList {
    type State = usize;
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
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    //TODO: maybe get colors from the terminal colors?
                    .bg(ratatui::style::Color::Blue)
                    .fg(ratatui::style::Color::Rgb(0, 0, 0)),
            );
        let mut list_state = ListState::default();
        list_state.select(Some(self.state));
        StatefulWidget::render(list, area, buf, &mut list_state)
    }
}
