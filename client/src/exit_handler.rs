use crate::*;

pub enum ExitMenuEvent {
    Enter,
    MoveUp,
    MoveDown,
}

impl From<ClientEvent> for ExitMenuEvent {
    fn from(value: ClientEvent) -> Self {
        match value {
            ClientEvent::Enter => ExitMenuEvent::Enter,
            ClientEvent::UpArrow => ExitMenuEvent::MoveUp,
            ClientEvent::DownArrow => ExitMenuEvent::MoveDown,
            _ => unreachable!(),
        }
    }
}

impl ClientStateManager {
    // return (cancel_stream_listener, full_exit)
    pub async fn handle_exitmenu_event(&mut self, e: ExitMenuEvent) -> (bool, bool) {
        let button_num = match self.state {
            ClientStateInternal::GetServer {..} | ClientStateInternal::AskName {..}
            | ClientStateInternal::JoinRoom {..} | ClientStateInternal::NewRoom {..} => 2,
            ClientStateInternal::WaitPlayer {..} | ClientStateInternal::WaitReady {..} => 3,
            ClientStateInternal::Gaming {..} | ClientStateInternal::GameResult {..} => 4,
        };
        let mut cancel_stream = false;
        let mut full_exit = false;
        match e {
            ExitMenuEvent::Enter =>
             (cancel_stream, full_exit) = self.handle_exitmenu_enter().await,
            ExitMenuEvent::MoveUp => {
                self.exitmenu.1 += button_num - 1;
                self.exitmenu.1 %= button_num;
            }
            ExitMenuEvent::MoveDown => {
                self.exitmenu.1 += 1;
                self.exitmenu.1 %= button_num;
            }
        }
        (cancel_stream, full_exit)
    }

    // return (cancel_stream_listener, full_exit)
    pub async fn handle_exitmenu_enter(&mut self) -> (bool, bool) {
        self.exitmenu.0 = false;
        match self.state {
            ClientStateInternal::GetServer {..}
            | ClientStateInternal::AskName {..}
            | ClientStateInternal::NewRoom {..}
            | ClientStateInternal::JoinRoom {..} => {
                match self.exitmenu.1 {
                    0 => {},
                    // in JoinRoom State, we need to cancel stream listener,
                    // but full_exit implies cancelling the stream listener
                    1 => return (false, true),
                    _ => panic!("Invalid button num!"),
                }
            }
            ClientStateInternal::WaitPlayer {
                client: ref mut c, ref players, ref roomid, ..
            } | ClientStateInternal::WaitReady {
                client: ref mut c, ref players, ref roomid, ..
            } => {
                match self.exitmenu.1 {
                    0 => {}
                    1 => {
                        let _ = c.exit_room(players[0].1, roomid.clone()).await;
                        self.state = ClientStateInternal::AskName {
                            client: c.clone(),
                            input: Input::new(players[0].0.clone()),
                            msg: "Exited room successfully.\n\
                                    Please enter your nickname:".into(),
                            button: 0,
                            is_input: true,
                        };
                        self.exitmenu.1 = 0;
                        return (true, false)
                    },
                    2 => {
                        let _ = c.exit_room(players[0].1, roomid.clone()).await;
                        return (true, true)
                    },
                    _ => panic!("Invalid button num!"),
                }
            }
            ClientStateInternal::Gaming {
                client: ref mut c, ref game, ref roomid, my_remote_idx, ..
            } => {
                match self.exitmenu.1 {
                    0 => {}
                    1 => {
                        c.exit_game(my_remote_idx, roomid.clone()).await.unwrap();
                        self.state = ClientStateInternal::WaitReady {
                            client: c.clone(),
                            players: game.get_player_names().into_iter().enumerate().map(
                                |(i, name)| (name, Self::get_remote_idx(my_remote_idx, i), false)
                            ).collect(),
                            roomid: roomid.clone(),
                            msg: vec!["Please press the button to get ready!".into()],
                        };
                        self.exitmenu.1 = 0;
                    }
                    2 => {
                        let _ = c.exit_room(my_remote_idx, roomid.clone()).await;
                        self.state = ClientStateInternal::AskName {
                            client: c.clone(),
                            input: Input::new(game.get_my_name()),
                            msg: "Exited room successfully.\n\
                                    Please enter your nickname:".into(),
                            button: 0,
                            is_input: true,
                        };
                        self.exitmenu.1 = 0;
                        return (true, false)
                    },
                    3 => {
                        let _ = c.exit_room(my_remote_idx, roomid.clone()).await;
                        return (true, true)
                    },
                    _ => panic!("Invalid button num!"),
                }
            }
            ClientStateInternal::GameResult {
                client: ref mut c, ref players, my_remote_idx, ref roomid, ..
            } => {
                match self.exitmenu.1 {
                    0 => {}
                    1 => {
                        c.exit_game(my_remote_idx, roomid.clone()).await.unwrap();
                        let ri = c.room_status(roomid.clone()).await.unwrap();
                        let ps = rpc::room_info_to_players(my_remote_idx, &ri);
                        assert!(!ps[0].2);
                        self.state = ClientStateInternal::WaitReady {
                            players: ps,
                            client: c.clone(),
                            roomid: roomid.clone(),
                            msg: vec!["Please press the button to get ready!".into()],
                        };
                        self.exitmenu.1 = 0;
                    }
                    2 => {
                        let _ = c.exit_room(my_remote_idx, roomid.clone()).await;
                        self.state = ClientStateInternal::AskName {
                            client: c.clone(),
                            input: Input::new(players[0].0.clone()),
                            msg: "Exited room successfully.\n\
                                    Please enter your nickname:".into(),
                            button: 0,
                            is_input: true,
                        };
                        self.exitmenu.1 = 0;
                        return (true, false)
                    },
                    3 => {
                        let _ = c.exit_room(my_remote_idx, roomid.clone()).await;
                        return (true, true)
                    },
                    _ => panic!("Invalid button num!"),
                }
            }
        }
        (false, false)
    }
}
