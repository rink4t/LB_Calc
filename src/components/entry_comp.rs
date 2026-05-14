use ratatui::{Frame, layout::Rect, macros::ratatui_core::widgets, style::{Color, Style, Styled, Stylize}, text::{Line, Span, ToSpan}, widgets::{Block, Borders, Paragraph}};

use crate::components::DrawableComp;

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