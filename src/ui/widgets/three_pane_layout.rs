use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Rect, Layout},
    widgets::{Block, Borders, ListState, StatefulWidget, Widget},
};

use crate::backend::DirList;

use super::DirSelectionList;

#[derive(Clone)]
pub struct ThreePaneLayout {
    dir_selection_list: DirSelectionList,
}

impl ThreePaneLayout {
    pub fn from(dir_list: DirList) -> ThreePaneLayout {
        ThreePaneLayout {
            dir_selection_list: DirSelectionList::from(dir_list),
        }
    }
}

impl Widget for ThreePaneLayout {
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

        self.dir_selection_list
            .clone()
            .render(chunks[1], buf, &mut self.dir_selection_list.state);
    }
}
