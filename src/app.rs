use crossterm::event::{self, KeyCode, KeyModifiers};
use ratatui::{Frame, backend::Backend, widgets::{self, Block, Borders, Paragraph}};
use ratatui::style::Style;
use color_eyre::Result;

use crate::{event::EventApp, tui::Tui};

#[derive(Default, Debug, PartialEq, Eq)]
pub enum Focus {
    #[default]
    Entry,
    Btn,
    Info,
    Table,
    Props,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Main,
    Info,
}

pub struct App {
    running: bool,
    comp_focus: Focus,
    screen_focus: Screen,
}


impl App {
    pub fn new() -> App {
        App { running: true, comp_focus: Focus::default(), screen_focus: Screen::default() }
    }

    pub async fn run(&mut self) -> color_eyre::Result<()> {
        let mut tui = Tui::new(200)?;
        
        tui.init()?;

        while self.running {
            tui.terminal.draw(|f| {
                let _ = self.draw(f);
            })?;
            
            self.event(&mut tui).await?;
        }

        tui.shutdown();

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) -> Result<()> {

        let text = Paragraph::new(format!("counter: {}", 0))
        .block(Block::default().borders(Borders::ALL)
        .title("counter")).style(Style::default().bg(ratatui::style::Color::Blue));

        frame.render_widget(widgets::Clear, frame.area());
        frame.render_widget(text, frame.area());

        Ok(())
    }

    async fn event(&mut self, tui: &mut Tui) -> color_eyre::Result<()> {

        let Some(event) = tui.event_hndl.next().await else {
            return Ok(());
        };

        match self.screen_focus {
            Screen::Main => {self.scr_main(event).await?},
            Screen::Info => {},
        }

        Ok(())
    }

    async fn scr_main(&mut self, event: EventApp) -> color_eyre::Result<()> {
        match event {
            EventApp::Key(key) => {
                if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                    self.running = false;
                }

                self.components_event(key.code).await?;
            },
            EventApp::Tick => {},
        }
        Ok(())
    }

    async fn components_event(&mut self, key: KeyCode) -> color_eyre::Result<()> {

        match self.comp_focus {
            Focus::Entry => {},
            Focus::Btn => {},
            Focus::Info => {self.screen_focus = Screen::Info},
            Focus::Props => {},
            Focus::Table => {},
            _ => {},
        }

        Ok(())
    }

}