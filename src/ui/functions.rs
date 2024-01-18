use std::io;

use anyhow::{Context, Result};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use super::widgets::ThreePaneLayout;

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

pub fn process_terminal_resize(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ui: &ThreePaneLayout,
) -> Result<()> {
    terminal
        .draw(|frame| {
            frame.render_widget(ui.clone(), frame.size());
        })
        .context("[app.run()] Failed to draw on terminal during resize")?;

    Ok(())
}
