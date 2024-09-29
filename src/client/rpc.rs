use crate::{*, heart7_client::*};
use tonic::transport::Channel;
use tokio::sync::mpsc;
use crate::client::ClientEvent;
use std::net::Ipv4Addr;
use tonic::codec::Streaming;
use tokio_util::sync::CancellationToken;
use anyhow::Result;

#[derive(Clone)]
pub struct Client {
    pub c: Heart7Client<Channel>,
    pub addr: String,
}

pub type GameStream = Streaming<GameMsg>;

impl Client {
    pub fn connect_spawn(addr: &str, tx: &mpsc::Sender<ClientEvent>) {
        let txc = tx.clone();
        let addr = addr.to_string();
        tokio::spawn(async move {
            let (ip, port): (String, String) = match addr.find(':') {
                Some(i) => (addr[0..i].into(), addr[i+1..].into()),
                None => ("".into(), "".into())
            };
            txc.send(ClientEvent::ServerConnectResult(
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
            )).await.expect("Send Action::ServerConnectResult to client");
        });
    }

    pub fn get_addr(&self) -> String {
        self.addr.clone()
    }

    pub async fn new_room(&mut self, name: String) -> Result<()> {
        let request = Request::new(NewRoomReq {
            roomid: name
        });

        let r = self.c.new_room(request).await?.into_inner();
        if r.success {
            assert!(r.msg == "Ok");
            Ok(())
        } else {
            Err(Status::new(
                Code::Internal,
                format!("Server response false when new room, {}", r.msg)
            ).into())
        }
    }

    pub async fn join_room(&mut self, name: String, roomid: String) -> Result<GameStream> {
        let request = Request::new(JoinRoomReq{
            player: Some(PlayerInfo { name }),
            roomid
        });

        Ok(self.c.join_room(request).await?.into_inner())
    }

    pub fn spawn_stream_listener(
        mut stream: GameStream,
        cancel: &CancellationToken,
        tx: &mpsc::Sender<ClientEvent>)
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
                            Ok(None) => {
                                info!("GameStream closed! Stream listener exits!");
                                break;
                            }
                            Ok(Some(msg)) => txc.send(ClientEvent::StreamMsg(msg)).await
                                .expect("Send Action::StreamMsg to client"),
                        }
                    }
                }
            }
        });
    }

    pub async fn room_status(&mut self, roomid: String) -> Result<RoomInfo> {
        let request = Request::new(RoomReq{
            playerid: 0,
            roomid
        });

        Ok(self.c.room_status(request).await?.into_inner())
    }

    pub async fn game_ready(&mut self, pid: u32, roomid: String) -> Result<GameReadyReply> {
        let request = Request::new(RoomReq{
            playerid: pid,
            roomid
        });

        Ok(self.c.game_ready(request).await?.into_inner())
    }

    pub async fn game_status(&mut self, pid: u32, roomid: String) -> Result<GameInfo> {
        let request = Request::new(RoomReq{
            playerid: pid,
            roomid
        });

        Ok(self.c.game_status(request).await?.into_inner())
    }

    pub async fn play_card(&mut self, pid: u32, roomid: String, play: Play) -> Result<()> {
        let roomreq = RoomReq{
            playerid: pid,
            roomid
        };
        let request = Request::new(PlayReq{
            roomreq: Some(roomreq),
            playone: Some(PlayOne{
                play: Some(play)
            })
        });

        let r = self.c.play_card(request).await?.into_inner();
        if r.success {
            assert!(r.msg == "Ok");
            Ok(())
        } else {
            Err(Status::new(
                Code::Internal,
                format!("Server response false when playing card, {}", r.msg)
            ).into())
        }
    }

    pub async fn exit_game(&mut self, pid: u32, roomid: String) -> Result<()> {
        let request = Request::new(RoomReq{
            playerid: pid,
            roomid
        });

        let r = self.c.exit_game(request).await?.into_inner();
        if r.success {
            assert!(r.msg == "Ok");
            Ok(())
        } else {
            Err(Status::new(
                Code::Internal,
                format!("Server response false when exit game, {}", r.msg)
            ).into())
        }
    }

    pub async fn exit_room(&mut self, pid: u32, roomid: String) -> Result<()> {
        let request = Request::new(RoomReq{
            playerid: pid,
            roomid
        });

        let r = self.c.exit_room(request).await?.into_inner();
        if r.success {
            assert!(r.msg == "Ok");
            Ok(())
        } else {
            Err(Status::new(
                Code::Internal,
                format!("Server response false when exit room, {}", r.msg)
            ).into())
        }
    }
}

pub fn room_info_to_players(myidx: usize, ri: &RoomInfo) -> Vec<(String, usize, bool)> {
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
    players.rotate_left(myidx);
    players
}
