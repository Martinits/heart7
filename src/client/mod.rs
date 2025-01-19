mod rpc;
mod msg_handler;
mod key_handler;
mod exit_handler;

use crate::*;
use crate::client::rpc::RpcClient;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tui::Tui;
use tui_input::Input;
use crate::rule::*;
use std::panic;
use exit_handler::ExitMenuEvent;
use anyhow::Result;

pub enum ClientState {
    GetServer {
        input: Input,
        msg: String,
        connecting: bool,
    },
    AskName {
        client: RpcClient,
        input: Input,
        msg: String,
        button: u16,
        is_input: bool,
    },
    NewRoom {
        client: RpcClient,
        input: Input,
        msg: String,
        name: String,
    },
    JoinRoom {
        client: RpcClient,
        input: Input,
        msg: String,
        name: String,
        stream_listener_cancel: CancellationToken,
    },
    WaitPlayer {
        client: RpcClient,
        players: Vec<(String, usize, bool)>,
        msg: Vec<String>,
        roomid: String,
        stream_listener_cancel: CancellationToken,
    },
    WaitReady {
        client: RpcClient,
        players: Vec<(String, usize, bool)>,
        msg: Vec<String>,
        roomid: String,
        stream_listener_cancel: CancellationToken,
    },
    Gaming {
        client: RpcClient,
        choose: usize, // 0 for none
        game: Game,
        my_remote_idx: usize,
        roomid: String,
        button: u32,
        msg: Option<String>,
        stream_listener_cancel: CancellationToken,
    },
    GameResult {
        ds: Vec<Vec<(Card, usize)>>,
        client: RpcClient,
        my_remote_idx: usize,
        players: Vec<(String, Vec<Card>)>,
        roomid: String,
        stream_listener_cancel: CancellationToken,
    },
}

pub enum ClientEvent {
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
    ServerConnectResult(Result<RpcClient, String>),
    StreamMsg(GameMsg),
}

pub struct Client {
    tui: Tui,
    cancel: CancellationToken,
    state: ClientState,
    tx: mpsc::Sender<ClientEvent>,
    rx: mpsc::Receiver<ClientEvent>,
    exitmenu: (bool, u32),
    default_addr: String,
}

impl Client {
    pub async fn new(
        cancel: CancellationToken,
        tx: mpsc::Sender<ClientEvent>,
        rx: mpsc::Receiver<ClientEvent>,
        default_addr: String,
    ) -> Result<Self> {
        Ok(Self {
            tui: Tui::new(cancel.clone())?,
            exitmenu: (false, 0),
            cancel,
            state: ClientState::GetServer {
                input: Input::new(default_addr.clone()).with_cursor(0),
                msg: "Welcome to Seven-of-Heart !!!\n\
                        Please enter game server address:".into(),
                connecting: false,
            },
            tx,
            rx,
            default_addr,
        })
    }

    async fn event_dispatcher(&mut self, a: ClientEvent) -> bool {
        let blocked = self.tui.should_block().unwrap();

        if self.exitmenu.0 {
            match a {
                ClientEvent::Esc if !blocked
                    => self.handle_esc(),
                ClientEvent::Enter if !blocked
                    => self.handle_exitmenu_event(ExitMenuEvent::Enter).await,
                ClientEvent::UpArrow if !blocked
                    => self.handle_exitmenu_event(ExitMenuEvent::MoveUp).await,
                ClientEvent::DownArrow if !blocked
                    => self.handle_exitmenu_event(ExitMenuEvent::MoveDown).await,
                ClientEvent::CtrlC
                    => panic!("Got Ctrl-C!"),
                ClientEvent::Resize(x, y) => {
                    self.tui.resize((x, y)).unwrap();
                    true
                },
                ClientEvent::Refresh => true,
                ClientEvent::ServerConnectResult(r)
                    => self.handle_server_connect_result(r),
                ClientEvent::StreamMsg(msg)
                    => self.handle_stream_msg(msg).await,
                _ => false,
            }
        } else {
            match a {
                ClientEvent::Esc if !blocked
                    => self.handle_esc(),
                ClientEvent::Enter if !blocked
                    => self.handle_enter().await,
                ClientEvent::LeftArrow if !blocked
                    => self.handle_lr_arrow(true),
                ClientEvent::RightArrow if !blocked
                    => self.handle_lr_arrow(false),
                ClientEvent::UpArrow if !blocked
                    => self.handle_ud_arrow(true),
                ClientEvent::DownArrow if !blocked
                    => self.handle_ud_arrow(false),
                ClientEvent::Type(c) if !blocked
                    => self.handle_typing(c),
                ClientEvent::CtrlC
                    => panic!("Got Ctrl-C!"),
                ClientEvent::Resize(x, y) => {
                    self.tui.resize((x, y)).unwrap();
                    true
                },
                ClientEvent::Refresh => true,
                ClientEvent::Backspace if !blocked
                    => self.handle_del(true),
                ClientEvent::Delete if !blocked
                    => self.handle_del(false),
                ClientEvent::ServerConnectResult(r)
                    => self.handle_server_connect_result(r),
                ClientEvent::StreamMsg(msg)
                    => self.handle_stream_msg(msg).await,
                _ => false,
            }
        }
    }

    pub async fn run(&mut self) -> Result<()> {
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

        let mut need_redraw = true;
        loop {
            if need_redraw {
                self.draw()?;
            }
            tokio::select! {
                _ = self.cancel.cancelled() => {
                    break;
                }
                action = self.rx.recv() => {
                    need_redraw = match action {
                        None => panic!("Channel to client closed!"),
                        Some(a) => self.event_dispatcher(a).await,
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        if self.tui.should_block()? {
            self.tui.draw_blocked()?;
        } else {
            self.tui.draw(&mut self.state, self.exitmenu)?;
        }
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        // self.cancel.cancel();
        self.tui.exit()?;
        Ok(())
    }
}
