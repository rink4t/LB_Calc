use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{Frame, layout::Rect, style::{Color, Stylize}, text::{Line, Span, ToSpan}, widgets::{Block, Paragraph}};

use crate::components::{Component, DrawableComp};

//|-----------------{Res Tables ( . .)φ}------------------|

#[derive(PartialEq, Eq)]
pub enum Editing {
    Yes,
    No,
}

//|-----------------{EntryLineComp( . .)φ}------------------|

pub struct EntryLineComp {
    text: String,
    mode: Editing,
    focus: bool,
}

impl EntryLineComp {
    pub fn new(init_focus: bool) -> Self {
        Self { text: String::from("a&b"), mode: Editing::No, focus: init_focus }
    }

    pub fn is_editing(&self) -> bool {
        if self.mode == Editing::Yes { true } else { false }
    }

    //Prototype this functions need a little amount of code to be good enought "(--_--)"
    //at this moment only can handle entrys like 'a', 'b', '&', 'c' = ab&c
    //ab&c can be two things "ab" as a var or the user forgot the operant in that case is necessary delete the entry
    //from c to b and then reinsert everything like: 'a', '&', 'b', '&', 'c' = ab&c
    //Thats no good enought, if you forget a operant in that example only needs delete 3 elements 
    //in other cases possible you need delete a big amount of the entry for only one errorヽ(°〇°)ﾉ. 

    fn add_ch(&mut self, ch: char) {
        self.text.push(ch);
    }

    fn rmv_ch(&mut self) {
        self.text.pop();
    }

    pub fn clear(&mut self) {
        self.text.clear();
    }

    pub fn get_entry(&self) -> Option<String>{
        if !self.text.is_empty() {
            return Some(self.text.clone());
        }else {
            return None;
        }
    }

}

impl DrawableComp for EntryLineComp {
    fn draw(&self, f: &mut Frame, rect: Rect){
        let value: Span;
        let label = Span::raw("Entry: ");

        if self.focus {
            value = self.text.to_span().bg(Color::Rgb(36, 26, 26));
        }else {
            value = self.text.to_span();
        }

        let text = Paragraph::new(Line::from(vec![label, value])).block(Block::bordered());

        f.render_widget(ratatui::widgets::Clear, rect);
        f.render_widget(text, rect);

    }
}

impl Component for EntryLineComp {
    fn event(&mut self, key: crossterm::event::KeyEvent) -> color_eyre::Result<()> {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char(ch) if self.mode == Editing::Yes => {self.add_ch(ch);}
                KeyCode::Backspace => if self.mode == Editing::Yes {self.rmv_ch();}
                KeyCode::Enter => {
                    match self.mode {
                        Editing::Yes => {self.mode = Editing::No;},
                        Editing::No => {self.mode = Editing::Yes;},
                    }
                }, 
                _ => {}
            }
        }
        Ok(())
    }

    fn focus(&mut self, focus: bool) {
        self.focus = focus;
    }
}