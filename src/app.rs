use crate::backend::events::event_handler::EventHandler;
use crate::backend::AppBackend;
use anyhow::{Context, Result};
use std::{env, thread, time::Duration};

pub fn run() -> Result<()> {
    let mut app_backend = AppBackend::new(
        env::current_dir()
            .context("[app.run()] Failed to get current directory from environment")?,
    )
    .context("[app.run()] Failed to create AppBackend")?;

    let event_handler = EventHandler::new().context("Failed to make event handler")?;

    loop {
        event_handler
            .process_events(&mut app_backend)
            .context("[main app.rs loop] Failed to process events")?;

        thread::sleep(Duration::from_millis(5));
    }
}
