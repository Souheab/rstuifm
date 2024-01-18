use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode};
use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::{backend::AppBackend, ui};


pub struct EventHandler {
    crossterm_event_tx: mpsc::Sender<crossterm::event::Event>,
    crossterm_event_rx: mpsc::Receiver<crossterm::event::Event>,
    event_listener: JoinHandle<()>,
}

impl EventHandler {
    pub fn new() -> Result<EventHandler> {
        let (crossterm_event_tx, crossterm_event_rx) = mpsc::channel();

        let crossterm_event_tx_clone = crossterm_event_tx.clone();

        let event_listener = thread::spawn(move || loop {
            if let Ok(event) = event::read() {
                crossterm_event_tx_clone.send(event).expect("Unable to send message to crossterm_event mpsc while inside event_listener thread");
            }
            thread::sleep(Duration::from_millis(50));
        });

        Ok(EventHandler {
            crossterm_event_tx,
            crossterm_event_rx,
            event_listener,
        })
    }

    pub fn process_events(&self, app_backend: &mut AppBackend) -> Result<()> {
        let event = self
            .crossterm_event_rx
            .recv()
            .context("[process_events] Failed to receive from mpsc channel")?;
        process_crossterm_event(event, app_backend).context("Failed to process crossterm event")?;
        Ok(())
    }
}

fn process_crossterm_event(
    event: crossterm::event::Event,
    app_backend: &mut AppBackend,
) -> Result<()> {
    match event {
        Event::Key(key) => {
            process_key_event(key, app_backend);
            Ok(())
        }

        Event::Resize(_width, _height) => {
            let terminal = &mut app_backend.terminal;
            let ui = &app_backend.tabs.selected_tab_ref().ui;
            ui::functions::process_terminal_resize(terminal, ui).context("Failed to respond to terminal resize")?;
            Ok(())
        }

        _ => Ok(()),
    }
}

fn process_key_event(key: crossterm::event::KeyEvent, app_backend: &mut AppBackend) {
    if key.code == KeyCode::Char('q') {
        app_backend.exit_app().unwrap();
    }

    if key.code == KeyCode::Char('j') {
        app_backend.select_next();
    }

    if key.code == KeyCode::Char('k') {
        app_backend.select_previous();
    }
}
