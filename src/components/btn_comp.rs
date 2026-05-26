use std::sync::mpsc;

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{style::{Color, Style, Stylize}, text::Span, widgets::{Block, Paragraph}};

use crate::components::{Component, DrawableComp};

use engine::ExprRes;

//|-----------------{Btn flags >ᴗ<}------------------|
pub enum BtnFlag {
    Update,
    Clean, 
    ChangeWindow,
}

//|-----------------{State ( . .)φ}------------------|
#[derive(PartialEq, Eq)]
enum State {
    Pressed,
    Release,
}

//|-----------------{Button ( . .)φ}------------------|

pub struct ButtonComp {
    text: String,
    state: State,
    focus: bool,
    sender: mpsc::Sender<BtnFlag>,
    flag: BtnFlag,
}

impl ButtonComp {
    pub fn new(text: &str, init_focus: bool, snd: mpsc::Sender<BtnFlag>, flag: BtnFlag) -> ButtonComp {
        ButtonComp { text: text.to_string(), state: State::Release, focus: false, sender: snd, flag: flag}
    }

    pub fn execute(&self) -> color_eyre::Result<()>{

        match self.flag {
            BtnFlag::Update => {self.sender.send(BtnFlag::Update)?},
            BtnFlag::ChangeWindow => {self.sender.send(BtnFlag::ChangeWindow)?},
            BtnFlag::Clean => {self.sender.send(BtnFlag::Clean)?},
        }

        Ok(())
    }

    pub fn get_keybinds(&self) -> String{
        "Enter: Execute".to_string()
    }

}

impl DrawableComp for ButtonComp {
    fn draw(&self, f: &mut ratatui::prelude::Frame, rect: ratatui::prelude::Rect) {
        let text =  if self.state == State::Pressed {
            Span::from(self.text.clone()).bg(Color::Yellow)
        } else {
            if self.focus { 
                Span::from(self.text.clone().bg(Color::Rgb(36, 26, 26))) 
            }else{ Span::from(self.text.clone()) }
        };

        let btn = Paragraph::new(text).block(Block::bordered().border_style(Style::default())).centered();

        f.render_widget(btn, rect);

    }
}

impl Component for ButtonComp {
    fn event(&mut self, key: crossterm::event::KeyEvent) -> color_eyre::Result<()>{
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Enter => {
                    self.state = State::Pressed;
                }
                _ => {}
            }
        }

        if key.kind == KeyEventKind::Release {
            match key.code {
                KeyCode::Enter => {
                    self.state = State::Release;
                    self.execute()?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn focus(&mut self, focus: bool) {
        self.focus = focus;
    }
}