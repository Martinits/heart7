use crate::client::rpc;
use super::*;
use tui_input::Input;

pub enum ExitMenuEvent {
    Enter,
    MoveUp,
    MoveDown,
}

impl Client {
    pub async fn handle_exitmenu_event(&mut self, e: ExitMenuEvent) -> bool {
        let button_num = match self.state {
            ClientState::GetServer {..} | ClientState::AskName {..}
            | ClientState::JoinRoom {..} | ClientState::NewRoom {..} => 2,
            ClientState::WaitPlayer {..} | ClientState::WaitReady {..} => 3,
            ClientState::Gaming {..} | ClientState::GameResult {..} => 4,
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
            ClientState::GetServer {..} | ClientState::AskName {..} | ClientState::NewRoom {..} => {
                match self.exitmenu.1 {
                    0 => {},
                    1 => self.cancel.cancel(),
                    _ => panic!("Invalid button num!"),
                }
            }
            ClientState::JoinRoom { stream_listener_cancel: ref cancel, ..} => {
                // whether or not the stream listener is spawned, we just cancel
                cancel.cancel();
                match self.exitmenu.1 {
                    0 => {},
                    1 => self.cancel.cancel(),
                    _ => panic!("Invalid button num!"),
                }
            }
            ClientState::WaitPlayer {
                client: ref mut c, stream_listener_cancel: ref cancel,
                ref players, ref roomid, ..
            } | ClientState::WaitReady {
                client: ref mut c, stream_listener_cancel: ref cancel,
                ref players, ref roomid, ..
            } => {
                match self.exitmenu.1 {
                    0 => {}
                    1 => {
                        let _ = c.exit_room(players[0].1, roomid.clone()).await;
                        cancel.cancel();
                        self.state = ClientState::AskName {
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
                        let _ = c.exit_room(players[0].1, roomid.clone()).await;
                        cancel.cancel();
                        self.cancel.cancel();
                    },
                    _ => panic!("Invalid button num!"),
                }
            }
            ClientState::Gaming {
                client: ref mut c, stream_listener_cancel: ref cancel,
                ref game, ref roomid, my_remote_idx, ..
            } => {
                match self.exitmenu.1 {
                    0 => {}
                    1 => {
                        c.exit_game(my_remote_idx, roomid.clone()).await.unwrap();
                        self.state = ClientState::WaitReady {
                            client: c.clone(),
                            players: game.get_player_names().into_iter().enumerate().map(
                                |(i, name)| (name, Self::get_remote_idx(my_remote_idx, i), false)
                            ).collect(),
                            roomid: roomid.clone(),
                            stream_listener_cancel: cancel.clone(),
                            msg: vec!["Please press ENTER to get ready!".into()],
                        };
                        self.exitmenu.1 = 0;
                    }
                    2 => {
                        let _ = c.exit_room(my_remote_idx, roomid.clone()).await;
                        cancel.cancel();
                        self.state = ClientState::AskName {
                            client: c.clone(),
                            input: Input::new(game.get_my_name()),
                            msg: "Exited room successfully.\n\
                                    Please enter your nickname:".into(),
                            button: 0,
                            is_input: true,
                        };
                        self.exitmenu.1 = 0;
                    },
                    3 => {
                        let _ = c.exit_room(my_remote_idx, roomid.clone()).await;
                        cancel.cancel();
                        self.cancel.cancel();
                    },
                    _ => panic!("Invalid button num!"),
                }
            }
            ClientState::GameResult {
                client: ref mut c, stream_listener_cancel: ref cancel,
                ref players, my_remote_idx, ref roomid, ..
            } => {
                match self.exitmenu.1 {
                    0 => {}
                    1 => {
                        c.exit_game(my_remote_idx, roomid.clone()).await.unwrap();
                        let ri = c.room_status(roomid.clone()).await.unwrap();
                        let ps = rpc::room_info_to_players(my_remote_idx, &ri);
                        assert!(!ps[0].2);
                        self.state = ClientState::WaitReady {
                            players: ps,
                            client: c.clone(),
                            roomid: roomid.clone(),
                            msg: vec!["Please press ENTER to get ready!".into()],
                            stream_listener_cancel: cancel.clone(),
                        };
                        self.exitmenu.1 = 0;
                    }
                    2 => {
                        let _ = c.exit_room(my_remote_idx, roomid.clone()).await;
                        cancel.cancel();
                        self.state = ClientState::AskName {
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
                        let _ = c.exit_room(my_remote_idx, roomid.clone()).await;
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
