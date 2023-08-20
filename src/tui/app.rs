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
use crate::game::Card;
use crate::client::desk::*;

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
    },
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
    Tab,
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
    pub async fn new(
        tui: Tui<B>,
        cancel: &CancellationToken,
        tx: mpsc::Sender<Action>,
        rx: mpsc::Receiver<Action>,
    ) -> Self {
        Self {
            tui,
            cancel: cancel.clone(),
            // state: Default::default(),
            state: AppState::Gaming{
                client: Client{
                    c: heart7_client::Heart7Client::connect("http://127.0.0.1:20007").await.unwrap(),
                    addr:"127.0.0.1:20007".into()
                },
                players: vec![
                    ("1".into(), 0, 0),
                    ("2".into(), 1, 0),
                    ("3".into(), 2, 0),
                    ("4".into(), 3, 0)
                ],
                next: 0,
                choose: 0,
                last: None,
                cards: Vec::new(),
                holds: Vec::new(),
                has_last: false,
                desk: Default::default(),
                roomid: "jbhfvhsbdfvhbkdsfhbv".into(),
                button: 0,
                play_cnt: 0
            },
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
                            Action::Tab => self.handle_tab(),
                            Action::ServerConnectResult(r)
                                => self.handle_server_connect_result(r),
                            Action::StreamMsg(msg)
                                => self.handle_stream_msg(msg).await,
                        }
                    }
                }
            }
        }

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
            AppState::Gaming {
                client: ref mut c, ref players, choose, ref mut cards,
                ref mut holds, ref roomid, ref button, ..
            } if cards.len() != 0 => {
                let play = match *button {
                        0 => Play::Discard(cards[choose].clone().into()),
                        _ => Play::Hold(cards[choose].clone().into())
                };
                match c.play_card(players[0].1 as u32, roomid.clone(), play).await {
                    Ok(_) => {
                        let c = cards.remove(choose);
                        if *button == 1 {
                            holds.push(c);
                            holds.sort();
                        }
                    },
                    Err(s) => panic!("Failed to play card to server: {}", s),
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
            AppState::Gaming {
                ref mut choose, ref cards, ..
            } if cards.len() != 0 => {
                if is_left {
                    if *choose > 1 {
                        *choose -= 1;
                    }
                } else {
                    *choose += 1;
                    if *choose > cards.len() {
                        *choose = cards.len();
                    }
                }
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

    fn handle_tab(&mut self) -> bool {
        match self.state {
            AppState::GetRoom {ref mut button, ..}=> {
                *button += 1;
                *button %= 2;
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

    fn someone_hold(players: &mut Vec<(String, usize, u32)>, pid: u32) {
        if let Some(p) = players.iter_mut().find(|p| p.1 == pid as usize) {
            p.2 += 1;
        } else {
            panic!("playerid in PlayInfo not exist!");
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
            AppState::WaitReady {ref mut client, ref mut players, ref roomid, ..} => {
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
                                    next: next as usize,
                                    last: None,
                                    cards,
                                    holds: Vec::new(),
                                    desk: Default::default(),
                                    choose: 0,
                                    button: 0,
                                    has_last: false,
                                    play_cnt: 0,
                                }
                            }
                            Err(s) => panic!("Failed to get GameStatus on start: {}", s),
                        }
                    }
                    None => panic!("Got empty GameMsg!"),
                    _ => panic!("Got GameMsg other than RoomInfo in state WaitPlayer!"),
                }
                true
            }
            AppState::Gaming {
                ref mut players, ref mut next, ref mut last, ref mut has_last,
                ref cards, ref mut desk, ref mut play_cnt, ..
            } if cards.len() != 0 => {
                match msg.msg {
                    Some(Msg::Play(PlayInfo { player: pid, playone })) => {
                        assert!(pid < 4);
                        if playone == None {
                            panic!("Empty PlayInfo in GameMsg Play!");
                        } else if let Some(po) = playone {
                            if po.play == None {
                                panic!("Empty PlayOne in GameMsg Play!");
                            } else if let Some(play) = po.play {
                                *play_cnt += 1;
                                match play {
                                    Play::Discard(ci) => {
                                        let c = Card::from_info(&ci);
                                        *last = Some(c.clone());
                                        desk.update(c.clone(), *play_cnt%4 == 1);
                                    }
                                    Play::Hold(ci) => {
                                        assert!(ci.num == 0 && ci.suit == 0);
                                        *last = None;
                                        Self::someone_hold(players, pid);
                                    }
                                }
                                *next += 1;
                                *next %= 4;
                                *has_last = true;
                            }
                        }
                    }
                    _ => panic!("Got GameMsg other than Msg::Play in state Gaming!"),
                }
                true
            }
            _ => false
        }
    }
}

// client workflow
// new room
// join room -> stream
// room status -> draw first
// listen stream and draw
// get a roominfo: new player join in, if all 4 join in, display ready state
// get a whoready: someone get ready
// get a start: server start game, and client should rpc GameStatus to get cards
// continue listen stream
//
// handle when someone exits
//
// handle Esc of all states
