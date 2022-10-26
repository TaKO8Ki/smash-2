use crossterm::event::{Event as TermEvent, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::time::Duration;
use tracing::debug;
use tracing_subscriber::{self, fmt, prelude::*, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    enable_raw_mode().ok();

    debug!("start");
    'main: loop {
        match crossterm::event::poll(Duration::from_millis(100)) {
            Ok(true) => loop {
                if let Ok(TermEvent::Key(ev)) = crossterm::event::read() {
                    match (ev.code, ev.modifiers) {
                        (KeyCode::Char('q'), KeyModifiers::NONE) => break 'main,
                        _ => (),
                    }
                }

                match crossterm::event::poll(Duration::from_millis(0)) {
                    Ok(true) => (),
                    _ => break,
                }
            },
            _ => (),
        }
    }

    disable_raw_mode().ok();
}
