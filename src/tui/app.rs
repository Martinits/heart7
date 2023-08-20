use std::error::Error;
use crate::*;
use crate::client::rpc::{self, Client};
use super::ui;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use ratatui::backend::Backend;
use tui::tui::Tui;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;
use crossterm::event::{
    Event as CrosstermEvent,
    KeyEvent,
    KeyCode,
    KeyModifiers,
};

pub type AppResult<T> = Result<T, Box<dyn Error>>;

pub enum AppState {
    GetServer {
        input: Input,
        msg: String,
        connecting: bool,
    },
    GetRoom {
        client: Client,
        input: Input,
        msg: String,
        button: u16,
        is_input: bool,
    },
    JoinRoom {
        client: Client,
        input: Input,
        msg: String,
        name: String,
    },
    WaitPlayer {
        client: Client,
        players: Vec<(String, usize, bool)>,
        msg: String,
        roomid: String,
    },
    WaitReady {
        client: Client,
        players: Vec<(String, usize, bool)>,
        msg: String,
        roomid: String,
    },
    Gaming,
    GameResult,
    // ExitMenu(Box<Self>),
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
}

impl<B: Backend> App<B> {
    pub fn new(
        tui: Tui<B>,
        cancel: &CancellationToken,
        tx: mpsc::Sender<Action>,
        rx: mpsc::Receiver<Action>,
    ) -> Self {
        Self {
            tui,
            cancel: cancel.clone(),
            state: Default::default(),
            tx,
            rx,
        }
    }

    pub fn init(&mut self) -> AppResult<()> {
        self.tui.init(&self.cancel)?;
        Ok(())
    }

