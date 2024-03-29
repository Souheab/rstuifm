use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::backend::DirList;

use super::{DirSelectionList, RightPane};

#[derive(Clone)]
pub struct ThreePaneLayout {
    pub mid_pane: Option<DirSelectionList>,
    pub dot_files_enabled: bool,
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
            mid_pane: Some(DirSelectionList::from(dir_list)),
            dot_files_enabled: true
        }
    }

    pub fn default() -> ThreePaneLayout {
        ThreePaneLayout { mid_pane: None , dot_files_enabled: true}
    }

    pub fn select_next(&mut self) {
        if let Some(mid_pane) = &mut self.mid_pane {
            mid_pane.select_next();
        }
    }

    pub fn select_previous(&mut self) {
        if let Some(mid_pane) = &mut self.mid_pane {
            mid_pane.select_previous();
        }
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
                    Constraint::Percentage(20),
                    Constraint::Percentage(30),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(area);

        for chunk in chunks.iter() {
            block.clone().render(*chunk, buf);
        }

        if let Some(mid_pane) = self.mid_pane {
            mid_pane.clone().render(chunks[1], buf);
        }

        let left_pane = state.left_pane.clone();
        let right_pane = state.right_pane.clone();

        if let Some(dir_list) = left_pane {
            dir_list.render(chunks[0], buf)
        }

        right_pane.render(chunks[2], buf);
    }
}
