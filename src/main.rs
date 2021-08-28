use std::{error, io};
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

mod event;

use event::{Event, Events};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut events = Events::new();

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    
    loop {
        match events.rx.try_recv() {
            Ok(Event::Input(input)) => match input {
                Key::Char('q') | Key::Ctrl('c') => {
                    break;
                }
                _ => {}
            },
            Ok(Event::Tick) => {},
            Err(err) => {
                eprintln!("{}", err)
            }
        }
    }

    Ok(())
}