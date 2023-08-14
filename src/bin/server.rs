use tokio_stream::wrappers::WatchStream;
use heart7::room::RoomManager;
use heart7::{*, heart7_server::*};
use std::error::Error;

#[derive(Debug, Default)]
pub struct Heart7D {
    rm: RoomManager,
}

#[tonic::async_trait]
impl Heart7 for Heart7D {
    async fn new_room(
        &self,
        request: Request<PlayerInfo>,
    ) -> Result<Response<NewRoomReply>, Status> {

        debug!("Got NewRoom request: {:?}", request);

        let aroom = self.rm.new_room().await?;
        let room = aroom.read().await;

        Ok(Response::new(NewRoomReply{
            roomid: room.get_id()
        }))
    }

    type JoinRoomStream = WatchStream<Result<GameMsg, Status>>;

    async fn join_room(
        &self,
        request: Request<JoinRoomReq>,
    ) -> Result<Response<Self::JoinRoomStream>, Status> {

        debug!("Got JoinRoom request: {:?}", request);

        let aroom = self.rm.get_room(&request.get_ref().roomid).await?;
        let mut room = aroom.write().await;

        let player = request.get_ref().player.as_ref().ok_or(
            Status::new(
                Code::InvalidArgument,
                "Empty PlayerInfo!"
            )
        )?;

        if 3 == room.add_player(&player)? {
            let ar = aroom.clone();
            tokio::spawn(async move {
                let room = ar.read().await;
                let msg = GameMsg {
                    msg: Some(Msg::RoomInfo(room.get_room_info().unwrap()))
                };
                room.send_gamemsg(msg);
            });
        }

        Ok(Response::new(WatchStream::new(room.get_gamemsg_rx()?)))
    }

    async fn room_status(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<RoomInfo>, Status> {

        debug!("Got RoomStatus request: {:?}", request);

        let aroom = self.rm.get_room(&request.get_ref().roomid).await?;
        let room_info = aroom.write().await.get_room_info()?;

        Ok(Response::new(room_info))
    }

    async fn game_ready(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<GameReadyReply>, Status> {

        debug!("Got GameReady request: {:?}", request);

        let aroom = self.rm.get_room(&request.get_ref().roomid).await?;
        let mut room = aroom.write().await;

        let left = room.player_ready(request.get_ref().playerid as usize)?;

        {
            let ar = aroom.clone();
            tokio::spawn(async move {
                let room = ar.read().await;
                let msg = GameMsg {
                    msg: Some(Msg::WhoReady(request.get_ref().playerid))
                };
                room.send_gamemsg(msg);
            });
        }

        if left == 0 {
            let ar = aroom.clone();
            tokio::spawn(async move {
                let mut room = ar.write().await;
                room.start_game();
            });
        }

        Ok(Response::new(GameReadyReply { left }))
    }

    type GameMessageStream = WatchStream<Result<GameMsg, Status>>;

    async fn game_message(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<Self::GameMessageStream>, Status> {

        debug!("Got GameMessage request: {:?}", request);

        let aroom = self.rm.get_room(&request.get_ref().roomid).await?;

        let rx = aroom.read().await.get_gamemsg_rx()?;

        Ok(Response::new(WatchStream::new(rx)))
    }

    async fn game_status(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<GameInfo>, Status> {

        debug!("Got GameStatus request: {:?}", request);

        let aroom = self.rm.get_room(&request.get_ref().roomid).await?;

        let room = aroom.read().await;

        Ok(Response::new(room.get_game_info(request.get_ref().playerid)?))
    }

    async fn play_card(
        &self,
        request: Request<PlayReq>,
    ) -> Result<Response<CommonReply>, Status> {

        debug!("Got PlayCard request: {:?}", request);

        let roomreq = &request.get_ref().roomreq.as_ref().ok_or(
            Status::new(
                Code::InvalidArgument,
                "Empty RoomReq!"
            )
        )?;

        let playone = request.get_ref().playone.as_ref().ok_or(
            Status::new(
                Code::InvalidArgument,
                "Empty PlayOne"
            )
        )?;
        let play = playone.play.as_ref().ok_or(
            Status::new(
                Code::InvalidArgument,
                "Empty PlayOne"
            )
        )?;

        let aroom = self.rm.get_room(&roomreq.roomid).await?;
        let mut room = aroom.write().await;

        room.play_card(roomreq.playerid, play)?;

        {
            let ar = aroom.clone();
            let pid = roomreq.playerid;
            let pone = playone.clone();
            tokio::spawn(async move {
                let room = ar.read().await;
                let playinfo = PlayInfo{
                    player: pid,
                    playone: Some(pone),
                };
                let msg = GameMsg {
                    msg: Some(Msg::Play(playinfo)),
                };
                room.send_gamemsg(msg);
            });
        }


        let reply = CommonReply {
            success: true,
            msg: "Ok".into(),
        };

        Ok(Response::new(reply))
    }

    async fn exit_game(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<CommonReply>, Status> {

        debug!("Got a request: {:?}", request);

        let reply = CommonReply {
            success: true,
            msg: "Ok".into(),
        };

        Ok(Response::new(reply))
    }

    async fn exit_room(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<CommonReply>, Status> {

        debug!("Got a request: {:?}", request);

        let reply = CommonReply {
            success: true,
            msg: "Ok".into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let sock_addr = format!("0.0.0.0:{}", DEFAULT_PORT).parse()?;
    let server = Heart7D::default();

    Server::builder()
        .add_service(Heart7Server::new(server))
        .serve(sock_addr)
        .await?;

    Ok(())
}
