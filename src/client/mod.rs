mod rpc;
pub mod desk;
mod rpc_handler;
mod key_handler;
mod exit_handler;

use std::error::Error;
use crate::*;
use crate::client::rpc::Client;
use crate::tui::ui;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use ratatui::layout::Rect;
use tui::tui::Tui;
use tui_input::Input;
use crate::game::Card;
use crate::client::desk::*;
use std::panic;
use exit_handler::ExitMenuEvent;

fn add_cancel_to_panic(cancel: CancellationToken) {
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        cancel.cancel();
        panic_hook(panic);
    }));
}

pub type AppResult<T> = Result<T, Box<dyn Error>>;

pub enum AppState {
    GetServer {
        input: Input,
        msg: String,
        connecting: bool,
    },
    AskName {
        client: Client,
        input: Input,
        msg: String,
        button: u16,
        is_input: bool,
    },
    NewRoom {
        client: Client,
        input: Input,
        msg: String,
        name: String,
    },
    JoinRoom {
        client: Client,
        input: Input,
        msg: String,
        name: String,
        stream_listener_cancel: CancellationToken,
    },
    WaitPlayer {
        client: Client,
        players: Vec<(String, usize, bool)>,
        msg: Vec<String>,
        roomid: String,
        stream_listener_cancel: CancellationToken,
    },
    WaitReady {
        client: Client,
        players: Vec<(String, usize, bool)>,
        msg: Vec<String>,
        roomid: String,
        stream_listener_cancel: CancellationToken,
    },
    Gaming {
        client: Client,
        players: Vec<(String, usize, u32)>, //(name, idx, hold)
        next: usize,
        choose: usize, // 0 for none
        last: Option<Card>, // None for hold
        cards: Vec<Card>,
        holds: Vec<Card>,
        has_last: bool,
        desk: Desk,
        roomid: String,
        button: u32,
        play_cnt: u32,
        msg: Option<String>,
        stream_listener_cancel: CancellationToken,
    },
    GameResult {
        ds: Vec<Vec<(Card, usize)>>,
        client: Client,
        players: Vec<(String, usize, Vec<Card>)>,
        roomid: String,
        stream_listener_cancel: CancellationToken,
    },
}

pub enum AppEvent {
    Enter,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    Esc,
    CtrlC,
    Type(char),
    Resize(u16, u16),
    Refresh,
    Backspace,
    Delete,
    ServerConnectResult(Result<Client, String>),
    StreamMsg(GameMsg),
}

pub struct App {
    tui: Tui,
    cancel: CancellationToken,
    state: AppState,
    tx: mpsc::Sender<AppEvent>,
    rx: mpsc::Receiver<AppEvent>,
    block_event: bool,
    sz: (u16, u16),
    exitmenu: (bool, u32),
    default_addr: String,
}

impl App {
    pub async fn new(
        tui: Tui,
        cancel: &CancellationToken,
        tx: mpsc::Sender<AppEvent>,
        rx: mpsc::Receiver<AppEvent>,
        sz: Rect,
        default_addr: String,
    ) -> Self {
        Self {
            tui,
            block_event: false,
            exitmenu: (false, 0),
            sz: (sz.width, sz.height),
            cancel: cancel.clone(),
            state: AppState::GetServer {
                input: Input::new(default_addr.clone()).with_cursor(0),
                msg: "Welcome to Seven-of-Heart !!!\n\
                        Please enter game server address:".into(),
                connecting: false,
            },
            tx,
            rx,
            default_addr,
        }
    }

    pub fn init(&mut self) -> AppResult<()> {
        self.tui.init(&self.cancel)?;
        Ok(())
    }

    async fn appevent_dispatcher(&mut self, a: AppEvent) -> bool {
        if self.exitmenu.0 {
            match a {
                AppEvent::Esc if !self.block_event
                    => self.handle_esc(),
                AppEvent::Enter if !self.block_event
                    => self.handle_exitmenu_event(ExitMenuEvent::Enter).await,
                AppEvent::UpArrow if !self.block_event
                    => self.handle_exitmenu_event(ExitMenuEvent::MoveUp).await,
                AppEvent::DownArrow if !self.block_event
                    => self.handle_exitmenu_event(ExitMenuEvent::MoveDown).await,
                AppEvent::CtrlC
                    => panic!("Got Ctrl-C!"),
                AppEvent::Resize(x, y) => {
                    self.sz = (x, y);
                    true
                },
                AppEvent::Refresh => true,
                AppEvent::ServerConnectResult(r)
                    => self.handle_server_connect_result(r),
                AppEvent::StreamMsg(msg)
                    => self.handle_stream_msg(msg).await,
                _ => false,
            }
        } else {
            match a {
                AppEvent::Esc if !self.block_event
                    => self.handle_esc(),
                AppEvent::Enter if !self.block_event
                    => self.handle_enter().await,
                AppEvent::LeftArrow if !self.block_event
                    => self.handle_lr_arrow(true),
                AppEvent::RightArrow if !self.block_event
                    => self.handle_lr_arrow(false),
                AppEvent::UpArrow if !self.block_event
                    => self.handle_ud_arrow(true),
                AppEvent::DownArrow if !self.block_event
                    => self.handle_ud_arrow(false),
                AppEvent::Type(c) if !self.block_event
                    => self.handle_typing(c),
                AppEvent::CtrlC
                    => panic!("Got Ctrl-C!"),
                AppEvent::Resize(x, y) => {
                    self.sz = (x, y);
                    true
                },
                AppEvent::Refresh => true,
                AppEvent::Backspace if !self.block_event
                    => self.handle_del(true),
                AppEvent::Delete if !self.block_event
                    => self.handle_del(false),
                AppEvent::ServerConnectResult(r)
                    => self.handle_server_connect_result(r),
                AppEvent::StreamMsg(msg)
                    => self.handle_stream_msg(msg).await,
                _ => false,
            }
        }
    }

    pub async fn run(&mut self) -> AppResult<()> {
        // Client Workflow
        //  1. new room
        //  2. join room -> stream
        //  3. room status -> draw first
        //  4. listen stream and draw
        //  5. get a roominfo: new player join in, if all 4 join in, display ready state
        //  6. get a whoready: someone get ready
        //  7. get a start: server start game, and client should rpc GameStatus to get cards
        //  8. continue listen stream
        //  9. rpc ExitGame after user confirm the gameresult
        // 10. return to WaitReady
        // 11. handle when someone exits
        // 12. handle Esc of all states

        let mut draw_or_not = true;
        loop {
            if draw_or_not {
                self.draw()?;
            }
            tokio::select! {
                _ = self.cancel.cancelled() => {
                    break;
                }
                action = self.rx.recv() => {
                    draw_or_not = match action {
                        None => panic!("Channel to app closed!"),
                        Some(a) => self.appevent_dispatcher(a).await,
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self) -> AppResult<()> {
        if self.sz.0 < 160 || self.sz.1 < 48 {
            self.block_event = true;
            self.tui.draw(|frame| ui::resize(frame, self.sz))?;
        } else {
            self.block_event = false;
            self.tui.draw(|frame| ui::render(frame, &self.state, self.exitmenu))?;
        }
        Ok(())
    }

    pub fn exit(&mut self) -> AppResult<()> {
        // self.cancel.cancel();
        self.tui.exit()?;
        Ok(())
    }
}
