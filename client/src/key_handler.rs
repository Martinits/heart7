use crate::*;

impl ClientStateManager {
    // return (redraw, spawn_rpc_client, spawn_stream_listener)
    pub async fn handle_enter(&mut self) -> (bool, Option<String>, Option<GameStream>) {
        let mut spawn_rpc_client = None;
        let mut spawn_stream_listener = None;
        let redraw = match self.state {
            ClientStateInternal::GetServer {
                ref mut input, ref mut msg, ref mut connecting
            } if !*connecting && input.value().len() > 0 => {
                // connect to server
                spawn_rpc_client = Some(input.value().to_string());
                *connecting = true;
                *msg = format!("Try connecting to {} ......", input.value());
                true
            }
            ClientStateInternal::AskName {
                ref input, button, client: ref mut c, is_input, ..
            } if !is_input && input.value().len() > 0 => {
                if button == 0{
                    // new room
                    info!("Player {} chooses to new room", input.value());
                    self.state = ClientStateInternal::NewRoom {
                        client: c.clone(),
                        input: Input::default(),
                        name: input.value().into(),
                        msg: format!("Hello, {}!\n\
                                Please enter new room name:", input.value()),
                    }
                } else {
                    //join room
                    info!("Player {} chooses to join room, enter JoinRoom state", input.value());
                    self.state = ClientStateInternal::JoinRoom {
                        name: input.value().into(),
                        input: Input::default(),
                        client: c.clone(),
                        msg: format!("Hello, {}!\n\
                                Please enter room ID:", input.value()),
                    };
                    self.exitmenu.1 = 0;
                }
                true
            }
            ClientStateInternal::NewRoom {
                client: ref mut c, ref input, ref name, ref mut msg
            } if input.value().len() > 0 => {
                match c.new_room(input.value().into()).await {
                    Ok(()) => {
                        info!("Get NewRoom result from server, enter JoinRoom state");
                        self.state = ClientStateInternal::JoinRoom {
                            client: c.clone(),
                            input: Input::new(input.value().into()),
                            msg: format!("Hello, {}!\n\
                                    Successfully created a room, ID is shown below.\n\
                                    Please press the button to join room:", name),
                            name: name.clone(),
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
            ClientStateInternal::JoinRoom {
                ref input, ref mut msg, client: ref mut c, ref name,
            } if input.value().len() > 0 => {
                info!("Joining room {}", input.value());
                match c.join_room(name.clone(), input.value().into()).await {
                    Ok(stream) => {
                        // spawn stream listerning task
                        spawn_stream_listener = Some(stream);
                    }
                    Err(s) => {
                        *msg = format!("Making JoinRoom request to server failed:\n\
                                        {}\n\
                                        Please retry:", s);
                    }
                }
                true
            }
            ClientStateInternal::WaitReady {
                ref mut client, ref mut players, ref roomid, ref mut msg, ..
            } if !players[0].2 => {
                let _ = client.game_ready(players[0].1, roomid.clone()).await
                        .unwrap_or_else(|s| panic!("Failed to GetReady: {}", s));
                players[0].2 = true;
                *msg = vec!["Waiting for other players to get ready......".into()];
                true
            }
            ClientStateInternal::Gaming {
                client: ref mut c, ref mut choose, ref mut game, my_remote_idx,
                ref roomid, ref button, ref mut msg, ..
            } if game.do_i_have_cards() && *choose != 0 && game.is_my_turn() => {
                let cards = game.get_my_cards();
                let playone = PlayOne {
                    is_discard: *button == 0,
                    card: Some(cards[*choose-1].clone().into()),
                };
                match c.play_card(my_remote_idx, roomid.clone(), playone).await {
                    Ok(_) => {
                        *choose = 0;
                        *msg = None;
                    },
                    Err(s) => {
                        if s.code() == Code::PermissionDenied {
                            *msg = Some(s.message().into());
                        } else {
                            panic!("Failed to play card to server");
                        }
                    }
                }
                true
            }
            ClientStateInternal::GameResult {
                ref mut client, ref roomid, my_remote_idx, ..
            } => {
                info!("Confirmed GameResult, enter WaitReady state");
                let _ = client.exit_game(my_remote_idx, roomid.clone()).await
                        .unwrap_or_else(|s| panic!("Failed to ExitGame in GameResult: {}", s));
                let ri = client.room_status(roomid.clone()).await
                            .unwrap_or_else(|s|
                                panic!("Failed to get RoomStatus in switching to WaitReady: {}", s)
                            );
                let ps = rpc::room_info_to_players(my_remote_idx, &ri);
                assert!(!ps[0].2);
                self.state = ClientStateInternal::WaitReady {
                    players: ps,
                    client: client.clone(),
                    roomid: roomid.clone(),
                    msg: vec!["Please press ENTER to get ready!".into()],
                };
                self.exitmenu.1 = 0;
                true
            }
            _ => {
                false
            }
        };
        (redraw, spawn_rpc_client, spawn_stream_listener)
    }

    pub fn handle_typing(&mut self, c: char) -> bool {
        match self.state {
            ClientStateInternal::GetServer {ref mut input, connecting, ..} if !connecting => {
                input.handle(InputRequest::InsertChar(c));
                true
            }
            ClientStateInternal::AskName {ref mut input, is_input, ..} if is_input => {
                input.handle(InputRequest::InsertChar(c));
                true
            }
            ClientStateInternal::NewRoom {ref mut input, ..}
            | ClientStateInternal::JoinRoom {ref mut input, ..} => {
                input.handle(InputRequest::InsertChar(c));
                true
            }
            _ => {
                false
            }
        }
    }

    pub fn handle_lr_arrow(&mut self, is_left: bool) -> bool {
        let req = if is_left {
            InputRequest::GoToPrevChar
        } else {
            InputRequest::GoToNextChar
        };
        match self.state {
            ClientStateInternal::GetServer {ref mut input, connecting, ..} if !connecting => {
                input.handle(req);
                true
            }
            ClientStateInternal::AskName {ref mut input, is_input, ref mut button, ..} => {
                if is_input {
                    input.handle(req);
                } else {
                    *button += 1;
                    *button %= 2;
                }
                true
            }
            ClientStateInternal::NewRoom {ref mut input, ..}
            | ClientStateInternal::JoinRoom {ref mut input, ..} => {
                input.handle(req);
                true
            }
            ClientStateInternal::Gaming {ref mut choose, ref game, ..} => {
                let cn = game.get_my_card_num();
                if cn != 0 {
                    if is_left {
                        *choose += cn + 1 - 1;
                    } else {
                        *choose += 1;
                    }
                    *choose %= cn + 1;
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
                } else {
                    false
                }
            }
            _ => {
                false
            }
        }
    }

    pub fn handle_ud_arrow(&mut self, _is_up: bool) -> bool {
        match self.state {
            ClientStateInternal::AskName { ref mut is_input, ..} => {
                *is_input = !*is_input;
                true
            }
            ClientStateInternal::Gaming {
                ref mut button, ref game, ..
            } if game.is_my_turn() => {
                *button += 1;
                *button %= 2;
                true
            }
            _ => false
        }
    }

    pub fn handle_esc(&mut self) -> bool {
        self.exitmenu.0 = !self.exitmenu.0;
        true
    }

    pub fn handle_del(&mut self, is_back: bool) -> bool {
        let keycode = if is_back {
            InputRequest::DeletePrevChar
        } else {
            InputRequest::DeleteNextChar
        };
        match self.state {
            ClientStateInternal::GetServer {ref mut input, connecting, ..} if !connecting => {
                input.handle(keycode);
                true
            }
            ClientStateInternal::AskName {ref mut input, is_input, ..} if is_input => {
                input.handle(keycode);
                true
            }
            ClientStateInternal::NewRoom {ref mut input, ..}
            | ClientStateInternal::JoinRoom {ref mut input, ..} => {
                input.handle(keycode);
                true
            }
            _ => false,
        }
    }

    pub fn handle_reset_input(&mut self, new_input: String) -> bool {
        match self.state {
            ClientStateInternal::GetServer { ref mut input, .. }
            | ClientStateInternal::AskName { ref mut input, .. }
            | ClientStateInternal::NewRoom { ref mut input, .. }
            | ClientStateInternal::JoinRoom { ref mut input, .. } => {
                *input = Input::new(new_input);
                true
            }
            _ => false,
        }
    }
}
