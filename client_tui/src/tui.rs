use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use std::io;
use std::panic;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use anyhow::Result;
use crate::*;

type TuiBackend = CrosstermBackend<std::io::Stdout>;

const BLOCK_THRESHOLD_WIDTH: u16 = 160;
const BLOCK_THRESHOLD_HEIGHT: u16 = 48;

pub struct Tui {
    terminal: Terminal<TuiBackend>,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend)?;
        let mut cui = Self { terminal };
        cui.init()?;
        Ok(cui)
    }

    fn init(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("Failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    fn get_size(&self) -> Result<(u16, u16)> {
        let sz = self.terminal.size()?;
        Ok((sz.width, sz.height))
    }

    pub fn should_block(&self) -> Result<bool> {
        let (w, h) = self.get_size()?;
        Ok(w < BLOCK_THRESHOLD_WIDTH || h < BLOCK_THRESHOLD_HEIGHT)
    }

    pub fn draw(&mut self, cs: ClientState) -> Result<()> {
        self.terminal.draw(|frame| ui::render(frame, cs))?;
        Ok(())
    }

    pub fn draw_blocked(&mut self) -> Result<()> {
        let sz = self.get_size()?;
        self.terminal.draw(|frame| ui::blocked(frame, sz))?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
