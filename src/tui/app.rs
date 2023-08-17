use std::error::Error;
use crate::{*, heart7_client::*};
use super::ui::UI;
use tokio_util::sync::CancellationToken;
use ratatui::backend::Backend;
use tui::tui::Tui;

pub type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Default)]
pub enum AppState {
    #[default] GetServer,
    GetRoom,
    JoinRoom,
    Gaming,
    GameResult,
}

#[derive(Debug)]
pub struct App<B: Backend> {
    tui: Tui<B>,
    cancel: CancellationToken,
    state: AppState,
}

impl<B: Backend> App<B> {
    pub fn new(tui: Tui<B>, cancel: &CancellationToken) -> Self {
        Self {
            tui,
            cancel: cancel.clone(),
            state: AppState::default()
        }
    }

    pub fn init(&mut self) -> AppResult<()> {
        self.tui.init(&self.cancel)?;
        Ok(())
    }

    pub async fn run(&mut self) -> AppResult<()> {

        // self.tui.draw() UI:draw_home();
        self.tui.draw(|frame| UI::render(frame))?;

        loop {
            tokio::select! {
                _ = self.cancel.cancelled() => {
                    break;
                }
            }
        }

        // let server_ip = "127.0.0.1";
        //
        // let mut client = Heart7Client::connect(
        //     format!("http://{}:{}", server_ip, DEFAULT_PORT)
        // ).await?;
        //
        // let request = tonic::Request::new(PlayerInfo {
        //     name: "Martinits".into(),
        // });
        //
        // let response = client.new_room(request).await?;
        //
        // println!("RESPONSE={:?}", response);

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

    pub fn exit(&mut self) -> AppResult<()> {
        // self.cancel.cancel();
        self.tui.exit()?;
        Ok(())
    }
}
