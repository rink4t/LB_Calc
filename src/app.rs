use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use engine::Engine;
use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect}, text::{Line, Text}, widgets::{Block, Borders, Clear, Paragraph, Wrap}};
use ratatui::style::Style;
use color_eyre::Result;
use std::{sync::mpsc};

use crate::{components::{Component, DrawableComp}, event::EventApp, tui::{Tui}};

use crate::components::EntryLineComp;
use crate::components::{ButtonComp, BtnFlag};
use crate::components::TableComp;
use crate::components::PropsComp;

//|-----------------{Focus and Screen >ᴗ<}------------------|

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Focus {
    #[default]
    Entry,
    Exe,
    Info,
    Table,
    //Props,
    CloseInf,
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
    props: PropsComp,
    inf_btn: ButtonComp,

    //components inf_screen: 
    close: ButtonComp,

    //main focus comp:
    main_fcs: Focus,

    //err_msg tmp impl:
    err_msg: String,

    //actions e and r
//    sender: mpsc::Sender<BtnFlag>,
    reciver: mpsc::Receiver<BtnFlag>,

    //engine: 
    engine: Engine,
}


impl App {
    pub fn new() -> App {

        let engine = Engine::new();

        let (sx, rx) = mpsc::channel();
        
        App { 
            running: true, comp_focus: Focus::Entry, 
            screen_focus: Screen::default(), 
            entry: EntryLineComp::new(true), 
            exe_resolv: ButtonComp::new("Resolve", false, sx.clone(), BtnFlag::Update), 
            table: TableComp::default(), 
            props: PropsComp::default(), 
            inf_btn: ButtonComp::new("Info", false, sx.clone(),BtnFlag::ChangeWindow), 
            main_fcs: Focus::Entry, 
            err_msg: String::new(), 
            close: ButtonComp::new("Close", true, sx.clone(), BtnFlag::ChangeWindow), 
            reciver: rx, 
            engine: engine 
        }
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
        self.inf_btn.draw(frame, top_layout[2]);

        let errors = Paragraph::new(Text::from(self.err_msg.clone())).block(Block::default().title("Errors").borders(Borders::ALL).style(Style::default()));
        frame.render_widget(errors, chunks[1]);

        if !self.err_msg.is_empty() {
            let errors = Paragraph::new(Text::from(self.err_msg.clone())).block(Block::default().title("Errors").borders(Borders::ALL).style(Style::default()));
            frame.render_widget(errors, chunks[1]);
        }else {
            let errors = Paragraph::new(Text::from("No errors reported <(￣︶￣)>")).block(Block::default().title("Errors").borders(Borders::ALL).style(Style::default()));
            frame.render_widget(errors, chunks[1]);
        }

        let middle_layout = Layout::default().direction(Direction::Horizontal).constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ]).split(chunks[2]);
        
        self.table.draw(frame, middle_layout[0]);
        self.props.draw(frame, middle_layout[1]);

        let block3 = match self.comp_focus {
            Focus::Entry => {Paragraph::new(Line::from(self.entry.get_keybinds())).block(Block::new().borders(Borders::ALL))},
            Focus::Exe => {Paragraph::new(Line::from(self.exe_resolv.get_keybinds())).block(Block::new().borders(Borders::ALL))},
            Focus::Info => {Paragraph::new(Line::from(self.inf_btn.get_keybinds())).block(Block::new().borders(Borders::ALL))},
            Focus::Table => {Paragraph::new(Line::from(self.table.get_keybinds())).block(Block::new().borders(Borders::ALL))},
            Focus::CloseInf => {Paragraph::new(Line::from(self.inf_btn.get_keybinds())).block(Block::new().borders(Borders::ALL))},
        };
        
        frame.render_widget(block3, chunks[3]);

        if self.screen_focus == Screen::Info {
            //let popup_block = Block::default().title(Line::from("Information").alignment(Alignment::Center)).borders(Borders::ALL).style(Style::default().bg(ratatui::style::Color::Rgb(31, 31, 1)));
            let area = centered_rect(50, 75, frame.area());

            frame.render_widget(Clear, area);

            let popup_chunks = Layout::default().direction(Direction::Vertical).constraints(vec![
                Constraint::Percentage(96),
                Constraint::Min(3),
            ]).split(area);

            let sometext = Paragraph::new(vec![
                Line::from(vec!["Author: りnkat".into()]),
                Line::from(vec!["Some app info:".into()]),
                Line::from(vec!["1: The app still under development.".into()]),
                Line::from(vec!["2: Simbols &: ∧, |: ∨, !: ¬, ->: →, <->: ↔".into()]),
            ]).wrap(Wrap {trim: true}).block(Block::new().borders(Borders::ALL));

            frame.render_widget(sometext, popup_chunks[0]);
            self.close.draw(frame, popup_chunks[1]);

        }

        Ok(())
    }

    async fn event(&mut self, tui: &mut Tui) -> color_eyre::Result<()> {

        let Some(event) = tui.event_hndl.next().await else {
            return Ok(());
        };

        //match self.screen_focus {
            //Screen::Main => {self.scr_main(event).await?},
            //Screen::Info => {},
        //}

        self.scr_main(event).await?;

        Ok(())
    }

    async fn action(&mut self) -> color_eyre::Result<()> {
        if let Ok(action) = self.reciver.try_recv() {
            match action {
                BtnFlag::Update => {
                    //engine executes here and then (・v・) Odin (・v・) knows!
                    if let Some(expr) = self.entry.get_entry(){
                        let result = self.engine.solve_expr(expr);
                        
                        if result.err_msg.is_empty() {
                            self.table.update(result.ids, result.colums);
                            self.props.update(result.properties); 
                            self.err_msg.clear();
                        }else {
                            self.props.no_props();
                            self.table.no_table();
                            self.err_msg = result.err_msg;
                        } 
                    }
                },
                BtnFlag::ChangeWindow => {
                    match self.screen_focus {
                        Screen::Main => {
                            self.main_fcs = self.comp_focus;
                            self.screen_focus = Screen::Info;
                            self.comp_focus = Focus::CloseInf;
                            self.close.focus(true);
                        },
                        Screen::Info => {
                            self.comp_focus = self.main_fcs;
                            self.close.focus(false);
                            self.screen_focus = Screen::Main;

                        },
                    }
                }
            }   
        }
        
        Ok(())
    }

    async fn scr_main(&mut self, event: EventApp) -> color_eyre::Result<()> {
        match event {
            EventApp::Key(key) => {
                if self.screen_focus == Screen::Main {
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
                                self.comp_focus = Focus::Info;
                                self.inf_btn.focus(true);
                            },
                            Focus::Info => {
                                self.inf_btn.focus(false);
                                self.comp_focus = Focus::Table;
                                self.table.focus(true);
                            },
                            Focus::Table => {
                                self.table.focus(false);
                                self.comp_focus = Focus::Entry;
                                self.entry.focus(true);
                            }
                            _ => {},
                        }
                    }

                    self.main_components_event(key).await?; 
                }else if self.screen_focus == Screen::Info {
                    self.info_main_components_event(key).await?;
                }
            },
            //EventApp::Render => {}
            EventApp::Tick => {},
        }
        Ok(())
    }

    async fn main_components_event(&mut self, key: KeyEvent) -> color_eyre::Result<()> {
        match self.comp_focus {
            Focus::Entry => {self.entry.event(key)?;},
            Focus::Exe => {self.exe_resolv.event(key)?;},
            Focus::Info => {self.inf_btn.event(key)?;},
            Focus::Table => {self.table.event(key)?;},
            //Focus::Props => {},
            _ => {},
        }
        Ok(())
    }

    async fn info_main_components_event(&mut self, key: KeyEvent) -> color_eyre::Result<()> {
        match self.comp_focus {
            Focus::CloseInf => {self.close.event(key)?;}
            _ => {},
        }
        Ok(())
    }

}

fn centered_rect(percent_x: u16, percent_y: u16, rect: Rect) -> Rect{
    let popup_layout = Layout::default().direction(Direction::Vertical).constraints([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ]).split(rect);

    Layout::default().direction(Direction::Horizontal).constraints([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ]).split(popup_layout[1])[1]

}
