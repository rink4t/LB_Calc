use std::io::{self, Stdout, stdout};

use color_eyre::eyre::Ok;
use crossterm::{ExecutableCommand, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use ratatui::backend::{Backend, CrosstermBackend};


pub struct Tui {
    terminal: ratatui::Terminal<CrosstermBackend<Stdout>>,
    //event_hndl: 
}

impl Tui {

    pub fn new(tick_rate: u64) -> color_eyre::Result<Self>{
        Ok(Self { 
            terminal: ratatui::Terminal::new(CrosstermBackend::new(stdout()))?,
        })
    }

    pub fn init(&mut self) -> color_eyre::Result<()> {
        enable_raw_mode()?;
        io::stdout().execute(EnterAlternateScreen)?;
        Ok(())
    }

    pub fn shutdown(&mut self) {
        let leave_scr = io::stdout().execute(LeaveAlternateScreen).map(|_f| ());

        if let Err(err) = leave_scr {
            eprintln!("leave alternate screen failed (・・ヾ: \n{}", err);
        }

        let leave_raw_mode = disable_raw_mode();

        if let Err(err) = leave_raw_mode {
            eprintln!("leave raw mode failed (・・ヾ: \n{}", err);
        }

        if let Err(err) = self.terminal.show_cursor() {
            eprintln!("show the cursor failed: (・・ヾ: \n{}", err);
        }
    }
}