use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use engine::Engine;
use ratatui::{Frame, layout::{Constraint, Direction, Layout}, widgets::{Block, Borders}};
use ratatui::style::Style;
use color_eyre::Result;
use std::{sync::mpsc};

use crate::{components::{Component, DrawableComp, StatefullDrawableComp, TableComp, btn_comp::BtnFlag}, event::EventApp, tui::{Tui}};

use crate::components::EntryLineComp;
use crate::components::ButtonComp;

//|-----------------{Focus and Screen >ᴗ<}------------------|

#[derive(Default, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Focus {
    #[default]
    Entry,
    Exe,
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
    exe_resolv: ButtonComp,
    table: TableComp,

    //actions e and r
    sender: mpsc::Sender<BtnFlag>,
    reciver: mpsc::Receiver<BtnFlag>,

    //engine: 
    engine: Engine,
}


impl App {
    pub fn new() -> App {

        let engine = Engine::new();

        let (sx, rx) = mpsc::channel();
        let btn_sender = sx.clone();
        
        App { running: true, comp_focus: Focus::Entry, screen_focus: Screen::default(), entry: EntryLineComp::new(true), exe_resolv: ButtonComp::new("Resolve", false, btn_sender, BtnFlag::Update), table: TableComp::default(), sender: sx, reciver: rx, engine: engine }
    }

    pub async fn run(&mut self) -> color_eyre::Result<()> {
        let mut tui = Tui::new(200)?;
        
        tui.init()?;

        while self.running {
            tui.terminal.draw(|f| {
                let _ = self.draw(f);
            })?;
            
            self.event(&mut tui).await?;
            self.action().await?;
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
        self.exe_resolv.draw(frame, top_layout[1]);
        let infb = Block::default().borders(Borders::ALL).style(Style::default());
        frame.render_widget(infb, top_layout[2]);

        let middle_layout = Layout::default().direction(Direction::Horizontal).constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ]).split(chunks[1]);
        
        self.table.draw(frame, middle_layout[0]);
        let cnt_block2 = Block::default().borders(Borders::ALL).style(Style::default());
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

    async fn action(&mut self) -> color_eyre::Result<()> {
        if let Ok(action) = self.reciver.try_recv() {
            match action {
                BtnFlag::Update => {
                    //engine executes here and then (・v・) Odin (・v・) knows!
                    if let Some(expr) = self.entry.get_entry(){
                        let result = self.engine.solve_expr(expr);
                        self.table.update(result.ids, result.colums);
                    }
                    //self.engine.solve_expr(self.entry.get_entry())

                },
                _ => {},
            }   
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
                            self.comp_focus = Focus::Exe;
                            self.exe_resolv.focus(true);
                        },
                        Focus::Exe => {
                            self.exe_resolv.focus(false);
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
            Focus::Entry => {self.entry.event(key)?;},
            Focus::Exe => {self.exe_resolv.event(key)?;},
            Focus::Info => {},
            Focus::Props => {},
            Focus::Table => {},
        }

        Ok(())
    }

}