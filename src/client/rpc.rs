use crate::{*, heart7_client::*};
use tonic::transport::Channel;
use tokio::sync::mpsc;
use tui::app::Action;
use std::net::Ipv4Addr;
use crate::tui::app::AppResult;
use tonic::codec::Streaming;
use tokio_util::sync::CancellationToken;

#[derive(Clone)]
pub struct Client {
    c: Heart7Client<Channel>,
    addr: String,
}

pub type GameStream = Streaming<GameMsg>;

impl Client {
    pub fn connect_spawn(addr: &str, tx: &mpsc::Sender<Action>) {
        let txc = tx.clone();
        let addr = addr.to_string();
        tokio::spawn(async move {
            let (ip, port): (String, String) = match addr.find(':') {
                Some(i) => (addr[0..i].into(), addr[i+1..].into()),
                None => ("".into(), "".into())
            };
            txc.send(Action::ServerConnectResult(
                if ip.len() == 0 || port.len() == 0 {
                    Err("Invalid ip or port!".into())
                } else if !ip.parse::<Ipv4Addr>().is_ok() {
                    Err("Invalid ip address!".into())
                } else {
                    let url =format!("http://{}", &addr);
                    match Heart7Client::connect(url).await {
                        Ok(c) => Ok(Client{ c, addr }),
                        Err(e) => Err(e.to_string()),
                    }
                }
            )).await.expect("Send Action::ServerConnectResult to app");
        });
    }

    pub fn get_addr(&self) -> String {
        self.addr.clone()
    }

    pub async fn new_room(&mut self, name: String) -> AppResult<String> {
        let request = Request::new(PlayerInfo {
            name
        });

        Ok(self.c.new_room(request).await?.into_inner().roomid)
    }

    pub async fn join_room(&mut self, name: String, roomid: String) -> AppResult<GameStream> {
        let request = Request::new(JoinRoomReq{
            player: Some(PlayerInfo { name }),
            roomid
        });

        Ok(self.c.join_room(request).await?.into_inner())
    }

    pub fn spawn_stream_listener(
        mut stream: GameStream,
        cancel: &CancellationToken,
        tx: &mpsc::Sender<Action>)
    {
        let txc = tx.clone();
        let cancel = cancel.clone();
        tokio::spawn(async move {
            loop {
                tokio::select!{
                    _ = cancel.cancelled() => {
                        break;
                    }
                    maybe_msg = stream.message() => {
                        match maybe_msg {
                            Err(s) => panic!("GameStream error: {}", s),
                            Ok(None) => panic!("GameStream closed!"),
                            Ok(Some(msg)) => txc.send(Action::StreamMsg(msg)).await
                                .expect("Send Action::StreamMsg to app")
                        }
                    }
                }
            }
        });
    }

    pub async fn room_status(&mut self, roomid: String) -> AppResult<RoomInfo> {
        let request = Request::new(RoomReq{
            playerid: 0,
            roomid
        });

        Ok(self.c.room_status(request).await?.into_inner())
    }

    pub async fn game_ready(&mut self, pid: u32, roomid: String) -> AppResult<GameReadyReply> {
        let request = Request::new(RoomReq{
            playerid: pid,
            roomid
        });

        Ok(self.c.game_ready(request).await?.into_inner())
    }
}

// used for WaitPlayers and WaitReady state
pub fn room_info_to_players(myname: &String, ri: &RoomInfo) -> Vec<(String, usize, bool)> {
    let mut players = vec![("".into(), 0, false); 4];
    for i in 0..ri.players.len() {
        players[i].0 = ri.players[i].name.clone();
        players[i].1 = i;
    }
    if let Some(State::WaitReady(ref rl)) = ri.state {
        for i in &rl.l {
            players[*i as usize].2 = true;
        }
    }
    if let Some(idx) = ri.players.iter().position(|p| p.name == *myname) {
        players.rotate_left(idx)
    } else {
        panic!("Cannot find myself in RoomInfo!");
    }
    players
}
