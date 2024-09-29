use crate::client::rpc;
use super::*;
use tui_input::Input;

pub enum ExitMenuEvent {
    Enter,
    MoveUp,
    MoveDown,
}

impl App {
    pub async fn handle_exitmenu_event(&mut self, e: ExitMenuEvent) -> bool {
        let button_num = match self.state {
            AppState::GetServer {..} | AppState::AskName {..}
            | AppState::JoinRoom {..} | AppState::NewRoom {..} => 2,
            AppState::WaitPlayer {..} | AppState::WaitReady {..} => 3,
            AppState::Gaming {..} | AppState::GameResult {..} => 4,
        };
        match e {
            ExitMenuEvent::Enter => self.handle_exitmenu_enter().await,
            ExitMenuEvent::MoveUp => {
                self.exitmenu.1 += button_num - 1;
                self.exitmenu.1 %= button_num;
            }
            ExitMenuEvent::MoveDown => {
                self.exitmenu.1 += 1;
                self.exitmenu.1 %= button_num;
            }
        }
        true
    }

    pub async fn handle_exitmenu_enter(&mut self) {
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
                        let ri = c.room_status(roomid.clone()).await.unwrap();
                        let ps = rpc::room_info_to_players(players[0].1, &ri);
                        assert!(!ps[0].2);
                        self.state = AppState::WaitReady {
                            players: ps,
                            client: c.clone(),
                            roomid: roomid.clone(),
                            msg: vec!["Please press ENTER to get ready!".into()],
                            stream_listener_cancel: cancel.clone(),
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
}
