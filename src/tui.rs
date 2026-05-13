use std::io;

use color_eyre::eyre::Ok;
use crossterm::{ExecutableCommand, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use ratatui::{Terminal, backend::{self, CrosstermBackend}};


pub fn init() -> color_eyre::Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

pub fn shutdown() {
    let leave_scr = io::stdout().execute(LeaveAlternateScreen).map(|_f| ());

    if let Err(err) = leave_scr {
        eprintln!("leave alternate screen failed (・・ヾ: \n{}", err);
    }

    let leave_raw_mode = disable_raw_mode();

    if let Err(err) = leave_raw_mode {
        eprintln!("leave raw mode failed (・・ヾ: \n{}", err);
    }
}