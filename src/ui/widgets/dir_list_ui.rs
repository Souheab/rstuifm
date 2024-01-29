use ratatui::widgets::{Widget, List, Block, Borders};

use crate::backend::{DirList, symlink::{self, Symlink}, Folder, File};

#[derive(Clone)]
pub struct DirListUI {
    pub items: Vec<String>
}

impl DirListUI {
    pub fn from(folders: &Vec<Folder>, files: &Vec<File>, symlinks: &Vec<Symlink>) -> DirListUI {
        let folders: Vec<String> = folders.iter().map(|x| x.to_string()).collect();
        let files: Vec<String> = files.iter().map(|x| x.to_string()).collect();
        let symlinks: Vec<String> = symlinks.iter().map(|x| x.to_string()).collect();


        let items = folders
            .iter()
            .chain(files.iter())
            .chain(symlinks.iter())
            .map(|x| x.as_str().to_string())
            .collect::<Vec<String>>();

        DirListUI {items}

    }
}

impl Widget for DirListUI {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let list = List::new(self.items).block(Block::default().borders(Borders::ALL));
        list.render(area, buf);
    }
}
