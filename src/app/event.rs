use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct EventHandler {
    rx: mpsc::Receiver<Event>,
}

#[derive(Debug, Clone)]
pub enum Event {
    Tick,
    Input(crossterm::event::KeyEvent),
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || loop {
            if crossterm::event::poll(tick_rate).unwrap_or(false) {
                if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                    if tx.send(Event::Input(key)).is_err() {
                        return;
                    }
                }
            }
            if tx.send(Event::Tick).is_err() {
                return;
            }
        });

        Self { rx }
    }

    pub fn next(&self) -> Option<Event> {
        self.rx.try_recv().ok()
    }
}
