use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Paragraph, Widget},
};

use crate::backend::DirList;

#[derive(Clone)]
pub enum RightPane {
    DirList(Option<DirList>),
    PermissionDenied,
}

impl Widget for RightPane {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        match self {
            RightPane::DirList(possible_dir_list) => {
                if let Some(dir_list) = possible_dir_list {
                    dir_list.render(area, buf);
                }
            }
            RightPane::PermissionDenied => {
                let paragraph = Paragraph::new("Permission Denied").alignment(Alignment::Center);
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Ratio(1, 10),
                            Constraint::Ratio(1, 10),
                            Constraint::Ratio(8, 10),
                        ]
                        .as_ref(),
                    )
                    .split(area);

                paragraph.render(chunks[1], buf);
            }
        }
    }
}
