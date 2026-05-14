use crossterm::event::KeyCode;
use ratatui::{Frame, layout::Rect, macros::ratatui_core::widgets, style::{Color, Style, Styled, Stylize}, text::{Line, Span, ToSpan}, widgets::{Block, Borders, Paragraph}};

use crate::components::{Component, DrawableComp};

#[derive(PartialEq, Eq)]
pub enum Editing {
    Yes,
    No,
}

pub struct EntryLine {
    text: String,
    mode: Editing,
}

impl EntryLine {
    pub fn new() -> Self {
        Self { text: String::from("a&b"), mode: Editing::No }
    }

    pub fn is_editing(&self) -> bool {
        if self.mode == Editing::Yes { true } else { false }
    }

    //Prototype this functions need a little amount of code to be good enought "(--_--)"
    //at this moment only can handle entrys like 'a', 'b', '&', 'c' = ab&c
    //ab&c can be two thinks "ab" as a var or the user forgot the operant in that case is necessary delete the entry
    //from c to b and then reinsert everything like: 'a', '&', 'b', '&', 'c' = ab&c
    //Thats no good enought if you forgot a operant in that example only needs delete 3 elements 
    //in other cases possible a big amount of the entry for only one errorヽ(°〇°)ﾉ. 

    pub fn editing(&mut self, ch: char, insert: bool, remove: bool) {
        if insert {
            self.add_ch(ch);
        }

        if remove {
            self.rmv_ch();
        }
    }

    fn add_ch(&mut self, ch: char) {
        self.text.push(ch);
    }

    fn rmv_ch(&mut self) {
        self.text.pop();
    }

}

impl DrawableComp for EntryLine {
    fn draw(&self, f: &mut Frame, rect: Rect, focused: bool) -> color_eyre::Result<()> {
        let value: Span;
        let label = Span::raw("Entry: ");

        if focused {
            value = self.text.to_span().bg(Color::Rgb(36, 26, 26));
        }else {
            value = self.text.to_span();
        }

        let text = Paragraph::new(Line::from(vec![label, value])).block(Block::bordered());

        f.render_widget(ratatui::widgets::Clear, rect);
        f.render_widget(text, rect);

        Ok(())
    }
}

impl Component for EntryLine {
    fn event(&mut self, key: crossterm::event::KeyCode) {
        match key {
            KeyCode::Char(ch) if self.mode == Editing::Yes => {self.editing(ch, true, false);}
            KeyCode::Backspace => if self.mode == Editing::Yes {self.editing(' ', false, true);}
            KeyCode::Enter => {
                match self.mode {
                    Editing::Yes => {self.mode = Editing::No;},
                    Editing::No => {self.mode = Editing::Yes;},
                }
            }
            _ => {}
        }
    }

    fn focus(&mut self,_focus: bool) {
        
    }
}