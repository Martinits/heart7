use tonic::{transport::Server, Request, Response, Status};
use heart7::{*, heart7_server::*};

pub mod heart7 {
    tonic::include_proto!("heart7");
}

#[derive(Debug, Default)]
pub struct Heart7D {}

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

}

#[tonic::async_trait]
impl Heart7 for Heart7D {
    async fn new_room(
        &self,
        request: Request<PlayerInfo>,
    ) -> Result<Response<RoomInfo>, Status> {

        println!("Got a request: {:?}", request);

        let reply = RoomInfo {
            roomid: format!("Hello!").into(),
            players: Vec::new(),
            state: RoomState::NotFull as i32,
        };

        Ok(Response::new(reply))
    }

    async fn join_room(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<RoomInfo>, Status> {

        println!("Got a request: {:?}", request);

        let reply = RoomInfo {
            roomid: format!("Hello!").into(),
            players: Vec::new(),
            state: RoomState::NotFull as i32,
        };

        Ok(Response::new(reply))
    }

    async fn room_status(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<RoomInfo>, Status> {

        println!("Got a request: {:?}", request);

        let reply = RoomInfo {
            roomid: format!("Hello!").into(),
            players: Vec::new(),
            state: RoomState::NotFull as i32,
        };

        Ok(Response::new(reply))
    }

    async fn game_ready(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<CommonReply>, Status> {

        println!("Got a request: {:?}", request);

        let reply = CommonReply {
            success: true,
            msg: "Ok".into(),
        };

        Ok(Response::new(reply))
    }

    async fn game_status(
        &self,
        request: Request<RoomReq>,
    ) -> Result<Response<GameInfo>, Status> {

        println!("Got a request: {:?}", request);

        let reply = GameInfo {
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
        };

        Ok(Response::new(reply))
    }

    async fn play_card(
        &self,
        request: Request<PlayReq>,
    ) -> Result<Response<CommonReply>, Status> {

        println!("Got a request: {:?}", request);

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

        println!("Got a request: {:?}", request);

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

        println!("Got a request: {:?}", request);

        let reply = CommonReply {
            success: true,
            msg: "Ok".into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "localhost:20007".parse()?;
    let server = Heart7D::default();

    Server::builder()
        .add_service(Heart7Server::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
