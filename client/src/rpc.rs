use crate::*;
use tonic::codec::Streaming;

#[cfg(not(target_arch = "wasm32"))]
use tonic::transport::Channel;
#[cfg(target_arch = "wasm32")]
use tonic_web_wasm_client::Client;

#[derive(Clone, Debug)]
pub struct RpcClient {
    #[cfg(not(target_arch = "wasm32"))]
    pub c: Heart7Client<Channel>,
    #[cfg(target_arch = "wasm32")]
    pub c: Heart7Client<Client>,
    pub addr: String,
}

pub type GameStream = Streaming<GameMsg>;

impl RpcClient {
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn new(c: Heart7Client<Channel>, addr: String) -> RPCResult<Self> {
        let mut rpcclient = Self { c, addr };
        rpcclient.hello().await?;
        Ok(rpcclient)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn new(c: Heart7Client<Client>, addr: String) -> RPCResult<Self> {
        let mut rpcclient = Self { c, addr };
        rpcclient.hello().await?;
        Ok(rpcclient)
    }

    pub fn get_addr(&self) -> String {
        self.addr.clone()
    }

    pub async fn hello(&mut self) -> RPCResult<()> {
        let r = self.c.hello(EmptyRequest{}).await?.into_inner();
        if r.success {
            assert!(r.msg == "Hello!");
            Ok(())
        } else {
            Err(Status::new(
                Code::Internal,
                format!("Server response false when hello, {}", r.msg)
            ).into())
        }
    }

    pub async fn new_room(&mut self, name: String) -> RPCResult<()> {
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

    pub async fn join_room(&mut self, name: String, roomid: String) -> RPCResult<GameStream> {
        let request = Request::new(JoinRoomReq{
            player: Some(PlayerInfo { name }),
            roomid
        });

        Ok(self.c.join_room(request).await?.into_inner())
    }

    pub async fn room_status(&mut self, roomid: String) -> RPCResult<RoomInfo> {
        let request = Request::new(RoomReq{
            playerid: 0,
            roomid
        });

        Ok(self.c.room_status(request).await?.into_inner())
    }

    pub async fn game_ready(&mut self, pid: usize, roomid: String) -> RPCResult<GameReadyReply> {
        let request = Request::new(RoomReq{
            playerid: pid as u32,
            roomid
        });

        Ok(self.c.game_ready(request).await?.into_inner())
    }

    pub async fn game_status(&mut self, pid: usize, roomid: String) -> RPCResult<GameInfo> {
        let request = Request::new(RoomReq{
            playerid: pid as u32,
            roomid
        });

        Ok(self.c.game_status(request).await?.into_inner())
    }

    pub async fn play_card(&mut self, pid: usize, roomid: String, playone: PlayOne) -> RPCResult<()> {
        let roomreq = RoomReq{
            playerid: pid as u32,
            roomid
        };
        let request = Request::new(PlayReq{
            roomreq: Some(roomreq),
            playone: Some(playone),
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

    pub async fn exit_game(&mut self, pid: usize, roomid: String) -> RPCResult<()> {
        let request = Request::new(RoomReq{
            playerid: pid as u32,
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

    pub async fn exit_room(&mut self, pid: usize, roomid: String) -> RPCResult<()> {
        let request = Request::new(RoomReq{
            playerid: pid as u32,
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

pub fn room_info_to_players(my_remote_idx: usize, ri: &RoomInfo) -> Vec<(String, usize, bool)> {
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
    players.rotate_left(my_remote_idx);
    players
}
