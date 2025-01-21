mod room;

use tokio_stream::wrappers::ReceiverStream;
use room::RoomManager;
use crate::{*, heart7_server::*};

#[derive(Debug, Default)]
pub struct Heart7D {
    rm: RoomManager,
}

impl Heart7D {
    pub fn spawn_watch_dog(&self) {
        self.rm.spawn_watch_dog();
    }
}

type RPCResult<T> = Result<T, tonic::Status>;

#[tonic::async_trait]
impl Heart7 for Heart7D {
    async fn new_room(
        &self,
        request: Request<NewRoomReq>,
    ) -> Result<Response<CommonReply>, Status> {

        info!("Got NewRoom request: {:?}", request);

        let _ = self.rm.new_room(&request.get_ref().roomid).await?;

        let reply = CommonReply {
            success: true,
            msg: "Ok".into(),
        };

        info!("NewRoom response: {:?}", reply);
        Ok(Response::new(reply))
    }

    type JoinRoomStream = ReceiverStream<Result<GameMsg, Status>>;

    async fn join_room(
        &self,
        request: Request<JoinRoomReq>,
    ) -> Result<Response<Self::JoinRoomStream>, Status> {

        info!("Got JoinRoom request: {:?}", request);

        let aroom = self.rm.get_room(&request.get_ref().roomid).await?;
        let mut room = aroom.write().await;

        let player = request.get_ref().player.as_ref().ok_or(
            Status::new(
                Code::InvalidArgument,
                "Empty PlayerInfo!"
            )
        )?;

        if player.name.len() == 0 {
            return Err(Status::new(
                Code::InvalidArgument,
                "Empty name!"
            ))
        }

        let rx = room.add_player(&player)?;

        {
            let ar = aroom.clone();
            tokio::spawn(async move {
                let room = ar.read().await;
                let msg = Msg::RoomInfo(room.get_room_info().unwrap());
                info!("Sending GameMsg: {:?}", msg);
                room.send_gamemsg(msg).await;
            });
        }

        info!("JoinRoom response: ReceiverStream");
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn room_status(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<RoomInfo>, Status> {

        info!("Got RoomStatus request: {:?}", request);

        let aroom = self.rm.get_room(&request.get_ref().roomid).await?;
        let room_info = aroom.write().await.get_room_info()?;

        info!("RoomStatus response: {:?}", room_info);
        Ok(Response::new(room_info))
    }

    async fn game_ready(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<GameReadyReply>, Status> {

        info!("Got GameReady request: {:?}", request);

        let aroom = self.rm.get_room(&request.get_ref().roomid).await?;
        let mut room = aroom.write().await;

        let left = room.player_ready(request.get_ref().playerid as usize)?;

        {
            let ar = aroom.clone();
            tokio::spawn(async move {
                let room = ar.read().await;
                let msg = Msg::WhoReady(request.get_ref().playerid);
                info!("Sending GameMsg: {:?}", msg);
                room.send_gamemsg(msg).await;
            });
        }

        if left == 0 {
            let ar = aroom.clone();
            tokio::spawn(async move {
                let mut room = ar.write().await;
                room.start_game().await;
            });
        }

        let reply = GameReadyReply{ left };

        info!("GameReady response: {:?}", reply);
        Ok(Response::new(reply))
    }

    // type GameMessageStream = WatchStream<Result<GameMsg, Status>>;
    //
    // async fn game_message(
    //     &self,
    //     request: Request<RoomReq>,
    // ) -> Result<Response<Self::GameMessageStream>, Status> {
    //
    //     info!("Got GameMessage request: {:?}", request);
    //
    //     let aroom = self.rm.get_room(&request.get_ref().roomid).await?;
    //
    //     let rx = aroom.read().await.get_gamemsg_rx()?;
    //
    //     info!("GameMessage response: WatchStream");
    //     Ok(Response::new(WatchStream::new(rx)))
    // }

    async fn game_status(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<GameInfo>, Status> {

        info!("Got GameStatus request: {:?}", request);

        let aroom = self.rm.get_room(&request.get_ref().roomid).await?;

        let room = aroom.read().await;

        let reply = room.get_game_info(request.get_ref().playerid)?;

        info!("GameStatus response: {:?}", reply);
        Ok(Response::new(reply))
    }

    async fn play_card(
        &self,
        request: Request<PlayReq>,
    ) -> Result<Response<CommonReply>, Status> {

        info!("Got PlayCard request: {:?}", request);

        let roomreq = &request.get_ref().roomreq.as_ref().ok_or(
            Status::new(
                Code::InvalidArgument,
                "Empty RoomReq!"
            )
        )?;

        let playone = request.get_ref().playone.as_ref().ok_or(
            Status::new(
                Code::InvalidArgument,
                "Empty PlayCard"
            )
        )?;

        let aroom = self.rm.get_room(&roomreq.roomid).await?;
        let mut room = aroom.write().await;

        let mut pi = PlayInfo {
            player: roomreq.playerid,
            playone: Some(playone.clone()),
        };
        let who = roomreq.playerid as usize;
        let endgame = room.play_card(pi.clone().into())?;

        if playone.is_discard {
            let msg = Msg::Play(pi);
            info!("Sending GameMsg: {:?}", msg);
            room.send_gamemsg(msg).await;
        } else {
            let msg = Msg::Play(pi.clone());
            info!("Sending GameMsg to {} only: {:?}", who, msg);
            room.send_gamemsg_to(msg, who).await;

            pi.playone.as_mut().unwrap().card = Some(DUMMY_CARD.clone().into());
            let msg = Msg::Play(pi);
            info!("Sending GameMsg except {}: {:?}", who, msg);
            room.send_gamemsg_except(msg, who).await;
        }

        if endgame {
            let res = room.end_game()?;
            let ar = aroom.clone();
            tokio::spawn(async move {
                let room = ar.read().await;
                let msg = Msg::Endgame(res);
                info!("Sending GameMsg: {:?}", msg);
                room.send_gamemsg(msg).await;
            });
        }

        let reply = CommonReply {
            success: true,
            msg: "Ok".into(),
        };

        info!("PlayCard response: {:?}", reply);
        Ok(Response::new(reply))
    }

    async fn exit_game(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<CommonReply>, Status> {

        info!("Got ExitGame request: {:?}", request);

        let aroom = self.rm.get_room(&request.get_ref().roomid).await?;
        let mut room = aroom.write().await;

        room.exit_game(request.get_ref().playerid as usize).await?;

        let reply = CommonReply {
            success: true,
            msg: "Ok".into(),
        };

        info!("ExitGame response: {:?}", reply);
        Ok(Response::new(reply))
    }

    async fn exit_room(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<CommonReply>, Status> {

        info!("Got ExitRoom request: {:?}", request);

        let left_ones = {
            let aroom = self.rm.get_room(&request.get_ref().roomid).await?;
            let mut room = aroom.write().await;
            let left_ones = room.exit_room(request.get_ref().playerid as usize)?;
            if left_ones != 0 {
                let ar = aroom.clone();
                tokio::spawn(async move {
                    let room = ar.read().await;
                    let msg = Msg::ExitRoom(room.get_room_info().unwrap());
                    info!("Sending GameMsg: {:?}", msg);
                    room.send_gamemsg(msg).await;
                });
            }
            left_ones
        };

        if left_ones == 0 {
            self.rm.del_room(&request.get_ref().roomid).await?;
        }

        let reply = CommonReply {
            success: true,
            msg: "Ok".into(),
        };

        info!("ExitRoom response: {:?}", reply);
        Ok(Response::new(reply))
    }
}

