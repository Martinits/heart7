use crate::*;
use crate::client::rpc::{self, Client as RpcClient};
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
                        let gi = client.game_status(players[0].1 as u32, roomid.clone())
                                    .await.unwrap_or_else(
                                        |s| panic!("Failed to get GameStatus on start: {}", s)
                                    );
                        let mut cards: Vec<Card> = gi.cards.iter().map(
                            |c| Card::from_info(c)
                        ).collect();
                        cards.sort();

                        self.state = ClientState::Gaming{
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
                ref mut players, ref mut next, ref mut last, ref mut has_last,
                ref mut desk, ref mut play_cnt, ref mut client, ref roomid, ref holds,
                stream_listener_cancel: ref cancel, ..
            } => {
                match msg.msg {
                    Some(Msg::Play(PlayInfo { player: pid, playone })) => {
                        assert!(pid < 4);
                        assert!(players[*next].1 == pid as usize);

                        let play = playone.expect("Empty PlayInfo in GameMsg Play!")
                                    .play.expect("Empty PlayOne in GameMsg Play!");
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
                    Some(Msg::Endgame(GameResult { desk, hold })) => {
                        let ds = desk.expect("Empty DeskResult in GameResult from server!");
                        // actually it should be already sorted
                        // holds.sort();
                        self.state = ClientState::GameResult{
                            ds: Self::parse_desk_result(&ds, players),
                            players: Self::parse_hold_result(&hold, players, holds),
                            client: client.clone(),
                            roomid: roomid.clone(),
                            stream_listener_cancel: cancel.clone(),
                        };
                        self.exitmenu.1 = 0;
                    }
                    Some(Msg::ExitGame(who)) => {
                        let exit_name = players.iter().find(|p| p.1 == who as usize).unwrap().0.clone();
                        self.state = ClientState::WaitReady {
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
        hs: &Vec<HoldList>, players: &Vec<(String, usize, u32)>, holds: &Vec<Card>
    ) -> Vec<(String, usize, Vec<Card>)> {
        let mut ret = vec![("".into(), 0, Vec::new()); 4];
        for (i, (name, idx, h)) in players.iter().enumerate() {
            ret[i].0 = name.clone();
            ret[i].1 = *idx;
            // check hold num
            assert!(*h as usize == hs[*idx].holds.len());
            ret[i].2 = hs[*idx].holds.iter().map(|c| Card::from_info(c)).collect();
            // ret[i].2.sort();
        }
        // check my holds
        assert!(holds.len() == ret[0].2.len());
        // assert!(!ret[0].2.iter().zip(holds).any(|(a, b)| *a != *b));
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
}
