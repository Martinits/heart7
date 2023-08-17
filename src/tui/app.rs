use std::error::Error;
use crate::{*, heart7_client::*};
use super::ui::UI;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use ratatui::backend::Backend;
use tui::tui::Tui;

pub type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Default)]
pub enum AppState {
    #[default] GetServer,
    GetRoom,
    JoinRoom,
    WaitPlayer,
    WaitReady,
    Gaming,
    GameResult,
}

#[derive(Debug)]
pub enum Action {
    Enter,
    LeftArrow,
    RightArrow,
    Esc,
    CtrlC,
    Type(char),
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
            state: AppState::GetServer,
        }
    }

    pub fn init(&mut self) -> AppResult<()> {
        self.tui.init(&self.cancel)?;
        Ok(())
    }

    pub async fn run(&mut self, mut rx: mpsc::Receiver<Action>) -> AppResult<()> {

        let mut draw_or_not = true;
        loop {
            if draw_or_not {
                self.draw()?;
            }
            tokio::select! {
                _ = self.cancel.cancelled() => {
                    break;
                }
                action = rx.recv() => {
                    draw_or_not = match action {
                        None => panic!("Channel to app closed!"),
                        Some(Action::Esc) => self.handle_esc(),
                        Some(Action::Enter) => self.handle_enter(),
                        Some(Action::LeftArrow) => self.handle_arrow(true),
                        Some(Action::RightArrow) => self.handle_arrow(false),
                        Some(Action::Type(c)) => self.handle_type(c),
                        Some(Action::CtrlC) => panic!("Got Ctrl-C!"),
                    }
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

    fn draw(&mut self) -> AppResult<()> {
        self.tui.draw(|frame| UI::render(frame, &self.state))?;
        Ok(())
    }

    pub fn exit(&mut self) -> AppResult<()> {
        // self.cancel.cancel();
        self.tui.exit()?;
        Ok(())
    }

    fn handle_enter(&self) -> bool {
        true
    }

    fn handle_type(&self, c: char) -> bool {
        true
    }

    fn handle_arrow(&self, is_left: bool) -> bool {
        true
    }

    fn handle_esc(&self) -> bool {
        true
    }
}
