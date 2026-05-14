use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{Frame, layout::{Constraint, Direction, Layout}, widgets::{self, Block, Borders, Paragraph}};
use ratatui::style::Style;
use color_eyre::Result;

use crate::{components::{Component, DrawableComp}, event::EventApp, tui::Tui};

use crate::components::EntryLine;

//|-----------------{Focus and Screen >ᴗ<}------------------|

#[derive(Default, Debug, PartialEq, Eq)]
#[allow(dead_code)]
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

//|-----------------{App ( . .)φ}------------------|
pub struct App {
    running: bool,
    comp_focus: Focus,
    screen_focus: Screen,

    //components:
    entry: EntryLine,
}


impl App {
    pub fn new() -> App {
        App { running: true, comp_focus: Focus::Entry, screen_focus: Screen::default(), entry: EntryLine::new() }
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

        let chunks = Layout::default().direction(Direction::Vertical).constraints([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(3),
        ]).split(frame.area());


        if self.comp_focus == Focus::Entry {
            self.entry.draw(frame, chunks[0],true)?;
        }else {
            self.entry.draw(frame, chunks[0],false)?;
        }
        
        
        let block2 = Block::default().borders(Borders::ALL).style(Style::default());
        frame.render_widget(block2, chunks[1]);
        let block3 = Block::default().borders(Borders::ALL).style(Style::default());
        frame.render_widget(block3, chunks[2]);

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

                if key.code == KeyCode::Tab {
                    match self.comp_focus {
                        Focus::Entry if !self.entry.is_editing() => {self.comp_focus = Focus::Btn},
                        _ => {},
                    }
                }

                self.components_event(key.code).await?;
            },
            EventApp::Tick => {},
        }
        Ok(())
    }

    async fn components_event(&mut self, key: KeyCode) -> color_eyre::Result<()> {

        match self.comp_focus {
            Focus::Entry => {self.entry.event(key);},
            Focus::Btn => {},
            Focus::Info => {self.screen_focus = Screen::Info},
            Focus::Props => {},
            Focus::Table => {},
        }

        Ok(())
    }

}