    pub async fn run(&mut self) -> AppResult<()> {

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
                        Some(a) => match a {
                            Action::Esc => self.handle_esc(),
                            Action::Enter => self.handle_enter().await,
                            Action::LeftArrow => self.handle_lr_arrow(true),
                            Action::RightArrow => self.handle_lr_arrow(false),
                            Action::UpArrow => self.handle_ud_arrow(true),
                            Action::DownArrow => self.handle_ud_arrow(false),
                            Action::Type(c) => self.handle_type(c),
                            Action::CtrlC => panic!("Got Ctrl-C!"),
                            Action::Resize(_, _) => true,
                            Action::Refresh => true,
                            Action::Backspace => self.handle_del(true),
                            Action::Delete => self.handle_del(false),
                            Action::ServerConnectResult(r)
                                => self.handle_server_connect_result(r),
                            Action::StreamMsg(msg)
                                => self.handle_stream_msg(msg).await,
                        }
                    }
                }
            }
        }

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
        self.tui.draw(|frame| ui::render(frame, &self.state))?;
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
            AppState::GetRoom {
                ref input, ref mut msg, button, client: ref mut c, is_input
            } if !is_input => {
                if button == 0{
                    // new room
                    info!("Player {} chooses to new room", input.value());
                    match c.new_room(input.value().into()).await {
                        Ok(roomid) => {
                            info!("Get NewRoom result from server, enter JoinRoom state");
                            self.state = AppState::JoinRoom {
                                client: c.clone(),
                                input: Input::new(roomid),
                                msg: format!("Hello, {}!\n\
                                        Successfully created a room, ID is shown below.\n\
                                        Please press ENTER to join room:", input.value()),
                                name: input.value().into(),
                            };
                        },
                        Err(s) => {
                            *msg = format!("Making NewRoom request to server failed:\n\
                                            {}\n\
                                            Please retry:", s);
                        }
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
                    }
                }
                true
            }
            AppState::JoinRoom {
                ref input, ref mut msg, client: ref mut c, ref name
            } => {
                info!("Joining room {}", input.value());
                match c.join_room(name.clone(), input.value().into()).await {
                    Ok(stream) => {
                        // spawn stream listerning task
                        info!("Spawning GameStream listener...");
                        Client::spawn_stream_listener(stream, &self.cancel, &self.tx);
                        info!("Querying RoomStatus...");
                        match c.room_status(input.value().into()).await {
                            Ok(rs) => match rs.state {
                                Some(State::NotFull(_)) => {
                                    info!("Join room {}, enter WaitPlayer state", input.value());
                                    self.state = AppState::WaitPlayer {
                                        players: rpc::room_info_to_players(name, &rs),
                                        client: c.clone(),
                                        roomid: input.value().into(),
                                        msg: "Waiting for other players to join room......".into(),
                                    };
                                }
                                Some(State::WaitReady(_)) => {
                                    info!("Join room {}, enter WaitReady state", input.value());
                                    self.state = AppState::WaitReady {
                                        players: rpc::room_info_to_players(name, &rs),
                                        client: c.clone(),
                                        roomid: input.value().into(),
                                        msg: "Please press ENTER to get ready!".into(),
                                    }
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
                ref mut client, ref mut players, ref roomid, ref mut msg
            } if !players[0].2 => {
                match client.game_ready(players[0].1 as u32, roomid.clone()).await {
                    Ok(_) => {
                        players[0].2 = true;
                        *msg = "Waiting for other players to get ready......".into();
                    }
                    Err(s) => panic!("Failed to GetReady: {}", s),
                }
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
            AppState::GetRoom {ref mut input, is_input, ..} if is_input => {
                input.handle_event(
                    &CrosstermEvent::Key(
                        KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)
                    )
                );
                true
            }
            AppState::JoinRoom {ref mut input, ..} => {
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
            AppState::GetRoom {ref mut input, is_input, ref mut button, ..} => {
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
            AppState::JoinRoom {ref mut input, ..} => {
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
            _ => {
                false
            }
        }
    }

    fn handle_ud_arrow(&mut self, _is_up: bool) -> bool {
        match self.state {
            AppState::GetRoom { ref mut is_input, ..} => {
                *is_input = !*is_input;
                true
            }
            _ => false
        }
    }

    fn handle_esc(&self) -> bool {
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
            AppState::GetRoom {ref mut input, is_input, ..} if is_input => {
                input.handle_event(
                    &CrosstermEvent::Key(KeyEvent::new(keycode, KeyModifiers::NONE))
                );
                true
            }
            AppState::JoinRoom {ref mut input, ..}=> {
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
                            self.state = AppState::GetRoom {
                                client: c,
                                input: Input::default(),
                                msg: "Game server connected.\n\
                                        Please enter your nickname:".into(),
                                button: 0,
                                is_input: true,
                            };
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
        match self.state {
            AppState::WaitPlayer {ref mut client, ref mut players, ref roomid, ..} => {
                match msg.msg {
                    Some(Msg::RoomInfo(ri)) => {
                        *players = rpc::room_info_to_players(&players[0].0, &ri);
                        if let Some(State::WaitReady(_)) =  ri.state {
                            info!("Stream got RoomInfo: WaitReady, enter WaitReady state");
                            self.state = AppState::WaitReady{
                                client: client.clone(),
                                players: players.clone(),
                                msg: "Please press ENTER to get ready!".into(),
                                roomid: roomid.clone(),
                            }
                        }
                    }
                    None => panic!("Got empty GameMsg!"),
                    _ => panic!("Got GameMsg other than RoomInfo in state WaitPlayer!"),
                }
                true
            }
            AppState::WaitReady {ref mut players, ..} => {
                match msg.msg {
                    Some(Msg::RoomInfo(ri)) => {
                        *players = rpc::room_info_to_players(&players[0].0, &ri);
                    }
                    Some(Msg::WhoReady(who)) => {
                        Self::someone_get_ready(players, who as usize);
                    }
                    Some(Msg::Start(next)) => {
                    }
                    None => panic!("Got empty GameMsg!"),
                    _ => panic!("Got GameMsg other than RoomInfo in state WaitPlayer!"),
                }
                true
            }
            _ => false
        }
    }
}
