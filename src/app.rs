use crossterm::event::KeyCode;
use ratatui::{Frame, backend::Backend};
use color_eyre::Result;

pub enum Focus {
    Entry,
    Btn,
    Info,
    //ErrMsgs,
    Table,
    Props,
}

pub struct App {
    running: bool,
    focus: Focus,
}


impl App {
    pub fn new() -> App {
        App { running: true, focus: Focus::Entry }
    }

    pub fn draw<B: Backend>(&mut self, frame: &mut Frame) -> Result<()> {
        Ok(())
    }

    pub async fn event(&mut self, key: KeyCode) -> color_eyre::Result<()> {
        Ok(())
    }
}