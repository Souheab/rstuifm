use std::io::Stdout;

use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Rect, Layout, Direction, Constraint},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListState, StatefulWidget},
    Terminal,
};

use crate::backend::DirList;

#[derive(Clone)]
pub struct DirSelectionList {
    pub state: ListState,
    pub items: DirList,
}

impl DirSelectionList {
    pub fn from(items: DirList) -> DirSelectionList {
        let mut state = ListState::default();
        state.select(Some(0));
        DirSelectionList { state, items }
    }

    pub fn select(&mut self, index: usize) {
        self.state.select(Some(index));
    }

    pub fn select_next(&mut self) {
        match self.state.selected() {
            Some(index) => {
                if self.items.len()!= 0 && index >= self.items.len() - 1 {
                    self.select(0);
                } else {
                    self.select(index + 1);
                }
            }
            None => self.select(0),
        }
    }

    pub fn select_previous(&mut self) {
        match self.state.selected() {
            Some(index) => {
                if index != 0 {
                    self.select(index - 1);
                }

                else {
                    self.select(self.items.len() - 1)
                }
            }
            None => self.select(0),
        }
    }

}

impl StatefulWidget for DirSelectionList {
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
