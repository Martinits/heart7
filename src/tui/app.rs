use std::error::Error;
use crate::{*, heart7_client::*};

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

    pub async fn run(&mut self) -> AppResult<()> {

        let server_ip = "127.0.0.1";

        let mut client = Heart7Client::connect(
            format!("http://{}:{}", server_ip, DEFAULT_PORT)
        ).await?;

        let request = tonic::Request::new(PlayerInfo {
            name: "Martinits".into(),
        });

        let response = client.new_room(request).await?;

        println!("RESPONSE={:?}", response);

        // new room
        // join room -> stream
        // room status -> draw first
        // listen stream and draw: should not be able to read first initmsg
        // get a roominfo: new player join in, if all 4 join in, display ready state
        // get a whoready: someone get ready
        // get a start: server start game, and client should rpc GameStatus to get cards
        // continue listen stream
        //
        // handle when someone exits
        //
        Ok(())
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
