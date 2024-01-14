use crate::{
    app_backend::AppBackend,
    ui::{self, restore_terminal},
};
use std::env;
use std::io;

use crossterm::event::{self, Event, KeyCode};

use anyhow::{Context, Result};
use ratatui::{backend::CrosstermBackend, Terminal};

pub fn run() -> Result<()> {
    let mut terminal = ui::setup_terminal().context("[app.run()] Failed to setup terminal")?;
    let app_backend = AppBackend::new(
        env::current_dir()
            .context("[app.run()] Failed to get current directory from environment")?,
    )
    .context("[app.run()] Failed to create AppBackend")?;
    let ui_context = app_backend.get_ui_context();

    terminal
        .draw(|frame| {
            frame.render_widget(ui_context.main_ui.clone(), frame.size());
        })
        .context("[app.run()] Failed to draw on terminal")?;

    loop {
        // Handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('q') {
                        return exit(terminal);
                    }
                }

                Event::Resize(_width, _height) => {
                    terminal
                        .draw(|frame| {
                            frame.render_widget(ui_context.main_ui.clone(), frame.size());
                        })
                        .context("[app.run()] Failed to draw on terminal during resize")?;
                }

                _ => {}
            }
        }
    }
}

fn exit(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    restore_terminal(&mut terminal).context("[app.exit()] Failed to restore terminal")?;
    Ok(())
}
