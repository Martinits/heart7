use std::error::Error;
use crate::*;
use crate::client::rpc::{self, Client};
use super::ui;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use ratatui::backend::Backend;
use ratatui::layout::Rect;
use tui::tui::Tui;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;
use crossterm::event::{
    Event as CrosstermEvent,
    KeyEvent,
    KeyCode,
    KeyModifiers,
};
use crate::game::Card;
use crate::client::desk::*;

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

impl Default for AppState {
    fn default() -> Self {
        AppState::GetServer {
            input: server_addr_prompt(),
            msg: "Welcome to Seven-of-Heart !!!\n\
                    Please enter game server address:".into(),
            connecting: false,
        }
    }
}

fn server_addr_prompt() -> Input {
    Input::new(format!("127.0.0.1:{}", DEFAULT_PORT)).with_cursor(0)
}

pub enum Action {
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

pub struct App<B: Backend> {
    tui: Tui<B>,
    cancel: CancellationToken,
    state: AppState,
    tx: mpsc::Sender<Action>,
    rx: mpsc::Receiver<Action>,
    block_event: bool,
    sz: (u16, u16),
    exitmenu: (bool, u32),
}

impl<B: Backend> App<B> {
    pub async fn new(
        tui: Tui<B>,
        cancel: &CancellationToken,
        tx: mpsc::Sender<Action>,
        rx: mpsc::Receiver<Action>,
        sz: Rect,
    ) -> Self {
        Self {
            tui,
            block_event: false,
            exitmenu: (false, 0),
            sz: (sz.width, sz.height),
            cancel: cancel.clone(),
            state: Default::default(),
            // state: AppState::Gaming{
            //     client: Client{
            //         c: heart7_client::Heart7Client::connect("http://127.0.0.1:20007").await.unwrap(),
            //         addr:"127.0.0.1:20007".into()
            //     },
            //     players: vec![
            //         ("first".into(), 0, 0),
            //         ("second".into(), 1, 0),
            //         ("third".into(), 2, 0),
            //         ("fourth".into(), 3, 0)
            //     ],
            //     next: 3,
            //     choose: 0,
            //     last: Some(Card{suit:CardSuit::Spade, num: 13}),
            //     cards: vec![
            //         Card { suit: CardSuit::Spade, num: 3},
            //         Card { suit: CardSuit::Diamond, num: 11},
            //         Card { suit: CardSuit::Club, num: 13},
            //         Card { suit: CardSuit::Spade, num: 4},
            //         Card { suit: CardSuit::Heart, num: 1},
            //         Card { suit: CardSuit::Diamond, num: 12},
            //         Card { suit: CardSuit::Heart, num: 7},
            //         Card { suit: CardSuit::Spade, num: 6},
            //         Card { suit: CardSuit::Club, num: 2},
            //         Card { suit: CardSuit::Heart, num: 8},
            //         Card { suit: CardSuit::Spade, num: 10},
            //         Card { suit: CardSuit::Diamond, num: 3},
            //         Card { suit: CardSuit::Club, num: 1},
            //     ],
            //     holds: vec![
            //         Card { suit: CardSuit::Spade, num: 4},
            //         Card { suit: CardSuit::Heart, num: 1},
            //         Card { suit: CardSuit::Diamond, num: 12},
            //         Card { suit: CardSuit::Heart, num: 7},
            //         Card { suit: CardSuit::Spade, num: 6},
            //         Card { suit: CardSuit::Club, num: 2},
            //         Card { suit: CardSuit::Heart, num: 8},
            //         Card { suit: CardSuit::Spade, num: 10},
            //         Card { suit: CardSuit::Diamond, num: 3},
            //         Card { suit: CardSuit::Club, num: 1},
            //     ],
            //     has_last: true,
            //     desk: Desk {
            //         diamond: (vec![
            //             (Card{suit:CardSuit::Diamond, num: 1}, 1),
            //             (Card{suit:CardSuit::Diamond, num: 2}, 2),
            //             (Card{suit:CardSuit::Diamond, num: 3}, 1),
            //             (Card{suit:CardSuit::Diamond, num: 4}, 1),
            //         ],
            //         vec![
            //             (Card{suit:CardSuit::Diamond, num: 13}, 0),
            //         ]),
            //         ..Default::default()
            //     },
            //     roomid: "jbhfvhsbdfvhbkdsfhbv".into(),
            //     button: 0,
            //     play_cnt: 0,
            //     msg: None,
            //     stream_listener_cancel: CancellationToken::new(),
            // },
            // state: AppState::GameResult{
            //     stream_listener_cancel: CancellationToken::new(),
            //     ds: vec![
            //         vec![
            //             (Card{suit: CardSuit::Spade, num: 1}, 3),
            //             (Card{suit: CardSuit::Spade, num: 2}, 0),
            //             (Card{suit: CardSuit::Spade, num: 3}, 1),
            //             (Card{suit: CardSuit::Spade, num: 4}, 1),
            //             (Card{suit: CardSuit::Spade, num: 5}, 2),
            //             (Card{suit: CardSuit::Spade, num: 6}, 3),
            //             (Card{suit: CardSuit::Spade, num: 7}, 0),
            //             (Card{suit: CardSuit::Spade, num: 8}, 0),
            //             (Card{suit: CardSuit::Spade, num: 9}, 1),
            //             (Card{suit: CardSuit::Spade, num: 10}, 2),
            //             (Card{suit: CardSuit::Spade, num: 11}, 3),
            //             (Card{suit: CardSuit::Spade, num: 12}, 0),
            //             (Card{suit: CardSuit::Spade, num: 13}, 1),
            //         ],
            //         vec![
            //             (Card{suit: CardSuit::Heart, num: 1}, 2),
            //             (Card{suit: CardSuit::Heart, num: 2}, 3),
            //             (Card{suit: CardSuit::Heart, num: 3}, 0),
            //             (Card{suit: CardSuit::Heart, num: 4}, 0),
            //             (Card{suit: CardSuit::Heart, num: 5}, 1),
            //             (Card{suit: CardSuit::Heart, num: 6}, 3),
            //             (Card{suit: CardSuit::Heart, num: 7}, 0),
            //             (Card{suit: CardSuit::Heart, num: 8}, 2),
            //             (Card{suit: CardSuit::Heart, num: 9}, 3),
            //             (Card{suit: CardSuit::Heart, num: 10}, 1),
            //             (Card{suit: CardSuit::Heart, num: 11}, 0),
            //             (Card{suit: CardSuit::Heart, num: 12}, 2),
            //             (Card{suit: CardSuit::Heart, num: 13}, 1),
            //         ],
            //         vec![
            //             (Card{suit: CardSuit::Club, num: 3}, 0),
            //             (Card{suit: CardSuit::Club, num: 4}, 2),
            //             (Card{suit: CardSuit::Club, num: 5}, 0),
            //             (Card{suit: CardSuit::Club, num: 6}, 2),
            //             (Card{suit: CardSuit::Club, num: 7}, 1),
            //             (Card{suit: CardSuit::Club, num: 8}, 2),
            //             (Card{suit: CardSuit::Club, num: 9}, 1),
            //             (Card{suit: CardSuit::Club, num: 10}, 3),
            //             (Card{suit: CardSuit::Club, num: 11}, 3),
            //             (Card{suit: CardSuit::Club, num: 12}, 0),
            //         ],
            //         vec![
            //             (Card{suit: CardSuit::Diamond, num: 1}, 2),
            //             (Card{suit: CardSuit::Diamond, num: 2}, 1),
            //             (Card{suit: CardSuit::Diamond, num: 3}, 0),
            //             (Card{suit: CardSuit::Diamond, num: 4}, 1),
            //             (Card{suit: CardSuit::Diamond, num: 5}, 1),
            //             (Card{suit: CardSuit::Diamond, num: 6}, 3),
            //             (Card{suit: CardSuit::Diamond, num: 7}, 1),
            //             (Card{suit: CardSuit::Diamond, num: 8}, 0),
            //             (Card{suit: CardSuit::Diamond, num: 9}, 0),
            //             (Card{suit: CardSuit::Diamond, num: 10}, 2),
            //             (Card{suit: CardSuit::Diamond, num: 11}, 2),
            //             (Card{suit: CardSuit::Diamond, num: 12}, 1),
            //             (Card{suit: CardSuit::Diamond, num: 13}, 0),
            //         ],
            //     ],
            //     client: Client{
            //         c: heart7_client::Heart7Client::connect("http://127.0.0.1:20007").await.unwrap(),
            //         addr:"127.0.0.1:20007".into()
            //     },
            //     roomid: "jbhfvhsbdfvhbkdsfhbv".into(),
            //     players: vec![
            //         ("first".into(), 0, vec![
            //             Card { suit: CardSuit::Spade, num: 4},
            //             Card { suit: CardSuit::Heart, num: 1},
            //             Card { suit: CardSuit::Diamond, num: 12},
            //             Card { suit: CardSuit::Heart, num: 7},
            //             Card { suit: CardSuit::Spade, num: 6},
            //             Card { suit: CardSuit::Club, num: 2},
            //             Card { suit: CardSuit::Heart, num: 8},
            //             Card { suit: CardSuit::Spade, num: 10},
            //             Card { suit: CardSuit::Diamond, num: 3},
            //             Card { suit: CardSuit::Club, num: 1},
            //             Card { suit: CardSuit::Heart, num: 8},
            //             Card { suit: CardSuit::Club, num: 9},
            //             Card { suit: CardSuit::Diamond, num: 3},
            //         ]),
            //         ("second".into(), 1, vec![
            //             Card { suit: CardSuit::Spade, num: 4},
            //             Card { suit: CardSuit::Heart, num: 1},
            //             Card { suit: CardSuit::Diamond, num: 12},
            //             Card { suit: CardSuit::Heart, num: 7},
            //             Card { suit: CardSuit::Spade, num: 6},
            //             Card { suit: CardSuit::Club, num: 2},
            //             Card { suit: CardSuit::Heart, num: 8},
            //             Card { suit: CardSuit::Spade, num: 10},
            //             Card { suit: CardSuit::Diamond, num: 3},
            //             Card { suit: CardSuit::Club, num: 1},
            //             Card { suit: CardSuit::Heart, num: 8},
            //             Card { suit: CardSuit::Club, num: 9},
            //             Card { suit: CardSuit::Diamond, num: 3},
            //         ]),
            //         ("third".into(), 2, vec![
            //             Card { suit: CardSuit::Spade, num: 4},
            //             Card { suit: CardSuit::Heart, num: 1},
            //             Card { suit: CardSuit::Diamond, num: 12},
            //             Card { suit: CardSuit::Heart, num: 7},
            //             Card { suit: CardSuit::Spade, num: 6},
            //             Card { suit: CardSuit::Club, num: 2},
            //             Card { suit: CardSuit::Heart, num: 8},
            //             Card { suit: CardSuit::Spade, num: 10},
            //             Card { suit: CardSuit::Diamond, num: 3},
            //             Card { suit: CardSuit::Club, num: 1},
            //             Card { suit: CardSuit::Heart, num: 8},
            //             Card { suit: CardSuit::Club, num: 9},
            //             Card { suit: CardSuit::Diamond, num: 3},
            //         ]),
            //         ("fourth".into(), 3, vec![
            //             Card { suit: CardSuit::Spade, num: 4},
            //             Card { suit: CardSuit::Heart, num: 1},
            //             Card { suit: CardSuit::Diamond, num: 12},
            //             Card { suit: CardSuit::Heart, num: 7},
            //             Card { suit: CardSuit::Spade, num: 6},
            //             Card { suit: CardSuit::Club, num: 2},
            //             Card { suit: CardSuit::Heart, num: 8},
            //             Card { suit: CardSuit::Spade, num: 10},
            //             Card { suit: CardSuit::Diamond, num: 3},
            //             Card { suit: CardSuit::Club, num: 1},
            //             Card { suit: CardSuit::Heart, num: 8},
            //             Card { suit: CardSuit::Club, num: 9},
            //             Card { suit: CardSuit::Diamond, num: 3},
            //         ])
            //     ]
            // },
            tx,
            rx,
        }
    }

