use crossterm::event::KeyCode;
use ratatui::{Frame, backend::Backend};
use color_eyre::Result;

use crate::tui::Tui;

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

    pub async fn run(&mut self) -> color_eyre::Result<()> {
        let mut tui = Tui::new(200)?;
        
        tui.init()?;

        while self.running {
            //self.draw(frame);
            //self.event();
        }

        tui.shutdown();

        Ok(())
    }

    fn draw<B: Backend>(&mut self, frame: &mut Frame) -> Result<()> {
        Ok(())
    }

    async fn event(&mut self, key: KeyCode) -> color_eyre::Result<()> {
        Ok(())
    }

}