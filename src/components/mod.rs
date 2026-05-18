use async_trait::async_trait;
use ratatui::{Frame, layout::Rect};
use crossterm::event::{KeyCode, KeyEvent};

//modules (/ =ω=)/
pub mod entry_comp;
pub mod btn_comp;
pub mod table;

//components use (≧◡≦)
pub use entry_comp::EntryLineComp;
pub use btn_comp::ButtonComp;
pub use table::TableComp;

pub trait DrawableComp {
    fn draw(&self, f: &mut Frame, rect: Rect);
}

pub trait StatefullDrawableComp {
    fn draw(&self, f: &mut Frame, rect: Rect) -> color_eyre::Result<()>;
}

#[async_trait]
pub trait Component {
    fn event(&mut self, key: KeyEvent);

    fn focus(&mut self, _focus: bool);
}