    pub fn init(&mut self) -> AppResult<()> {
        self.tui.init(&self.cancel)?;
        Ok(())
    }

    pub async fn run(&mut self) -> AppResult<()> {
        // client workflow
        // new room
        // join room -> stream
        // room status -> draw first
        // listen stream and draw
        // get a roominfo: new player join in, if all 4 join in, display ready state
        // get a whoready: someone get ready
        // get a start: server start game, and client should rpc GameStatus to get cards
        // continue listen stream
        // rpc ExitGame after user confirm the gameresult
        // return to WaitReady
        // handle when someone exits
        // handle Esc of all states

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
                        Some(a) => {
                            if self.exitmenu.0 {
                                match a {
                                    Action::Esc if !self.block_event
                                        => self.handle_esc(),
                                    Action::Enter if !self.block_event
                                        => self.handle_exitmenu_event(0).await,
                                    Action::UpArrow if !self.block_event
                                        => self.handle_exitmenu_event(1).await,
                                    Action::DownArrow if !self.block_event
                                        => self.handle_exitmenu_event(2).await,
                                    Action::CtrlC
                                        => panic!("Got Ctrl-C!"),
                                    Action::Resize(x, y) => {
                                        self.sz = (x, y);
                                        true
                                    },
                                    Action::Refresh => true,
                                    Action::ServerConnectResult(r)
                                        => self.handle_server_connect_result(r),
                                    Action::StreamMsg(msg)
                                        => self.handle_stream_msg(msg).await,
                                    _ => false,
                                }
                            } else {
                                match a {
                                    Action::Esc if !self.block_event
                                        => self.handle_esc(),
                                    Action::Enter if !self.block_event
                                        => self.handle_enter().await,
                                    Action::LeftArrow if !self.block_event
                                        => self.handle_lr_arrow(true),
                                    Action::RightArrow if !self.block_event
                                        => self.handle_lr_arrow(false),
                                    Action::UpArrow if !self.block_event
                                        => self.handle_ud_arrow(true),
                                    Action::DownArrow if !self.block_event
                                        => self.handle_ud_arrow(false),
                                    Action::Type(c) if !self.block_event
                                        => self.handle_type(c),
                                    Action::CtrlC
                                        => panic!("Got Ctrl-C!"),
                                    Action::Resize(x, y) => {
                                        self.sz = (x, y);
                                        true
                                    },
                                    Action::Refresh => true,
                                    Action::Backspace if !self.block_event
                                        => self.handle_del(true),
                                    Action::Delete if !self.block_event
                                        => self.handle_del(false),
                                    Action::ServerConnectResult(r)
                                        => self.handle_server_connect_result(r),
                                    Action::StreamMsg(msg)
                                        => self.handle_stream_msg(msg).await,
                                    _ => false,
                                }
                            }
                        }
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

    async fn handle_enter(&mut self) -> bool {
        match self.state {
            AppState::GetServer {
                ref mut input, ref mut msg, ref mut connecting
            } if ! *connecting => {
                // connect to server
                Client::connect_spawn(input.value(), &self.tx);
                *connecting = true;
                *msg = format!("Try connecting to {} ......", input.value());
                true
            }
            AppState::AskName {
                ref input, button, client: ref mut c, is_input, ..
            } if !is_input => {
                if button == 0{
                    // new room
                    info!("Player {} chooses to new room", input.value());
                    self.state = AppState::NewRoom {
                        client: c.clone(),
                        input: Input::default(),
                        name: input.value().into(),
                        msg: format!("Hello, {}!\n\
                                Please enter new room name:", input.value()),
                    }
                } else {
                    //join room
                    info!("Player {} chooses to join room, enter JoinRoom state", input.value());
                    self.state = AppState::JoinRoom {
                        name: input.value().into(),
                        input: Input::default(),
                        client: c.clone(),
                        msg: format!("Hello, {}!\n\
                                Please enter room ID:", input.value()),
                        stream_listener_cancel: CancellationToken::new(),
                    };
                    self.exitmenu.1 = 0;
                }
                true
            }
            AppState::NewRoom { client: ref mut c, ref input, ref name, ref mut msg} => {
                match c.new_room(input.value().into()).await {
                    Ok(()) => {
                        info!("Get NewRoom result from server, enter JoinRoom state");
                        self.state = AppState::JoinRoom {
                            client: c.clone(),
                            input: Input::new(input.value().into()),
                            msg: format!("Hello, {}!\n\
                                    Successfully created a room, ID is shown below.\n\
                                    Please press ENTER to join room:", name),
                            name: name.clone(),
                            stream_listener_cancel: CancellationToken::new(),
                        };
                        self.exitmenu.1 = 0;
                    },
                    Err(s) => {
                        *msg = format!("Making NewRoom request to server failed:\n\
                                        {}\n\
                                        Please retry:", s);
                    }
                }
                true
            }
            AppState::JoinRoom {
                ref input, ref mut msg, client: ref mut c, ref name,
                stream_listener_cancel: ref cancel
            } => {
                info!("Joining room {}", input.value());
                match c.join_room(name.clone(), input.value().into()).await {
                    Ok(stream) => {
                        // spawn stream listerning task
                        info!("Spawning GameStream listener...");
                        Client::spawn_stream_listener(stream, cancel, &self.tx);
                        info!("Querying RoomStatus...");
                        match c.room_status(input.value().into()).await {
                            Ok(rs) => match rs.state {
                                Some(State::NotFull(_)) => {
                                    info!("Join room {}, enter WaitPlayer state", input.value());
                                    self.state = AppState::WaitPlayer {
                                        players: rpc::room_info_to_players(name, &rs),
                                        client: c.clone(),
                                        roomid: input.value().into(),
                                        msg: vec!["Waiting for other players to join room......".into()],
                                        stream_listener_cancel: cancel.clone(),
                                    };
                                    self.exitmenu.1 = 0;
                                }
                                Some(State::WaitReady(_)) => {
                                    info!("Join room {}, enter WaitReady state", input.value());
                                    self.state = AppState::WaitReady {
                                        players: rpc::room_info_to_players(name, &rs),
                                        client: c.clone(),
                                        roomid: input.value().into(),
                                        msg: vec!["Please press ENTER to get ready!".into()],
                                        stream_listener_cancel: cancel.clone(),
                                    };
                                    self.exitmenu.1 = 0;
                                }
                                _ => panic!("Unexpected RoomStatus after JoinRoom!"),
                            }
                            Err(s) => panic!("Failed to get RoomStatus after JoinRoom: {}", s),
                        }
                    }
                    Err(s) => {
                        *msg = format!("Making JoinRoom request to server failed:\n\
                                        {}\n\
                                        Please retry:", s);
                    }
                }
                true
            }
            AppState::WaitReady {
                ref mut client, ref mut players, ref roomid, ref mut msg, ..
            } if !players[0].2 => {
                match client.game_ready(players[0].1 as u32, roomid.clone()).await {
                    Ok(_) => {
                        players[0].2 = true;
                        *msg = vec!["Waiting for other players to get ready......".into()];
                    }
                    Err(s) => panic!("Failed to GetReady: {}", s),
                }
                true
            }
            AppState::Gaming {
                client: ref mut c, ref players, ref mut choose, ref mut cards,
                ref mut holds, ref roomid, ref button, ref next, ref mut msg, ..
            } if cards.len() != 0 && *choose != 0 && *next == 0 => {
                let play = match *button {
                        0 => Play::Discard(cards[*choose-1].clone().into()),
                        _ => Play::Hold(cards[*choose-1].clone().into())
                };
                match c.play_card(players[0].1 as u32, roomid.clone(), play).await {
                    Ok(_) => {
                        let c = cards.remove(*choose-1);
                        *choose = 0;
                        *msg = None;
                        if *button == 1 {
                            holds.push(c);
                            holds.sort();
                        }
                    },
                    Err(s) => {
                        if let Ok(status) = s.downcast::<Status>() {
                            if status.code() == Code::PermissionDenied {
                                *msg = Some(status.message().into());
                            } else {
                                panic!("Failed to play card to server");
                            }
                        } else {
                            panic!("Failed to play card to server");
                        }
                    }
                }
                true
            }
            AppState::GameResult {
                ref client, ref players, ref roomid, stream_listener_cancel: ref cancel, ..
            } => {
                info!("Confirmed GameResult, enter WaitReady state");
                self.state = AppState::WaitReady {
                    players: players.iter().map(
                        |p| (p.0.clone(), p.1, false)
                    ).collect(),
                    client: client.clone(),
                    roomid: roomid.clone(),
                    msg: vec!["Please press ENTER to get ready!".into()],
                    stream_listener_cancel: cancel.clone(),
                };
                self.exitmenu.1 = 0;
                true
            }
            _ => {
                false
            }
        }
    }

    fn handle_type(&mut self, c: char) -> bool {
        match self.state {
            AppState::GetServer {ref mut input, connecting, ..} if !connecting => {
                input.handle_event(
                    &CrosstermEvent::Key(
                        KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)
                    )
                );
                true
            }
            AppState::AskName {ref mut input, is_input, ..} if is_input => {
                input.handle_event(
                    &CrosstermEvent::Key(
                        KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)
                    )
                );
                true
            }
            AppState::NewRoom {ref mut input, ..} | AppState::JoinRoom {ref mut input, ..} => {
                input.handle_event(
                    &CrosstermEvent::Key(
                        KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)
                    )
                );
                true
            }
            _ => {
                false
            }
        }
    }

    fn handle_lr_arrow(&mut self, is_left: bool) -> bool {
        match self.state {
            AppState::GetServer {ref mut input, connecting, ..} if !connecting => {
                input.handle_event(
                    &CrosstermEvent::Key(
                        KeyEvent::new(
                            if is_left {KeyCode::Left} else {KeyCode::Right},
                            KeyModifiers::NONE
                        )
                    )
                );
                true
            }
            AppState::AskName {ref mut input, is_input, ref mut button, ..} => {
                if is_input {
                    input.handle_event(
                        &CrosstermEvent::Key(
                            KeyEvent::new(
                                if is_left {KeyCode::Left} else {KeyCode::Right},
                                KeyModifiers::NONE
                            )
                        )
                    );
                } else {
                    *button += 1;
                    *button %= 2;
                }
                true

            }
            AppState::NewRoom {ref mut input, ..} | AppState::JoinRoom {ref mut input, ..} => {
                input.handle_event(
                    &CrosstermEvent::Key(
                        KeyEvent::new(
                            if is_left {KeyCode::Left} else {KeyCode::Right},
                            KeyModifiers::NONE
                        )
                    )
                );
                true
            }
            AppState::Gaming {
                ref mut choose, ref cards, ..
            } if cards.len() != 0 => {
                if is_left {
                    *choose += cards.len() + 1 - 1;
                } else {
                    *choose += 1;
                }
                *choose %= cards.len() + 1;
                // if is_left {
                //     if *choose > 1 {
                //         *choose -= 1;
                //     }
                // } else {
                //     *choose += 1;
                //     if *choose > cards.len() {
                //         *choose = cards.len();
                //     }
                // }
                true
            }
            _ => {
                false
            }
        }
    }

    fn handle_ud_arrow(&mut self, _is_up: bool) -> bool {
        match self.state {
            AppState::AskName { ref mut is_input, ..} => {
                *is_input = !*is_input;
                true
            }
            AppState::Gaming {
                ref mut button, ref next, ..
            } if *next == 0 => {
                *button += 1;
                *button %= 2;
                true
            }
            _ => false
        }
    }

    fn handle_esc(&mut self) -> bool {
        self.exitmenu.0 = !self.exitmenu.0;
        true
    }

    fn handle_del(&mut self, is_back: bool) -> bool {
        let keycode = if is_back {
            KeyCode::Backspace
        } else {
            KeyCode::Delete
        };
        match self.state {
            AppState::GetServer {ref mut input, connecting, ..} if !connecting => {
                input.handle_event(
                    &CrosstermEvent::Key(KeyEvent::new(keycode, KeyModifiers::NONE))
                );
                true
            }
            AppState::AskName {ref mut input, is_input, ..} if is_input => {
                input.handle_event(
                    &CrosstermEvent::Key(KeyEvent::new(keycode, KeyModifiers::NONE))
                );
                true
            }
            AppState::NewRoom {ref mut input, ..} | AppState::JoinRoom {ref mut input, ..} => {
                input.handle_event(
                    &CrosstermEvent::Key(KeyEvent::new(keycode, KeyModifiers::NONE))
                );
                true
            }
            _ => {
                false
            }
        }
    }

    fn handle_server_connect_result(&mut self, r: Result<Client, String>) -> bool {
        match self.state {
            AppState::GetServer {ref mut input, ref mut msg, ref mut connecting} => {
                if *connecting {
                    match r {
                        Ok(c) => {
                            info!("Server {} Connected, enter GetRoom state", c.get_addr());
                            self.state = AppState::AskName {
                                client: c,
                                input: Input::default(),
                                msg: "Game server connected.\n\
                                        Please enter your nickname:".into(),
                                button: 0,
                                is_input: true,
                            };
                            self.exitmenu.1 = 0;
                        },
                        Err(s) => {
                            *input = server_addr_prompt();
                            *msg = format!("Connecting to server failed:\n\
                                            {}\n\
                                            Please retry:", s);
                            *connecting = false;
                        }
                    }
                    true
                } else {
                    warn!("AppState is not connecting, drop server connecting result!");
                    false
                }
            }
            _ => false
        }
    }

    fn someone_get_ready(players: &mut Vec<(String, usize, bool)>, who: usize) {
        if let Some(p) = players.iter_mut().find(|p| p.1 == who) {
            p.2 = true;
        } else {
            panic!("Player ID {} doesn't exists!", who);
        }
    }

    async fn handle_stream_msg(&mut self, msg: GameMsg) -> bool {
        debug!("Got GameMsg: {:?}", msg);
        match self.state {
            AppState::WaitPlayer {
                ref mut client, ref mut players, ref roomid, ref stream_listener_cancel, ..
            } => {
                match msg.msg {
                    Some(Msg::RoomInfo(ri)) => {
                        *players = rpc::room_info_to_players(&players[0].0, &ri);
                        if let Some(State::WaitReady(_)) =  ri.state {
                            info!("Stream got RoomInfo: WaitReady, enter WaitReady state");
                            self.state = AppState::WaitReady{
                                client: client.clone(),
                                players: players.clone(),
                                msg: vec!["Please press ENTER to get ready!".into()],
                                roomid: roomid.clone(),
                                stream_listener_cancel: stream_listener_cancel.clone(),
                            };
                            self.exitmenu.1 = 0;
                        }
                    }
                    Some(Msg::ExitRoom(ri)) => {
                        *players = rpc::room_info_to_players(&players[0].0, &ri);
                    }
                    None => panic!("Got empty GameMsg!"),
                    _ => panic!("Got GameMsg not possible in state WaitPlayer!"),
                }
                true
            }
            AppState::WaitReady {
                ref mut client, ref mut players, ref roomid, ref stream_listener_cancel, ..
            } => {
                match msg.msg {
                    Some(Msg::RoomInfo(ri)) => {
                        *players = rpc::room_info_to_players(&players[0].0, &ri);
                    }
                    Some(Msg::WhoReady(who)) => {
                        Self::someone_get_ready(players, who as usize);
                    }
                    Some(Msg::Start(next)) => {
                        match client.game_status(players[0].1 as u32, roomid.clone()).await {
                            Ok(gi) => {
                                let mut cards: Vec<Card> = gi.cards.iter().map(
                                    |c| Card::from_info(c)
                                ).collect();
                                cards.sort();

                                self.state = AppState::Gaming{
                                    client: client.clone(),
                                    roomid: roomid.clone(),
                                    players: players.iter().map(
                                        |p| (p.0.clone(), p.1, 0)
                                    ).collect(),
                                    next: players.iter().position(|p| p.1 == next as usize)
                                            .unwrap() as usize,
                                    last: None,
                                    cards,
                                    holds: Vec::new(),
                                    desk: Default::default(),
                                    choose: 0,
                                    button: 0,
                                    has_last: false,
                                    play_cnt: 0,
                                    msg: None,
                                    stream_listener_cancel: stream_listener_cancel.clone(),
                                };
                                self.exitmenu.1 = 0;
                            }
                            Err(s) => panic!("Failed to get GameStatus on start: {}", s),
                        }
                    }
                    Some(Msg::ExitRoom(ri)) => {
                        self.state = AppState::WaitPlayer {
                            players: rpc::room_info_to_players(&players[0].0, &ri),
                            client: client.clone(),
                            roomid: roomid.clone(),
                            msg: vec!["Someone exits room.".into(),
                                "Waiting for other players to join room......".into()],
                            stream_listener_cancel: stream_listener_cancel.clone(),
                        };
                        self.exitmenu.1 = 0;
                    }
                    Some(Msg::ExitGame(_)) => {
                        // if who as usize != players[0].1 {
                        //     panic!("Got ExitGame but not myself in state WaitReady!")
                        // }
                    }
                    None => panic!("Got empty GameMsg!"),
                    _ => panic!("Got GameMsg not possible in state WaitReady!"),
                }
                true
            }
            AppState::Gaming {
                ref mut players, ref mut next, ref mut last, ref mut has_last,
                ref mut desk, ref mut play_cnt, ref mut client, ref roomid, ref holds,
                stream_listener_cancel: ref cancel, ..
            } => {
                match msg.msg {
                    Some(Msg::Play(PlayInfo { player: pid, playone })) => {
                        assert!(pid < 4);
                        assert!(players[*next].1 == pid as usize);
                        if playone == None {
                            panic!("Empty PlayInfo in GameMsg Play!");
                        } else if let Some(po) = playone {
                            if po.play == None {
                                panic!("Empty PlayOne in GameMsg Play!");
                            } else if let Some(play) = po.play {
                                *play_cnt += 1;
                                if *play_cnt%4 == 1 {
                                    desk.new_round();
                                }
                                match play {
                                    Play::Discard(ci) => {
                                        let c = Card::from_info(&ci);
                                        *last = Some(c.clone());
                                        desk.add(c.clone(), *next == 0);
                                    }
                                    Play::Hold(ci) => {
                                        assert!(ci.num == 0 && ci.suit == 0);
                                        *last = None;
                                        players[*next].2 += 1;
                                    }
                                }
                                *next += 1;
                                *next %= 4;
                                *has_last = true;
                            }
                        }
                    }
                    Some(Msg::Endgame(GameResult { desk, hold })) => {
                        if let Some(ds) = desk {
                            // actually it should be already sorted
                            // holds.sort();
                            self.state = AppState::GameResult{
                                ds: Self::parse_desk_result(&ds, players),
                                players: Self::parse_hold_result(&hold, players, holds),
                                client: client.clone(),
                                roomid: roomid.clone(),
                                stream_listener_cancel: cancel.clone(),
                            };
                            self.exitmenu.1 = 0;
                        } else {
                            panic!("Empty DeskResult in GameResult from server!");
                        }
                    }
                    Some(Msg::ExitGame(who)) => {
                        let exit_name = players.iter().find(|p| p.1 == who as usize).unwrap().0.clone();
                        self.state = AppState::WaitReady {
                            client: client.clone(),
                            players: players.iter().map(
                                |p| (p.0.clone(), p.1, false)
                            ).collect(),
                            roomid: roomid.clone(),
                            stream_listener_cancel: cancel.clone(),
                            msg: vec![format!("Player {} exits game.", exit_name),
                                "Please press ENTER to get ready!".into()],
                        };
                        self.exitmenu.1 = 0;
                    }
                    Some(Msg::ExitRoom(ri)) => {
                        self.state = AppState::WaitPlayer {
                            players: rpc::room_info_to_players(&players[0].0, &ri),
                            client: client.clone(),
                            roomid: roomid.clone(),
                            msg: vec!["Someone exits room.".into(),
                                "Waiting for other players to join room......".into()],
                            stream_listener_cancel: cancel.clone(),
                        };
                        self.exitmenu.1 = 0;
                    }
                    _ => panic!("Got GameMsg other than Msg::Play in state Gaming!"),
                }
                true
            }
            AppState::GameResult {
                ref mut players, ref mut client, ref roomid,
                stream_listener_cancel: ref cancel, ..
            } => {
                match msg.msg {
                    Some(Msg::ExitGame(_)) => false,
                    Some(Msg::ExitRoom(ri)) => {
                        self.state = AppState::WaitPlayer {
                            players: rpc::room_info_to_players(&players[0].0, &ri),
                            client: client.clone(),
                            roomid: roomid.clone(),
                            msg: vec!["Someone exits room.".into(),
                                "Waiting for other players to join room......".into()],
                            stream_listener_cancel: cancel.clone(),
                        };
                        self.exitmenu.1 = 0;
                        true
                    }
                    _ => panic!("Got impossible GameMsg in state GameResult!"),
                }
            }
            _ => panic!("Shouldn't receive any stream msg in this state!"),
        }
    }

    fn parse_hold_result(
        hs: &Vec<HoldList>, players: &Vec<(String, usize, u32)>, holds: &Vec<Card>
    ) -> Vec<(String, usize, Vec<Card>)> {
        let mut ret = vec![("".into(), 0, Vec::new()); 4];
        for (i, (name, idx, h)) in players.iter().enumerate() {
            ret[i].0 = name.clone();
            ret[i].1 = *idx;
            // check hold num
            assert!(*h as usize == hs[*idx].holds.len());
            ret[i].2 = hs[*idx].holds.iter().map(|c| Card::from_info(c)).collect();
            ret[i].2.sort();
        }
        // check my holds
        assert!(holds.len() == ret[0].2.len());
        assert!(!ret[0].2.iter().zip(holds).any(|(a, b)| *a != *b));
        ret
    }

    fn parse_desk_result(ds: &DeskResult, players: &Vec<(String, usize, u32)>)
        -> Vec<Vec<(Card, usize)>> {

        let mut ret = Vec::new();
        for each in [&ds.spade, &ds.heart, &ds.club, &ds.diamond] {
            let mut chain: Vec<(Card, usize)> = each.iter().map(
                |cs| {
                    (Card::from_info(cs.card.as_ref().unwrap()),
                     players.iter().position(|p| p.1 == cs.whose as usize).unwrap())
                }
            ).collect();
            chain.sort_by(|a, b| b.cmp(a));
            ret.push(chain);
        }
        ret
    }

    async fn handle_exitmenu_event(&mut self, action: u32) -> bool {
        let button_num = match self.state {
            AppState::GetServer {..} | AppState::AskName {..}
            | AppState::JoinRoom {..} | AppState::NewRoom {..} => 2,
            AppState::WaitPlayer {..} | AppState::WaitReady {..} => 3,
            AppState::Gaming {..} | AppState::GameResult {..} => 4,
        };
        match action {
            0 => {
                match self.state {
                    AppState::GetServer {..} | AppState::AskName {..} | AppState::NewRoom {..} => {
                        match self.exitmenu.1 {
                            0 => {},
                            1 => self.cancel.cancel(),
                            _ => panic!("Invalid button num!"),
                        }
                    }
                    AppState::JoinRoom { stream_listener_cancel: ref cancel, ..} => {
                        // whether or not the stream listener is spawned, we just cancel
                        cancel.cancel();
                        match self.exitmenu.1 {
                            0 => {},
                            1 => self.cancel.cancel(),
                            _ => panic!("Invalid button num!"),
                        }
                    }
                    AppState::WaitPlayer {
                        client: ref mut c, stream_listener_cancel: ref cancel,
                        ref players, ref roomid, ..
                    } | AppState::WaitReady {
                        client: ref mut c, stream_listener_cancel: ref cancel,
                        ref players, ref roomid, ..
                    } => {
                        match self.exitmenu.1 {
                            0 => {}
                            1 => {
                                let _ = c.exit_room(players[0].1 as u32, roomid.clone()).await;
                                cancel.cancel();
                                self.state = AppState::AskName {
                                    client: c.clone(),
                                    input: Input::new(players[0].0.clone()),
                                    msg: "Exited room successfully.\n\
                                            Please enter your nickname:".into(),
                                    button: 0,
                                    is_input: true,
                                };
                                self.exitmenu.1 = 0;
                            },
                            2 => {
                                let _ = c.exit_room(players[0].1 as u32, roomid.clone()).await;
                                cancel.cancel();
                                self.cancel.cancel();
                            },
                            _ => panic!("Invalid button num!"),
                        }
                    }
                    AppState::Gaming {
                        client: ref mut c, stream_listener_cancel: ref cancel,
                        ref players, ref roomid, ..
                    } => {
                        match self.exitmenu.1 {
                            0 => {}
                            1 => {
                                c.exit_game(players[0].1 as u32, roomid.clone()).await.unwrap();
                                self.state = AppState::WaitReady {
                                    client: c.clone(),
                                    players: players.iter().map(
                                        |p| (p.0.clone(), p.1, false)
                                    ).collect(),
                                    roomid: roomid.clone(),
                                    stream_listener_cancel: cancel.clone(),
                                    msg: vec!["Please press ENTER to get ready!".into()],
                                };
                                self.exitmenu.1 = 0;
                            }
                            2 => {
                                let _ = c.exit_room(players[0].1 as u32, roomid.clone()).await;
                                cancel.cancel();
                                self.state = AppState::AskName {
                                    client: c.clone(),
                                    input: Input::new(players[0].0.clone()),
                                    msg: "Exited room successfully.\n\
                                            Please enter your nickname:".into(),
                                    button: 0,
                                    is_input: true,
                                };
                                self.exitmenu.1 = 0;
                            },
                            3 => {
                                let _ = c.exit_room(players[0].1 as u32, roomid.clone()).await;
                                cancel.cancel();
                                self.cancel.cancel();
                            },
                            _ => panic!("Invalid button num!"),
                        }
                    }
                    AppState::GameResult {
                        client: ref mut c, stream_listener_cancel: ref cancel,
                        ref players, ref roomid, ..
                    } => {
                        match self.exitmenu.1 {
                            0 => {}
                            1 => {
                                c.exit_game(players[0].1 as u32, roomid.clone()).await.unwrap();
                                self.state = AppState::WaitReady {
                                    client: c.clone(),
                                    players: players.iter().map(
                                        |p| (p.0.clone(), p.1, false)
                                    ).collect(),
                                    roomid: roomid.clone(),
                                    stream_listener_cancel: cancel.clone(),
                                    msg: vec!["Please press ENTER to get ready!".into()],
                                };
                                self.exitmenu.1 = 0;
                            }
                            2 => {
                                let _ = c.exit_room(players[0].1 as u32, roomid.clone()).await;
                                cancel.cancel();
                                self.state = AppState::AskName {
                                    client: c.clone(),
                                    input: Input::new(players[0].0.clone()),
                                    msg: "Exited room successfully.\n\
                                            Please enter your nickname:".into(),
                                    button: 0,
                                    is_input: true,
                                };
                                self.exitmenu.1 = 0;
                            },
                            3 => {
                                let _ = c.exit_room(players[0].1 as u32, roomid.clone()).await;
                                cancel.cancel();
                                self.cancel.cancel();
                            },
                            _ => panic!("Invalid button num!"),
                        }
                    }
                }
                self.exitmenu.0 = false;
            }
            1 => {
                self.exitmenu.1 += button_num - 1;
                self.exitmenu.1 %= button_num;
            }
            2 => {
                self.exitmenu.1 += 1;
                self.exitmenu.1 %= button_num;
            }
            _ => panic!("Invalid exitmenu action!"),
        }
        true
    }
}

// TODO:
// custom room name
// server clean room not alive per hour
// server close room for someone not alive for 1 min
// readme
