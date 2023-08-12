use tokio::sync::mpsc;
use tonic::Code;
use tokio_stream::wrappers::ReceiverStream;
use heart7::room::RoomManager;
use heart7::{*, heart7_server::*};
use log::{debug, error, info};
use std::error::Error;

#[derive(Debug, Default)]
pub struct Heart7D {
    rm: RoomManager,
}

impl Heart7D {
    fn new_card(&self) -> Card {
        Card {
            suit: CardSuit::Spade as i32,
            num: 1,
        }
    }

    fn new_chain(&self) -> Chain {
        Chain {
            null: true,
            head: Some(self.new_card()),
            tail: Some(self.new_card()),
            head_thisround: 0,
            tail_thisround: 0,
        }
    }

    fn new_game_info(&self) -> GameInfo {
        GameInfo {
            state: GameState::Notready as i32,
            ready: Vec::new(),
            cards: Vec::new(),
            waitfor: 0,
            desk: Some(Desk {
                spade: Some(self.new_chain()),
                heart: Some(self.new_chain()),
                club: Some(self.new_chain()),
                diamond: Some(self.new_chain()),
            }),
            held: Some(HeldCards {
                my: Vec::new(),
                others: Vec::new(),
            }),
        }
    }

    fn new_msg_play(&self) -> MsgPlay {
        MsgPlay {
            player: 0,
            play: Some(PlayOne {
                discard_or_hold: true,
                card: Some(self.new_card()),
            })
        }
    }

    fn new_game_msg(&self) -> GameMsg {
        GameMsg {
            r#type: GameMsgType::Play as i32,
            msg: Some(Msg::Play(self.new_msg_play())),
        }
    }
}

#[tonic::async_trait]
impl Heart7 for Heart7D {
    async fn new_room(
        &self,
        request: Request<PlayerInfo>,
    ) -> Result<Response<RoomInfo>, Status> {

        debug!("Got NewRoom request: {:?}", request);

        let aroom = self.rm.new_room().await?;
        let mut room = aroom.write().await;

        room.add_player(request.get_ref())?;

        let room_info = room.get_room_info()?;

        Ok(Response::new(room_info))
    }

    async fn join_room(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<RoomInfo>, Status> {

        debug!("Got JoinRoom request: {:?}", request);

        let player = request.get_ref().player.as_ref().ok_or(
            Status::new(
                Code::InvalidArgument,
                "Empty PlayerInfo!"
            )
        )?;

        let aroom = self.rm.get_room(&request.get_ref().roomid).await?;
        let mut room = aroom.write().await;

        room.add_player(player)?;

        let room_info = room.get_room_info()?;

        Ok(Response::new(room_info))
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

    type GameReadyStream = ReceiverStream<Result<GameMsg, Status>>;

    async fn game_ready(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<Self::GameReadyStream>, Status> {

        debug!("Got a request: {:?}", request);

        let (tx, rx) = mpsc::channel(4);

        let gmsg = self.new_game_msg();

        tokio::spawn(async move {
            for _ in 0..100 {
                tx.send(Ok(gmsg.clone())).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type GameMessageStream = ReceiverStream<Result<GameMsg, Status>>;

    async fn game_message(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<Self::GameMessageStream>, Status> {

        debug!("Got a request: {:?}", request);

        let (tx, rx) = mpsc::channel(4);

        let gmsg = self.new_game_msg();

        tokio::spawn(async move {
            for _ in 0..100 {
                tx.send(Ok(gmsg.clone())).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn game_status(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<GameInfo>, Status> {

        debug!("Got a request: {:?}", request);

        Ok(Response::new(self.new_game_info()))
    }

    async fn play_card(
        &self,
        request: Request<PlayReq>,
    ) -> Result<Response<CommonReply>, Status> {

        debug!("Got a request: {:?}", request);

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
