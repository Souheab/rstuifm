use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode};
use std::{
    thread,
    time::Duration,
};

use crate::{backend::AppBackend, ui};

// TODO create a seperate channel for input events, as these may be blocking with other terminal events and need to be cleared if the channel is stacked
// TODO use bounded channels with crossbeam prolly

pub struct EventHandler {
    crossterm_event_rx: crossbeam::channel::Receiver<crossterm::event::Event>,
    input_event_rx: crossbeam::channel::Receiver<crossterm::event::KeyEvent>,
}

impl EventHandler {
    pub fn new() -> Result<EventHandler> {
        let (crossterm_event_tx, crossterm_event_rx) = crossbeam::channel::unbounded();
        let (input_event_tx, input_event_rx) = crossbeam::channel::bounded(3);

        let crossterm_event_tx_clone = crossterm_event_tx.clone();
        let input_event_tx_clone = input_event_tx.clone();

        thread::spawn(move || loop {
            if let Ok(event) = event::read() {
                match event {
                    Event::Key(key_event) => {
                        input_event_tx_clone.try_send(key_event).ok();
                    }

                    _ => {
                        crossterm_event_tx_clone.send(event.clone()).ok();
                    }
                }
            }
            thread::sleep(Duration::from_millis(30));
        });

        Ok(EventHandler {
            crossterm_event_rx,
            input_event_rx,
        })
    }

    pub fn process_events(&self, app_backend: &mut AppBackend) -> Result<()> {
        let crossterm_event = self.crossterm_event_rx.try_recv();

        let key_event = self.input_event_rx.try_recv();

        if let Ok(event) = crossterm_event {
            process_crossterm_event(event, app_backend)
                .context("Failed to process crossterm event")?;
        }

        if let Ok(key) = key_event {
            process_key_event(key, app_backend);
        }
        Ok(())
    }
}

fn process_crossterm_event(
    event: crossterm::event::Event,
    app_backend: &mut AppBackend,
) -> Result<()> {
    match event {
        Event::Resize(_width, _height) => {
            let state = app_backend.get_new_state();
            let terminal = &mut app_backend.terminal;
            let ui = &app_backend.tabs.selected_tab_ref().ui;
            ui::functions::process_terminal_resize(terminal, ui, state)
                .context("Failed to respond to terminal resize")?;
            Ok(())
        }

        _ => Ok(()),
    }
}

fn process_key_event(key: crossterm::event::KeyEvent, app_backend: &mut AppBackend) {
    match key.code {
        KeyCode::Char('q') => app_backend.exit_app().unwrap(),
        KeyCode::Char('j') => app_backend.select_next(),
        KeyCode::Char('k') => app_backend.select_previous(),
        KeyCode::Char('l') => app_backend.select_right(),
        KeyCode::Char('h') => app_backend.select_left(),
        _ => (),
    }
}
