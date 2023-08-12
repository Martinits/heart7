use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use tokio::sync::watch::{self, Sender, Receiver};
use crate::{*, game::*};

type ARoom = Arc<RwLock<Room>>;
type MsgTX = Sender<Result<GameMsg, Status>>;
pub type MsgRX = Receiver<Result<GameMsg, Status>>;

#[derive(Debug, Default)]
pub struct RoomManager {
    rooms: Arc<RwLock<HashMap<String, ARoom>>>,
}

#[derive(Debug, Default)]
pub struct Room {
    players: Vec<Player>,
    state: RoomState,
    id: String,
    gamemsg_tx: Option<MsgTX>,
    gamemsg_rx: Option<MsgRX>,
    ready_cnt: u32,
}

#[derive(Debug, Default, Clone)]
struct Player {
    name: String,
    // gamemsg_rx: Option<Receiver<Result<GameMsg, Status>>>,
    game: Game,
}

impl RoomManager {
    pub async fn new_room(&self) -> RPCResult<ARoom> {
        let id = uuid::Uuid::new_v4().to_string();

        let hellomsg = GameMsg {
            msg: Some(Msg::Hello("Hello".into()))
        };

        let (tx, rx) = watch::channel(Ok(hellomsg));

        let r = Room {
            state: RoomState::NotFull,
            players: Vec::new(),
            id: id.clone(),
            gamemsg_tx: Some(tx),
            gamemsg_rx: Some(rx),
            ready_cnt: 0,
        };

        let mut rooms = self.rooms.write().await;

        if let Some(_) = rooms.get(&id) {
            Err(Status::new(
                Code::AlreadyExists,
                format!("Room {} already exists!", id),
            ))
        } else {
            let ar = Arc::new(RwLock::new(r));
            rooms.insert(id.clone(), ar.clone());
            Ok(ar)
        }

    }

    pub async fn get_room(&self, id: &String) -> RPCResult<ARoom> {
        if let Some(ar) = self.rooms.read().await.get(id) {
            Ok(ar.clone())
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("Room {} not found !", id),
            ))
        }
    }

    pub async fn del_room(&self, id: &String) -> RPCResult<()> {
        if let Some(_) = self.rooms.write().await.remove(id) {
            Ok(())
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("Room {} not found !", id),
            ))
        }
    }
}

// must hold Room lock before calling
impl Room {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_room_info(&self) -> RPCResult<RoomInfo> {
        Ok(RoomInfo {
            roomid: self.id.clone(),
            players: self.players.iter().map(
                |p| PlayerInfo{ name: p.name.clone() }
            ).collect(),
            state: self.state as i32,
        })
    }

    pub fn add_player(&mut self, p: &PlayerInfo) -> RPCResult<usize> {
        if self.state != RoomState::NotFull {
            return Err(Status::new(
                Code::ResourceExhausted,
                format!("Room {} is full!", &self.id)
            ));
        }

        self.players.push(Player {
            name: p.name.clone(),
            game: Default::default(),
        });

        if self.players.len() == 4 {
            self.state = RoomState::WaitReady;
        }

        Ok(self.players.len() - 1)
    }

    pub fn player_ready(&mut self, pid: usize) -> RPCResult<u32> {
        if self.state != RoomState::WaitReady {
            return Err(Status::new(
                Code::PermissionDenied,
                format!("Room {} is not full or game has begun!", &self.id)
            ))
        }

        if let Some(p) = self.players.get_mut(pid) {
            p.game.ready()?;
            self.ready_cnt += 1;
            Ok(4 - self.ready_cnt)
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("Player {} not exist!", pid),
            ))
        }
    }

    pub fn start_game(&mut self) -> RPCResult<()> {
        Ok(())
    }

    pub fn get_gamemsg_rx(&self) -> RPCResult<MsgRX> {
        if let Some(ref rx) = self.gamemsg_rx {
            Ok(rx.clone())
        } else {
            Err(Status::new(
                Code::Internal,
                "No channel created for this room"
            ))
        }
    }
}

pub fn start_game(rid: String) {

}
