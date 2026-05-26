use std::{sync::mpsc, thread, time::Duration};
use crossterm::event::{self, KeyEvent, KeyEventKind};
use ratatui::crossterm;

//|-----------------{Events of the app and console >ᴗ<}------------------|
pub enum EventApp {
    Key(KeyEvent),
    Tick,
    //Render,
}

//|-----------------{Event handler ( . .)φ}------------------|
pub struct EventHndl {
    sender: mpsc::Sender<EventApp>,
    reciver: mpsc::Receiver<EventApp>,
}

impl EventHndl {
    pub fn new(tick_rate: Duration) -> EventHndl {
        Self::hndl_build(tick_rate)
    }

    fn hndl_build(tick_rate: Duration) -> EventHndl {
        let (sx, rx) = mpsc::channel();

        let event_sx = sx.clone();

        thread::spawn(move || loop{
            if event::poll(tick_rate).unwrap() {
                let event = event::read().unwrap();
                match event {
                    event::Event::Key(key) if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Release => {
                        event_sx.send(EventApp::Key(key)).unwrap();
                    },
                    _ => continue,
                }
            }
            event_sx.send(EventApp::Tick).unwrap();
        });

        EventHndl { sender: sx, reciver: rx }
    }

    pub async fn next(&self) -> Option<EventApp> {
        self.reciver.recv().ok()
    }
}