use async_trait::async_trait;
use ratatui::{Frame, layout::Rect};
use crossterm::event::KeyCode;

//modules (/ =ω=)/
pub mod entry_comp;

//components use (≧◡≦)
pub use entry_comp::EntryLine;

pub trait DrawableComp {
    fn draw(&self, f: &mut Frame, rect: Rect, focused: bool) -> color_eyre::Result<()>;
}

pub trait StatefullDrawableComp {
    fn draw(&self, f: &mut Frame, rect: Rect, focused: bool) -> color_eyre::Result<()>;
}

#[async_trait]
pub trait Component {
    fn event(&mut self, key: KeyCode);

    fn focus(&mut self, _focus: bool);
}