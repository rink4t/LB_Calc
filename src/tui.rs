use std::{io::{self, Stdout, stdout}, time::Duration};

use color_eyre::eyre::Ok;
use crossterm::{ExecutableCommand, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use ratatui::backend::{CrosstermBackend};

use crate::event::{self, EventHndl};

pub struct Tui {
    pub terminal: ratatui::Terminal<CrosstermBackend<Stdout>>,
    pub event_hndl: event::EventHndl,
}

impl Tui {

    pub fn new(tick_rate: u64) -> color_eyre::Result<Self>{
        Ok(Self { 
            terminal: ratatui::Terminal::new(CrosstermBackend::new(stdout()))?,
            event_hndl: EventHndl::new(Duration::from_millis(tick_rate)),
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