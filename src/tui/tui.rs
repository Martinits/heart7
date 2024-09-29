use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use std::io;
use std::panic;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::Frame;
use tokio_util::sync::CancellationToken;
use anyhow::Result;

type TuiBackend = CrosstermBackend<std::io::Stdout>;

pub struct Tui {
    terminal: Terminal<TuiBackend>,
}

impl Tui {
    pub fn new(terminal: Terminal<TuiBackend>) -> Self {
        Self { terminal }
    }

    pub fn init(&mut self, cancel: &CancellationToken) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        let cancel_clone = cancel.clone();
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            cancel_clone.cancel();
            Self::reset().expect("Failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut Frame<TuiBackend>),
    {
        self.terminal.draw(f)?;
        // self.terminal.draw(|frame| UI::render(frame))?;
        Ok(())
    }

    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
