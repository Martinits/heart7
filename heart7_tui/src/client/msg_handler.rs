use crate::*;
use crate::client::rpc::{self, RpcClient};
use super::*;
use tui_input::Input;

impl Client {
    pub fn handle_server_connect_result(&mut self, r: Result<RpcClient, String>) -> bool {
        match self.state {
            ClientState::GetServer {ref mut input, ref mut msg, ref mut connecting} => {
                if !*connecting {
                    warn!("Client is not connecting, drop server connecting result!");
                    return false
                }
                match r {
                    Ok(c) => {
                        info!("Server {} Connected, enter GetRoom state", c.get_addr());
                        self.state = ClientState::AskName {
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
                        *input = Input::new(self.default_addr.clone()).with_cursor(0);
                        *msg = format!("Connecting to server failed:\n\
                                        {}\n\
                                        Please retry:", s);
                        *connecting = false;
                    }
                }
                true
            }
            _ => false
        }
    }

    pub async fn handle_stream_msg(&mut self, msg: GameMsg) -> bool {
        debug!("Got GameMsg: {:?}", msg);
        match self.state {
            ClientState::WaitPlayer {
                ref mut client, ref mut players, ref roomid, ref stream_listener_cancel, ..
            } => {
                match msg.msg {
                    Some(Msg::RoomInfo(ri)) => {
                        *players = rpc::room_info_to_players(msg.your_id as usize, &ri);
                        if let Some(State::WaitReady(_)) =  ri.state {
                            info!("Stream got RoomInfo: WaitReady, enter WaitReady state");
                            self.state = ClientState::WaitReady{
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
                        *players = rpc::room_info_to_players(msg.your_id as usize, &ri);
                    }
                    None => panic!("Got empty GameMsg!"),
                    _ => panic!("Got GameMsg not possible in state WaitPlayer!"),
                }
                true
            }
            ClientState::WaitReady {
                ref mut client, ref mut players, ref roomid, ref stream_listener_cancel, ..
            } => {
                match msg.msg {
                    Some(Msg::RoomInfo(ri)) => {
                        *players = rpc::room_info_to_players(msg.your_id as usize, &ri);
                    }
                    Some(Msg::WhoReady(who)) => {
                        Self::someone_get_ready(players, who as usize);
                    }
                    Some(Msg::Start(next)) => {
                        let gi = client.game_status(players[0].1, roomid.clone())
                                    .await.unwrap_or_else(
                                        |s| panic!("Failed to get GameStatus on start: {}", s)
                                    );
                        let cards: Vec<Card> = gi.cards.iter().map(
                            |c| c.into()
                        ).collect();

                        let mut game = Game::default();
                        players.iter().for_each(
                            |p| game.add_player(p.0.clone())
                        );
                        game.set_next(players.iter().position(|p| p.1 == next as usize).unwrap());
                        game.init_my_cards(cards);

                        self.state = ClientState::Gaming{
                            client: client.clone(),
                            roomid: roomid.clone(),
                            game,
                            my_remote_idx: players[0].1,
                            choose: 0,
                            button: 0,
                            msg: None,
                            stream_listener_cancel: stream_listener_cancel.clone(),
                        };
                        self.exitmenu.1 = 0;
                    }
                    Some(Msg::ExitRoom(ri)) => {
                        self.state = ClientState::WaitPlayer {
                            players: rpc::room_info_to_players(msg.your_id as usize, &ri),
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
                    Some(Msg::LoseConnection(ri)) => {
                        self.state = ClientState::WaitPlayer {
                            players: rpc::room_info_to_players(msg.your_id as usize, &ri),
                            client: client.clone(),
                            roomid: roomid.clone(),
                            msg: vec!["Someone lost connection...".into(),
                                "Waiting for other players to join room......".into()],
                            stream_listener_cancel: stream_listener_cancel.clone(),
                        };
                        self.exitmenu.1 = 0;
                    }
                    None => panic!("Got empty GameMsg!"),
                    _ => panic!("Got GameMsg not possible in state WaitReady!"),
                }
                true
            }
            ClientState::Gaming {
                ref mut client, ref roomid, ref mut game, my_remote_idx,
                stream_listener_cancel: ref cancel, ..
            } => {
                match msg.msg {
                    Some(Msg::Play(mut pi)) => {
                        pi.player = Self::get_local_idx(my_remote_idx, pi.player as usize) as u32;
                        game.play_card_no_check(pi.into()).unwrap();
                    }
                    Some(Msg::Endgame(GameEnding { desk, hold })) => {
                        let ds = desk.expect("Empty DeskResult in GameResult from server!");
                        // actually it should be already sorted
                        // holds.sort();
                        self.state = ClientState::GameResult{
                            ds: Self::parse_desk_result(&ds, my_remote_idx),
                            players: Self::parse_hold_result(
                                &hold, game.get_player_names(), my_remote_idx
                            ),
                            my_remote_idx,
                            client: client.clone(),
                            roomid: roomid.clone(),
                            stream_listener_cancel: cancel.clone(),
                        };
                        self.exitmenu.1 = 0;
                    }
                    Some(Msg::ExitGame(who)) => {
                        let exit_name = game.get_player_name(
                            Self::get_local_idx(my_remote_idx, who as usize)
                        );
                        self.state = ClientState::WaitReady {
                            client: client.clone(),
                            players: game.get_player_names().into_iter().enumerate().map(
                                |(i, name)| (name, Self::get_remote_idx(my_remote_idx, i), false)
                            ).collect(),
                            roomid: roomid.clone(),
                            stream_listener_cancel: cancel.clone(),
                            msg: vec![format!("Player {} exits game.", exit_name),
                                "Please press ENTER to get ready!".into()],
                        };
                        self.exitmenu.1 = 0;
                    }
                    Some(Msg::ExitRoom(ri)) => {
                        self.state = ClientState::WaitPlayer {
                            players: rpc::room_info_to_players(msg.your_id as usize, &ri),
                            client: client.clone(),
                            roomid: roomid.clone(),
                            msg: vec!["Someone exits room.".into(),
                                "Waiting for other players to join room......".into()],
                            stream_listener_cancel: cancel.clone(),
                        };
                        self.exitmenu.1 = 0;
                    }
                    Some(Msg::LoseConnection(ri)) => {
                        self.state = ClientState::WaitPlayer {
                            players: rpc::room_info_to_players(msg.your_id as usize, &ri),
                            client: client.clone(),
                            roomid: roomid.clone(),
                            msg: vec!["Someone lost connection...".into(),
                                "Waiting for other players to join room......".into()],
                            stream_listener_cancel: cancel.clone(),
                        };
                        self.exitmenu.1 = 0;
                    }
                    _ => panic!("Got GameMsg other than Msg::Play in state Gaming!"),
                }
                true
            }
            ClientState::GameResult {
                ref mut client, ref roomid,
                stream_listener_cancel: ref cancel, ..
            } => {
                match msg.msg {
                    Some(Msg::ExitGame(_)) => false,
                    Some(Msg::ExitRoom(ri)) => {
                        self.state = ClientState::WaitPlayer {
                            players: rpc::room_info_to_players(msg.your_id as usize, &ri),
                            client: client.clone(),
                            roomid: roomid.clone(),
                            msg: vec!["Someone exits room.".into(),
                                "Waiting for other players to join room......".into()],
                            stream_listener_cancel: cancel.clone(),
                        };
                        self.exitmenu.1 = 0;
                        true
                    }
                    Some(Msg::LoseConnection(ri)) => {
                        self.state = ClientState::WaitPlayer {
                            players: rpc::room_info_to_players(msg.your_id as usize, &ri),
                            client: client.clone(),
                            roomid: roomid.clone(),
                            msg: vec!["Someone lost connection...".into(),
                                "Waiting for other players to join room......".into()],
                            stream_listener_cancel: cancel.clone(),
                        };
                        self.exitmenu.1 = 0;
                        true
                    }
                    Some(Msg::WhoReady(_)) => {
                        info!("Stream got WhoReady in GameResult, drop");
                        true
                    }
                    _ => panic!("Got impossible GameMsg in state GameResult!"),
                }
            }
            _ => panic!("Shouldn't receive any stream msg in this state!"),
        }
    }

    fn someone_get_ready(players: &mut Vec<(String, usize, bool)>, who: usize) {
        if let Some(p) = players.iter_mut().find(|p| p.1 == who) {
            p.2 = true;
        } else {
            panic!("Player ID {} doesn't exists!", who);
        }
    }


    fn parse_hold_result(
        hs: &Vec<HoldList>, names: Vec<String>, my_remote_idx: usize
    ) -> Vec<(String, Vec<Card>)> {
        let mut ret: Vec<(String, Vec<Card>)> = vec![Default::default(); 4];
        for (local_idx, name) in names.into_iter().enumerate() {
            ret[local_idx].0 = name;
            ret[local_idx].1 = hs[Self::get_remote_idx(my_remote_idx, local_idx)]
                                .holds.iter().map(|c| c.into()).collect();
        }
        ret
    }

    fn get_local_idx(my_remote_idx: usize, remote_idx: usize) -> usize {
        (remote_idx + 4 - my_remote_idx) % 4
    }

    pub fn get_remote_idx(my_remote_idx: usize, local_idx: usize) -> usize {
        (local_idx + my_remote_idx) % 4
    }

    fn parse_desk_result(
        ds: &DeskResult, my_remote_idx: usize
    ) -> Vec<Vec<(Card, usize)>> {
        let mut ret = Vec::new();
        for each in [&ds.spade, &ds.heart, &ds.club, &ds.diamond] {
            let mut chain: Vec<(Card, usize)> = each.iter().map(
                |cs| {
                    (cs.card.as_ref().unwrap().into(),
                        Self::get_local_idx(my_remote_idx, cs.whose as usize))
                }
            ).collect();
            chain.sort_by(|a, b| b.cmp(a));
            ret.push(chain);
        }
        ret
    }
}
