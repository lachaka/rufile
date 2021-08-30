use std::{io, sync::mpsc, thread, time::Duration};
use termion::{event::Key, input::TermRead};

pub struct EventConfig {
    tick_rate: Duration,
}

impl Default for EventConfig {
    fn default() -> EventConfig {
        EventConfig {
            tick_rate: Duration::from_millis(200),
        }
    }
}

pub enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Debug)]
pub struct Events {
    pub rx: mpsc::Receiver<Event<Key>>,
}

impl Events {
    pub fn new() -> Events {
        Events::from_config(EventConfig::default())
    }

    fn from_config(config: EventConfig) -> Events {
        let (tx, rx) = mpsc::channel();

        let tx_cloned = tx.clone();
        thread::spawn(move || {
            let stdin = io::stdin();

            for evt in stdin.keys() {
                if let Ok(key) = evt {
                    if let Err(err) = tx_cloned.send(Event::Input(key)) {
                        eprintln!("{}", err);
                        return;
                    }
                }
            }
        });
     
        thread::spawn(move || loop {
            if let Err(err) = tx.send(Event::Tick) {
                eprintln!("{}", err);
                break;
            }
            thread::sleep(config.tick_rate);
        });     

        Events {
            rx,
        }
    }
}
