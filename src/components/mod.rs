use async_trait::async_trait;
use ratatui::{Frame, layout::Rect};
use crossterm::event::{KeyEvent};

//modules (/ =ω=)/
pub mod entry_comp;
pub mod btn_comp;
pub mod table_comp;
pub mod props_comp;

//components use (≧◡≦)
pub use entry_comp::EntryLineComp;
pub use btn_comp::{ButtonComp, BtnFlag};
pub use table_comp::TableComp;
pub use props_comp::PropsComp;

pub trait DrawableComp {
    fn draw(&mut self, f: &mut Frame, rect: Rect);
}

pub trait _StatefullDrawableComp {
    fn draw(&mut self, f: &mut Frame, rect: Rect);
}

#[async_trait]
pub trait Component {
    fn event(&mut self, key: KeyEvent) -> color_eyre::Result<()>;

    fn focus(&mut self, _focus: bool);
}