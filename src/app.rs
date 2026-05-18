use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{Frame, layout::{Constraint, Direction, Layout}, widgets::{self, Block, Borders, Paragraph}};
use ratatui::style::Style;
use color_eyre::Result;

use crate::{components::{Component, DrawableComp}, event::EventApp, tui::Tui};

use crate::components::EntryLineComp;
use crate::components::ButtonComp;

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
    entry: EntryLineComp,
    btn_resolv: ButtonComp,
}


impl App {
    pub fn new() -> App {
        App { running: true, comp_focus: Focus::Entry, screen_focus: Screen::default(), entry: EntryLineComp::new(true), btn_resolv: ButtonComp::new("Resolve", false) }
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

        let top_layout = Layout::default().direction(Direction::Horizontal).constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ]).split(chunks[0]);

        self.entry.draw(frame, top_layout[0]);
        self.btn_resolv.draw(frame, top_layout[1]);

        
        let infb = Block::default().borders(Borders::ALL).style(Style::default());

        frame.render_widget(infb, top_layout[2]);

        let middle_layout = Layout::default().direction(Direction::Horizontal).constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ]).split(chunks[1]);
        
        let cnt_block1 = Block::default().borders(Borders::ALL).style(Style::default());
        let cnt_block2 = Block::default().borders(Borders::ALL).style(Style::default());

        frame.render_widget(cnt_block1, middle_layout[0]);
        frame.render_widget(cnt_block2, middle_layout[1]);
        
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
                if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL && key.kind == KeyEventKind::Press {
                    self.running = false;
                }

                if key.code == KeyCode::Tab && key.kind == KeyEventKind::Press{
                    match self.comp_focus {
                        Focus::Entry if !self.entry.is_editing() => {
                            self.entry.focus(false); 
                            self.comp_focus = Focus::Btn;
                            self.btn_resolv.focus(true);
                        },
                        Focus::Btn => {
                            self.btn_resolv.focus(false);
                            self.comp_focus = Focus::Info
                        },
                        Focus::Info => {self.comp_focus = Focus::Table},
                        Focus::Table => {self.comp_focus = Focus::Props}
                        Focus::Props => {self.comp_focus = Focus::Entry; self.entry.focus(true);},
                        _ => {},
                    }
                }

                self.components_event(key).await?;
            },
            EventApp::Render => {}
            EventApp::Tick => {},
        }
        Ok(())
    }

    async fn components_event(&mut self, key: KeyEvent) -> color_eyre::Result<()> {

        match self.comp_focus {
            Focus::Entry => {self.entry.event(key);},
            Focus::Btn => {self.btn_resolv.event(key);},
            Focus::Info => {},
            Focus::Props => {},
            Focus::Table => {},
        }

        Ok(())
    }

}