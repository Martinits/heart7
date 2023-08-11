use std::collections::HashMap;
use super::*;
use tonic::Code;
use tokio::sync::RwLock;
use std::sync::Arc;

type ARoom = Arc<RwLock<Room>>;

#[derive(Debug, Default)]
pub struct RoomManager {
    rooms: Arc<RwLock<HashMap<String, ARoom>>>,
}

#[derive(Debug, Default)]
pub struct Room {
    players: Vec<Player>,
    state: RoomState,
    id: String,
}

#[derive(Debug, Default, Clone)]
struct Player {
    name: String,
}

type RMResult<T> = Result<T, tonic::Status>;

impl RoomManager {
    pub async fn new_room(&self) -> RMResult<ARoom> {
        let id = uuid::Uuid::new_v4().to_string();

        let r = Room {
            state: RoomState::NotFull,
            players: Vec::new(),
            id: id.clone(),
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

    pub async fn get_room(&self, id: &String) -> RMResult<ARoom> {
        if let Some(ar) = self.rooms.read().await.get(id) {
            Ok(ar.clone())
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("Room {} not found !", id),
            ))
        }
    }

    pub async fn del_room(&self, id: &String) -> RMResult<()> {
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
    pub fn get_room_info(&self) -> RMResult<RoomInfo> {
        Ok(RoomInfo {
            roomid: self.id.clone(),
            players: self.players.iter().map(
                |p| PlayerInfo{ name: p.name.clone() }
            ).collect(),
            state: self.state as i32,
        })
    }

    pub fn add_player(&mut self, p: &PlayerInfo) -> RMResult<()> {
        self.players.push(Player{
            name: p.name.clone(),
        });
        Ok(())
    }
}
