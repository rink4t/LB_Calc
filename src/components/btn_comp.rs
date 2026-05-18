use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{style::{Color, Style, Stylize}, text::Span, widgets::{Block, Paragraph}};

use crate::components::{Component, DrawableComp};

//|-----------------{Res Tables ( . .)φ}------------------|
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
}

impl ButtonComp {
    pub fn new(text: &str, init_focus: bool) -> ButtonComp {
        ButtonComp { text: text.to_string(), state: State::Release, focus: false }
    }

    pub async fn execute(&self) -> color_eyre::Result<()>{
        Ok(())
    }

}

impl DrawableComp for ButtonComp {
    fn draw(&self, f: &mut ratatui::prelude::Frame, rect: ratatui::prelude::Rect) {
        let text = match self.state {
            State::Pressed => {Span::from(self.text.clone()).bg(Color::Yellow)},
            State::Release => {Span::from(self.text.clone())}
        };

        let btn = Paragraph::new(text).block(Block::bordered().border_style(Style::default())).centered();

        f.render_widget(btn, rect);

    }
}

impl Component for ButtonComp {
    fn event(&mut self,key: crossterm::event::KeyEvent) {
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
                KeyCode::Enter => {self.state = State::Release;}
                _ => {}
            }
        }
    }

    fn focus(&mut self, focus: bool) {
        self.focus = focus;
    }
}