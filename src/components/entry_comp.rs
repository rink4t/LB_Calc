use crossterm::{cursor, event::{KeyCode, KeyEventKind}};
use ratatui::{Frame, layout::Rect, style::{Color, Stylize}, text::{self, Line, Span, ToSpan}, widgets::{Block, Paragraph}};

use crate::components::{Component, DrawableComp};

//|-----------------{Res Tables >ᴗ<}------------------|

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
    offset: u16,
    cursor: u16,
    width: u16,
}

impl EntryLineComp {
    pub fn new(init_focus: bool) -> Self {
        Self { text: String::from(""), mode: Editing::No, focus: init_focus, offset: 8, cursor: 0, width: 0}
    }

    pub fn is_editing(&self) -> bool {
        if self.mode == Editing::Yes { true } else { false }
    }

    fn add_ch(&mut self, ch: char) {

        if self.text.chars().count() > u16::MAX.into(){
            return;
        }

        let current_index = self.cursor as usize;

        self.text.insert(current_index, ch);

        self.cursor = self.cursor.saturating_add(1);
        
    }

    fn rmv_ch(&mut self) {
        let current_index = self.cursor as usize;

        if !self.text.is_empty() && current_index >= 1 {
            self.text.remove(current_index.saturating_sub(1));
        }
        
        self.move_cursor_left();
    }

    fn move_cursor_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    fn move_cursor_right(&mut self) {
        if (self.cursor as usize) < self.text.chars().count() {
            self.cursor = self.cursor.saturating_add(1);
        }
    }

    fn update_offset(&mut self) {
        if self.cursor < self.offset {
            self.offset = self.cursor;
        }

        if self.cursor >= self.offset + self.visible_with() as u16 {
            self.offset = (self.cursor.saturating_sub(self.visible_with() as u16)).saturating_add(1);
        }
        
        if self.cursor <= self.offset + self.visible_with() as u16{
            self.offset = (self.cursor.saturating_sub(self.visible_with() as u16)).saturating_add(1);
        }
    }

    fn cursor_x(&self) -> u16 {
        let visible_offset = (self.cursor.saturating_sub(self.offset)).saturating_add(8);
        visible_offset as u16
    }

    fn visible_with(&self) -> usize {
        (self.width.saturating_sub(9)) as usize
    }

    pub fn get_entry(&self) -> Option<String>{
        if !self.text.is_empty() {
            return Some(self.text.clone());
        }else {
            return None;
        }
    }

    pub fn get_keybinds(&self) -> String{
        "Enter: Edit/Stop Editing | Backspace: remove char | You can't use Tab while editing (o･ω･o)|".to_string()
    }

}

impl DrawableComp for EntryLineComp {
    fn draw(&mut self, f: &mut Frame, rect: Rect){

        if self.width != rect.width {
            self.width = rect.width;
        }

        let value: Span;
        let label = Span::raw("Entry: ");

        let visible_txt_width = self.visible_with();

        self.update_offset();

        if self.focus {
            let util = self.text.chars().skip(self.offset as usize).take(visible_txt_width).collect::<String>();
            value = Span::from(util).bg(Color::Rgb(36, 26, 26)); 
        }else {
            value = self.text.to_span();
        }

        let text = Paragraph::new(Line::from(vec![label, value])).block(Block::bordered());

        f.render_widget(ratatui::widgets::Clear, rect);
        f.render_widget(text, rect);

        if self.focus && self.is_editing() {
            
            let cursor_x = self.cursor_x();
            let cursor_y = rect.y + 1;

            f.set_cursor_position((cursor_x, cursor_y));
        }

    }
}

impl Component for EntryLineComp {
    fn event(&mut self, key: crossterm::event::KeyEvent) -> color_eyre::Result<()> {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char(ch) if self.mode == Editing::Yes => {self.add_ch(ch);}
                KeyCode::Backspace => if self.mode == Editing::Yes {self.rmv_ch();}
                KeyCode::Left => if self.mode == Editing::Yes {self.move_cursor_left();}
                KeyCode::Right => if self.mode == Editing::Yes {self.move_cursor_right();}
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