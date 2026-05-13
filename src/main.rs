use std::io;
use ratatui::{Terminal, backend::{CrosstermBackend}};

// modules (/ =ω=)/
mod tui;
mod app;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {

    tui::init();

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    //let mut app = App::new(terminal);
    //let res = app.run().await;

    tui::shutdown();

    Ok(())
}
