mod rpc;
mod msg_handler;
mod key_handler;
mod exit_handler;
mod input;
mod logging;

pub use rpc::{RpcClient, GameStream};
use std::panic;
pub use tonic::{Code, Request, Response, Status};
pub use heart7_rule::*;
pub use input::*;
pub use logging::*;

pub type RPCResult<T> = Result<T, tonic::Status>;

pub struct ClientState {
    pub exitmenu: (bool, u32),
    pub fsm: ClientStateMachine,
}

pub struct ClientStateBrief {
    pub exitmenu: (bool, u32),
    pub fsm: ClientStateMachineBrief,
}

pub enum ClientStateMachineBrief {
    GetServer,
    AskName,
    NewRoom,
    JoinRoom,
    WaitPlayer,
    WaitReady,
    Gaming,
    GameResult,
}

pub enum ClientStateMachine {
    GetServer {
        input: Input,
        msg: String,
        connecting: bool,
    },
    AskName {
        input: Input,
        msg: String,
        button: u16,
        is_input: bool,
    },
    NewRoom {
        input: Input,
        msg: String,
        name: String,
    },
    JoinRoom {
        input: Input,
        msg: String,
        name: String,
    },
    WaitPlayer {
        players: Vec<(String, usize, bool)>,
        msg: Vec<String>,
        roomid: String,
    },
    WaitReady {
        players: Vec<(String, usize, bool)>,
        msg: Vec<String>,
        roomid: String,
    },
    Gaming {
        choose: usize, // 0 for none
        game: Game,
        my_remote_idx: usize,
        roomid: String,
        button: u32,
        msg: Option<String>,
    },
    GameResult {
        ds: Vec<Vec<(Card, usize)>>,
        my_remote_idx: usize,
        players: Vec<(String, Vec<Card>)>,
        roomid: String,
    },
}

#[derive(Clone)]
enum ClientStateInternal {
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
    },
    WaitPlayer {
        client: RpcClient,
        players: Vec<(String, usize, bool)>,
        msg: Vec<String>,
        roomid: String,
    },
    WaitReady {
        client: RpcClient,
        players: Vec<(String, usize, bool)>,
        msg: Vec<String>,
        roomid: String,
    },
    Gaming {
        client: RpcClient,
        choose: usize, // 0 for none
        game: Game,
        my_remote_idx: usize,
        roomid: String,
        button: u32,
        msg: Option<String>,
    },
    GameResult {
        ds: Vec<Vec<(Card, usize)>>,
        client: RpcClient,
        my_remote_idx: usize,
        players: Vec<(String, Vec<Card>)>,
        roomid: String,
    },
}

impl Into<ClientStateMachine> for ClientStateInternal {
    fn into(self) -> ClientStateMachine {
        match self {
            ClientStateInternal::GetServer {
                input, msg, connecting
            } => ClientStateMachine::GetServer {
                input, msg, connecting
            },
            ClientStateInternal::AskName {
                input, msg, button, is_input, ..
            } => ClientStateMachine::AskName {
                input, msg, button, is_input
            },
            ClientStateInternal::NewRoom {
                input, msg, name, ..
            } => ClientStateMachine::NewRoom {
                input, msg, name
            },
            ClientStateInternal::JoinRoom {
                input, msg, name, ..
            } => ClientStateMachine::JoinRoom {
                input, msg, name,
            },
            ClientStateInternal::WaitPlayer {
                players, msg, roomid, ..
            } => ClientStateMachine::WaitPlayer {
                players, msg, roomid
            },
            ClientStateInternal::WaitReady {
                players, msg, roomid, ..
            } => ClientStateMachine::WaitReady {
                players, msg, roomid
            },
            ClientStateInternal::Gaming {
                choose, game, my_remote_idx, roomid, button, msg, ..
            } => ClientStateMachine::Gaming{
                choose, game, my_remote_idx, roomid, button, msg
            },
            ClientStateInternal::GameResult {
                ds, my_remote_idx, players, roomid, ..
            } => ClientStateMachine::GameResult {
                ds, my_remote_idx, players, roomid
            },
        }
    }
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
    StreamListenerSpawned,
    StreamMsg(GameMsg),
    ResetInput(String),
}

#[derive(Default)]
pub struct ClientStateAdvanceReply {
    pub need_redraw: bool,
    pub cancel_stream_listener: bool,
    pub full_exit: bool,
    pub spawn_rpc_client: Option<String>,
    pub spawn_stream_listener: Option<GameStream>,
}

