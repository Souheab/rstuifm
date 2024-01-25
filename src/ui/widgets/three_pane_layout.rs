use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::backend::DirList;

use super::{DirSelectionList, RightPane};

#[derive(Clone)]
pub struct ThreePaneLayout {
    pub mid_pane: DirSelectionList,
}

pub struct ThreePaneLayoutState {
    left_pane: Option<DirList>,
    right_pane: RightPane,
}

impl ThreePaneLayoutState {
    pub fn new(left_pane: Option<DirList>, right_pane: RightPane) -> ThreePaneLayoutState {
        ThreePaneLayoutState {
            left_pane,
            right_pane,
        }
    }
}

impl ThreePaneLayout {
    pub fn new(dir_list: DirList) -> ThreePaneLayout {
        ThreePaneLayout {
            mid_pane: DirSelectionList::from(dir_list),
        }
    }

    pub fn select_next(&mut self) {
        self.mid_pane.select_next();
    }

    pub fn select_previous(&mut self) {
        self.mid_pane.select_previous();
    }
}

impl StatefulWidget for ThreePaneLayout {
    type State = ThreePaneLayoutState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
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

        self.mid_pane
            .clone()
            .render(chunks[1], buf);

        let left_pane = state.left_pane.clone();
        let right_pane = state.right_pane.clone();

        if let Some(dir_list) = left_pane {
            dir_list.render(chunks[0], buf)
        }

        right_pane.render(chunks[2], buf);
    }
}
