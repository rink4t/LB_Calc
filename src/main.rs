use std::io;
use ratatui::{Terminal, backend::{CrosstermBackend}};

use crate::app::App;

// modules (/ =ω=)/
mod tui;
mod app;
mod event;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {

    let mut app = App::new();

    let res = app.run().await;

    res
}
