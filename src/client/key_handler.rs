use crate::*;
use crate::client::rpc::{self, RpcClient as RpcClient};
use super::*;
use tokio_util::sync::CancellationToken;
use crossterm::event::{
    Event as CrosstermEvent,
    KeyEvent,
    KeyCode,
    KeyModifiers,
};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

impl Client {
    pub async fn handle_enter(&mut self) -> bool {
        match self.state {
            ClientState::GetServer {
                ref mut input, ref mut msg, ref mut connecting
            } if !*connecting && input.value().len() > 0 => {
                // connect to server
                RpcClient::connect_spawn(input.value(), &self.tx);
                *connecting = true;
                *msg = format!("Try connecting to {} ......", input.value());
                true
            }
            ClientState::AskName {
                ref input, button, client: ref mut c, is_input, ..
            } if !is_input && input.value().len() > 0 => {
                if button == 0{
                    // new room
                    info!("Player {} chooses to new room", input.value());
                    self.state = ClientState::NewRoom {
                        client: c.clone(),
                        input: Input::default(),
                        name: input.value().into(),
                        msg: format!("Hello, {}!\n\
                                Please enter new room name:", input.value()),
                    }
                } else {
                    //join room
                    info!("Player {} chooses to join room, enter JoinRoom state", input.value());
                    let scancel = CancellationToken::new();
                    self.state = ClientState::JoinRoom {
                        name: input.value().into(),
                        input: Input::default(),
                        client: c.clone(),
                        msg: format!("Hello, {}!\n\
                                Please enter room ID:", input.value()),
                        stream_listener_cancel: scancel.clone(),
                    };
                    add_cancel_to_panic(scancel);
                    self.exitmenu.1 = 0;
                }
                true
            }
            ClientState::NewRoom {
                client: ref mut c, ref input, ref name, ref mut msg
            } if input.value().len() > 0 => {
                match c.new_room(input.value().into()).await {
                    Ok(()) => {
                        info!("Get NewRoom result from server, enter JoinRoom state");
                        let scancel = CancellationToken::new();
                        self.state = ClientState::JoinRoom {
                            client: c.clone(),
                            input: Input::new(input.value().into()),
                            msg: format!("Hello, {}!\n\
                                    Successfully created a room, ID is shown below.\n\
                                    Please press ENTER to join room:", name),
                            name: name.clone(),
                            stream_listener_cancel: scancel.clone(),
                        };
                        add_cancel_to_panic(scancel);
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
            ClientState::JoinRoom {
                ref input, ref mut msg, client: ref mut c, ref name,
                stream_listener_cancel: ref cancel
            } if input.value().len() > 0 => {
                info!("Joining room {}", input.value());
                match c.join_room(name.clone(), input.value().into()).await {
                    Ok(stream) => {
                        // spawn stream listerning task
                        info!("Spawning GameStream listener...");
                        RpcClient::spawn_stream_listener(stream, cancel, &self.tx);
                        self.state = ClientState::WaitPlayer {
                            players: vec![("".into(), 4, false); 4],
                            client: c.clone(),
                            roomid: input.value().into(),
                            msg: vec!["Waiting for other players to join room......".into()],
                            stream_listener_cancel: cancel.clone(),
                        };
                        self.exitmenu.1 = 0;
                    }
                    Err(s) => {
                        *msg = format!("Making JoinRoom request to server failed:\n\
                                        {}\n\
                                        Please retry:", s);
                    }
                }
                true
            }
            ClientState::WaitReady {
                ref mut client, ref mut players, ref roomid, ref mut msg, ..
            } if !players[0].2 => {
                let _ = client.game_ready(players[0].1, roomid.clone()).await
                        .unwrap_or_else(|s| panic!("Failed to GetReady: {}", s));
                players[0].2 = true;
                *msg = vec!["Waiting for other players to get ready......".into()];
                true
            }
            ClientState::Gaming {
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
            ClientState::GameResult {
                ref mut client, ref roomid, my_remote_idx,
                stream_listener_cancel: ref cancel, ..
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
                self.state = ClientState::WaitReady {
                    players: ps,
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

    pub fn handle_typing(&mut self, c: char) -> bool {
        match self.state {
            ClientState::GetServer {ref mut input, connecting, ..} if !connecting => {
                input.handle_event(
                    &CrosstermEvent::Key(
                        KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)
                    )
                );
                true
            }
            ClientState::AskName {ref mut input, is_input, ..} if is_input => {
                input.handle_event(
                    &CrosstermEvent::Key(
                        KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)
                    )
                );
                true
            }
            ClientState::NewRoom {ref mut input, ..} | ClientState::JoinRoom {ref mut input, ..} => {
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

    pub fn handle_lr_arrow(&mut self, is_left: bool) -> bool {
        match self.state {
            ClientState::GetServer {ref mut input, connecting, ..} if !connecting => {
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
            ClientState::AskName {ref mut input, is_input, ref mut button, ..} => {
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
            ClientState::NewRoom {ref mut input, ..} | ClientState::JoinRoom {ref mut input, ..} => {
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
            ClientState::Gaming {ref mut choose, ref game, ..} => {
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
            ClientState::AskName { ref mut is_input, ..} => {
                *is_input = !*is_input;
                true
            }
            ClientState::Gaming {
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
            KeyCode::Backspace
        } else {
            KeyCode::Delete
        };
        match self.state {
            ClientState::GetServer {ref mut input, connecting, ..} if !connecting => {
                input.handle_event(
                    &CrosstermEvent::Key(KeyEvent::new(keycode, KeyModifiers::NONE))
                );
                true
            }
            ClientState::AskName {ref mut input, is_input, ..} if is_input => {
                input.handle_event(
                    &CrosstermEvent::Key(KeyEvent::new(keycode, KeyModifiers::NONE))
                );
                true
            }
            ClientState::NewRoom {ref mut input, ..} | ClientState::JoinRoom {ref mut input, ..} => {
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

}
