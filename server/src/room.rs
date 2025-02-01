use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Sender, Receiver};
use crate::*;
use tokio::time;
use tokio_util::sync::CancellationToken;
use rand::{rng, seq::SliceRandom};

type ARoom = Arc<RwLock<Room>>;
type MsgTX = Sender<Result<GameMsg, Status>>;
pub type MsgRX = Receiver<Result<GameMsg, Status>>;

#[derive(Debug, Default)]
pub struct RoomManager {
    rooms: Arc<RwLock<HashMap<String, ARoom>>>,
}

#[derive(Debug, Default)]
pub struct Room {
    state: RoomState,
    id: String,
    game: Game,
    gamemsg_tx: Vec<MsgTX>,
    alive: bool,
    watch_dog_cancel: CancellationToken,
    player_alive: bool,
}

#[derive(Debug, Default, PartialEq)]
enum RoomState {
    #[default] NotFull,
    WaitReady,
    Gaming,
    EndGame,
}

impl RoomManager {
    pub fn spawn_watch_dog(&self) {
        let arooms = self.rooms.clone();
        tokio::spawn(async move {
            info!("Room watch dog running");
            loop {
                time::sleep(time::Duration::from_secs(3600)).await;

                let mut rooms = arooms.write().await;
                let keys: Vec<String> = rooms.iter().map(
                    |(id, _)| id.clone()
                ).collect();

                for id in keys.iter() {
                    let aroom = rooms.get(id).unwrap().clone();
                    let mut room = aroom.write().await;
                    if room.is_alive() {
                        room.unset_alive();
                    } else {
                        info!("Removing room {} by watch dog", id);
                        room.cancel();
                        rooms.remove(id).unwrap();
                    }
                }
            }
        });
    }