impl ClientStateAdvanceReply {
    pub fn redraw(&mut self) {
        self.need_redraw = true;
    }

    pub fn cancel_stream(&mut self) {
        self.cancel_stream_listener = true;
    }

    pub fn full_exit(&mut self) {
        self.full_exit = true;
    }
}

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

pub struct ClientStateManager {
    state: ClientStateInternal,
    exitmenu: (bool, u32), // (inside exitmenu or not, which button is choosed)
    default_addr: String,
}

impl ClientStateManager {
    pub fn new(default_addr: String) -> Self {
        Self {
            exitmenu: (false, 0),
            state: ClientStateInternal::GetServer {
                input: Input::new(default_addr.clone()).with_cursor(0),
                msg: "Welcome to Seven-of-Heart !!!\n\
                    Please enter game server address:".into(),
                connecting: false,
            },
            default_addr,
        }
    }

    // returns: (redraw, need_cancel)
    pub async fn advance(&mut self, e: ClientEvent, blocked: bool) -> ClientStateAdvanceReply {
        let mut reply = ClientStateAdvanceReply::default();
        let redraw = if self.exitmenu.0 {
            match e {
                ClientEvent::Esc if !blocked
                    => self.handle_esc(),
                ClientEvent::Enter | ClientEvent::UpArrow | ClientEvent::DownArrow if !blocked => {
                    let (cancel_stream, full_exit) =
                        self.handle_exitmenu_event(e.into()).await;
                    if cancel_stream {
                        reply.cancel_stream();
                    }
                    if full_exit {
                        reply.full_exit();
                    }
                    true
                }
                ClientEvent::CtrlC => panic!("Got Ctrl-C!"),
                ClientEvent::Resize(_, _) => true,
                ClientEvent::Refresh => true,
                ClientEvent::ServerConnectResult(r)
                    => self.handle_server_connect_result(r),
                ClientEvent::StreamMsg(msg)
                    => self.handle_stream_msg(msg).await,
                ClientEvent::StreamListenerSpawned
                    => self.handle_stream_listener_spawned(),
                ClientEvent::ResetInput(new_input)
                    => self.handle_reset_input(new_input),
                _ => false,
            }
        } else {
            match e {
                ClientEvent::Esc if !blocked
                    => self.handle_esc(),
                ClientEvent::Enter if !blocked => {
                    let (redraw, spawn_rpc_client, spawn_stream_listener) = self.handle_enter().await;
                    reply.spawn_rpc_client = spawn_rpc_client;
                    reply.spawn_stream_listener = spawn_stream_listener;
                    redraw
                },
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
                ClientEvent::Resize(_, _) => true,
                ClientEvent::Refresh => true,
                ClientEvent::Backspace if !blocked
                    => self.handle_del(true),
                ClientEvent::Delete if !blocked
                    => self.handle_del(false),
                ClientEvent::ServerConnectResult(r)
                    => self.handle_server_connect_result(r),
                ClientEvent::StreamMsg(msg)
                    => self.handle_stream_msg(msg).await,
                ClientEvent::StreamListenerSpawned
                    => self.handle_stream_listener_spawned(),
                ClientEvent::ResetInput(new_input)
                    => self.handle_reset_input(new_input),
                _ => false,
            }
        };
        if redraw {
            reply.redraw();
        }
        reply
    }

    pub fn get_client_state(&self) -> ClientState {
        ClientState {
            exitmenu: self.exitmenu.clone(),
            fsm: self.state.clone().into(),
        }
    }

    pub fn get_client_state_brief(&self) -> ClientStateBrief {
        let fsm = match self.state {
            ClientStateInternal::GetServer{..} => ClientStateMachineBrief::GetServer,
            ClientStateInternal::AskName{..} => ClientStateMachineBrief::AskName,
            ClientStateInternal::NewRoom{..} => ClientStateMachineBrief::NewRoom,
            ClientStateInternal::JoinRoom{..} => ClientStateMachineBrief::JoinRoom,
            ClientStateInternal::WaitPlayer{..} => ClientStateMachineBrief::WaitPlayer,
            ClientStateInternal::WaitReady{..} => ClientStateMachineBrief::WaitReady,
            ClientStateInternal::Gaming{..} => ClientStateMachineBrief::Gaming,
            ClientStateInternal::GameResult{..} => ClientStateMachineBrief::GameResult,
        };
        ClientStateBrief {
            exitmenu: self.exitmenu.clone(),
            fsm,
        }
    }
}
