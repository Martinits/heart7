use std::error::Error;

pub type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Default)]
pub enum AppState {
    #[default] GetServer,
    GetName,
    GetRoom,
    Gaming,
    GameResult,
}

#[derive(Debug, Default)]
pub struct App {
    pub running: bool,
    pub counter: u8,
    pub state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
