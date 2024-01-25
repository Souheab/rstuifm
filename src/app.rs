use crate::backend::events::event_handler::EventHandler;
use crate::backend::AppBackend;
use anyhow::{Context, Result};
use std::{env, path::PathBuf, thread, time::Duration};

pub fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut initial_path = env::current_dir().unwrap();

    if args.len() > 1 {
        initial_path = PathBuf::from(&args[1]);
    }

    let mut app_backend =
        AppBackend::new(initial_path).context("[app.run()] Failed to create AppBackend")?;

    let event_handler = EventHandler::new().context("Failed to make event handler")?;

    loop {
        event_handler
            .process_events(&mut app_backend)
            .context("[main app.rs loop] Failed to process events")?;

        thread::sleep(Duration::from_millis(5));
    }
}