    pub async fn new_room(&self, name: &String) -> RPCResult<ARoom> {
        let mut rooms = self.rooms.write().await;

        if let Some(_) = rooms.get(name) {
            return Err(Status::new(
                Code::AlreadyExists,
                format!("Room {} already exists!", name),
            ));
        }

        let r = Room {
            state: RoomState::NotFull,
            id: name.clone(),
            alive: true,
            watch_dog_cancel: CancellationToken::new(),
            player_alive: true,
            ..Default::default()
        };

        let cancel = r.watch_dog_cancel.clone();

        let ar = Arc::new(RwLock::new(r));
        rooms.insert(name.clone(), ar.clone());

        // spawn watch dog
        let aroom = ar.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancel.cancelled() => {
                        break;
                    }
                    _ = time::sleep(time::Duration::from_secs(600)) => {
                        let mut room = aroom.write().await;
                        info!("Player watch dog shoots for room {}", room.get_id());
                        match room.state {
                            RoomState::WaitReady => {
                                if room.player_alive {
                                    room.player_alive = false;
                                } else {
                                    info!("In WaitReady: player watch dog kills unready");
                                    room.kill_unready().unwrap();
                                    let ri = room.get_room_info().unwrap();
                                    room.send_gamemsg(
                                        Msg::LoseConnection(ri)
                                    ).await;
                                }
                            }
                            RoomState::Gaming => {
                                if room.player_alive {
                                    room.player_alive = false;
                                } else {
                                    let next = room.game.get_next();
                                    info!("In Gaming: player watch dog kills {:?}", next);
                                    room.exit_room(next).unwrap();
                                    let ri = room.get_room_info().unwrap();
                                    room.send_gamemsg(
                                        Msg::LoseConnection(ri)
                                    ).await;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        Ok(ar)
    }

    pub async fn get_room(&self, id: &String) -> RPCResult<ARoom> {
        if let Some(ar) = self.rooms.read().await.get(id) {
            ar.write().await.set_alive();
            Ok(ar.clone())
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("Room {} not found !", id),
            ))
        }
    }

    pub async fn del_room(&self, id: &String) -> RPCResult<()> {
        if let Some(ar) = self.rooms.write().await.remove(id) {
            ar.read().await.cancel();
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
    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn set_alive(&mut self) {
        self.alive = true;
    }

    pub fn unset_alive(&mut self) {
        self.alive = false;
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn cancel(&self) {
        self.watch_dog_cancel.cancel();
    }

    fn get_ready_list(&self) -> ReadyList {
        ReadyList {
            l: self.game.get_ready_list().into_iter().map(|s| s as u32).collect()
        }
    }

    // this function does not set the `myidx` item
    pub fn get_room_info(&self) -> RPCResult<RoomInfo> {
        Ok(RoomInfo {
            roomid: self.id.clone(),
            players: self.game.get_player_names().into_iter().map(
                |p| PlayerInfo{ name: p }
            ).collect(),
            state: Some(match self.state {
                RoomState::NotFull => State::NotFull(self.game.get_player_num() as u32),
                RoomState::WaitReady => State::WaitReady(self.get_ready_list()),
                RoomState::Gaming => State::Gaming(self.game.get_next() as u32),
                RoomState::EndGame => State::EndGame(0),
            })
        })
    }

    pub fn add_player(&mut self, p: &PlayerInfo) -> RPCResult<super::room::MsgRX> {
        if self.state != RoomState::NotFull {
            return Err(Status::new(
                Code::ResourceExhausted,
                format!("Room {} is full!", &self.id)
            ));
        }

        let (tx, rx) = mpsc::channel(DEFAULT_CHANNEL_SIZE);

        self.gamemsg_tx.push(tx);
        self.game.add_player(p.name.clone());

        if self.game.get_player_num() == 4 {
            self.state = RoomState::WaitReady;
        }

        Ok(rx)
    }

    pub async fn send_gamemsg(&self, msg: Msg) {
        for i in 0..self.gamemsg_tx.len() {
            self.send_gamemsg_to(msg.clone(), i).await;
        }
    }

    pub async fn send_gamemsg_to(&self, msg: Msg, to: usize) {
        self.gamemsg_tx.get(to).unwrap().send(Ok(
            GameMsg {
                msg: Some(msg),
                your_id: to as u32,
            }
        )).await.unwrap_or_else(
            |e| error!("Cannot send gamemsg: {:?}", e)
        );
    }

    pub async fn send_gamemsg_except(&self, msg: Msg, except: usize) {
        for i in 0..self.gamemsg_tx.len() {
            if i != except {
                self.send_gamemsg_to(msg.clone(), i).await;
            }
        }
    }

    pub fn player_ready(&mut self, pid: usize) -> RPCResult<u32> {
        if self.state != RoomState::WaitReady {
            return Err(Status::new(
                Code::PermissionDenied,
                format!("Room {} is not full or game has begun!", &self.id)
            ))
        }

        let not_ready_cnt = self.game.player_ready(pid)?;

        self.player_alive = true;

        Ok(not_ready_cnt)
    }

    // pub fn get_gamemsg_rx(&self) -> RPCResult<MsgRX> {
    //     if let Some(ref rx) = self.gamemsg_rx {
    //         Ok(rx.clone())
    //     } else {
    //         Err(Status::new(
    //             Code::Internal,
    //             "No channel created for this room"
    //         ))
    //     }
    // }

    pub async fn start_game(&mut self) {
        if self.state != RoomState::WaitReady {
            error!("Room {} is not full or game has begun!", &self.id);
        }
 
        let mut cards: Vec<u32> = (0..=51).collect();
        cards.shuffle(&mut rng());

        self.game.new_game(cards).unwrap_or_else(
            |e| error!("Cannot start game: {}", e)
        );

        self.state = RoomState::Gaming;

        let msg = Msg::Start(self.game.get_next() as u32);
        info!("Sending GameMsg: {:?}", msg);
        self.send_gamemsg(msg).await;
    }

    pub fn get_game_info(&self, pid: u32) -> RPCResult<GameInfo> {
        if self.state != RoomState::Gaming {
            return Err(Status::new(
                Code::PermissionDenied,
                "Not gaming!"
            ));
        }

        Ok(GameInfo {
            cards: self.game.get_someone_cards(pid as usize)?.into_iter().map(
                |c| c.into()
            ).collect(),
            holds: Some(HeldCards {
                my: self.game.get_someone_holds(pid as usize)?.into_iter().map(
                    |c| c.into()
                ).collect(),
                eachone: self.game.get_hold_nums(),
            }),
            desk: Some(self.game.get_desk_info()),
        })
    }

    pub fn play_card(&mut self, p: Play) -> RPCResult<bool> {
        if self.state != RoomState::Gaming {
            return Err(Status::new(
                Code::PermissionDenied,
                "Not gaming!"
            ));
        }

        let endgame = self.game.play_card(p)?;

        self.player_alive = true;

        Ok(endgame)
    }

    pub fn end_game(&mut self) -> RPCResult<GameEnding> {
        if self.state != RoomState::Gaming {
            return Err(Status::new(
                Code::PermissionDenied,
                "Room is not gaming!"
            ))
        }

        let ge = self.game.end_game()?;
        self.state = RoomState::EndGame;

        Ok(ge)
    }

    pub async fn exit_game(&mut self, pid: usize) -> RPCResult<()> {
        match self.state {
            RoomState::NotFull =>
                Err(Status::new(
                    Code::PermissionDenied,
                    "Room is not in a game!"
                )),
            RoomState::WaitReady => Ok(()),
            _ => {
                self.game.player_exit_game(pid)?;
                self.state = RoomState::WaitReady;
                self.send_gamemsg(Msg::ExitGame(pid as u32)).await;
                Ok(())
            }
        }
    }

    pub fn exit_room(&mut self, pid: usize) -> RPCResult<usize> {
        let left = self.game.player_exit(pid)?;
        self.state = RoomState::NotFull;
        self.gamemsg_tx.remove(pid);
        Ok(left)
    }

    pub fn kill_unready(&mut self) -> RPCResult<usize> {
        assert!(self.state == RoomState::WaitReady);

        let ready_list = self.game.get_ready_list();

        let left = self.game.kill_unready()?;

        let mut new_gtx = Vec::new();
        for i in ready_list {
            new_gtx.push(self.gamemsg_tx[i].clone());
        }
        self.gamemsg_tx = new_gtx;

        self.state = RoomState::NotFull;
        Ok(left)
    }
}
