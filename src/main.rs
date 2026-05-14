use crate::app::App;

// modules (/ =ω=)/
mod tui;
mod app;
mod event;
mod components;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {

    let mut app = App::new();

    let res = app.run().await;

    res
}
