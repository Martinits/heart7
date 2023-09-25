use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Sender, Receiver};
use crate::{*, game::*, desk::*};
use rand::{thread_rng, seq::SliceRandom};
use tokio::time;
use tokio_util::sync::CancellationToken;

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
    ready_cnt: u32,
    id: String,
    next: usize,
    desk: Desk,
    thisround: Vec<Card>,
    play_cnt: u32,
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

#[derive(Debug, Clone)]
struct Player {
    name: String,
    gamemsg_tx: MsgTX,
    game: Game,
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
            players: Vec::new(),
            id: name.clone(),
            ready_cnt: 0,
            next: 0,
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
                                    let next = room.next;
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
        let mut l: Vec<u32> = Vec::new();
        for (i, p) in self.players.iter().enumerate() {
            if p.game.is_ready() {
                l.push(i as u32);
            }
        }

        ReadyList { l }
    }

    // this function does not set the `myidx` item
    pub fn get_room_info(&self) -> RPCResult<RoomInfo> {
        Ok(RoomInfo {
            roomid: self.id.clone(),
            players: self.players.iter().map(
                |p| PlayerInfo{ name: p.name.clone() }
            ).collect(),
            state: Some(match self.state {
                RoomState::NotFull => State::NotFull(self.players.len() as u32),
                RoomState::WaitReady => State::WaitReady(self.get_ready_list()),
                RoomState::Gaming => State::Gaming(self.next as u32),
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

        self.players.push(Player {
            name: p.name.clone(),
            game: Default::default(),
            gamemsg_tx: tx,
        });

        if self.players.len() == 4 {
            self.state = RoomState::WaitReady;
        }

        Ok(rx)
    }

    pub async fn send_gamemsg(&self, msg: Msg) {
        for (i, p) in self.players.iter().enumerate() {
            if let Err(e) = p.gamemsg_tx.send(
                Ok(
                    GameMsg {
                        msg: Some(msg.clone()),
                        your_id: i as u32,
                    }
                )).await {
                error!("Cannot send gamemsg: {:?}", e);
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

        if let Some(p) = self.players.get_mut(pid) {
            p.game.ready()?;
            self.ready_cnt += 1;
            self.player_alive = true;
            Ok(4 - self.ready_cnt)
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("Player {} not exist!", pid),
            ))
        }
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
        } else if self.ready_cnt != 4 {
            error!("Room {} not ready!", &self.id);
        }
 
        self.players.iter_mut().for_each(
            |p| p.game.new_game()
        );

        self.desk = Default::default();

        self.thisround.clear();

        self.play_cnt = 0;

        let mut cards: Vec<u32> = (0..=51).collect();
        cards.shuffle(&mut thread_rng());

        for pi in 0..=3 {
            for c in &cards[pi*13 .. (pi+1)*13] {
                self.players[pi].game.add_card(c);
                if *c == 19 {
                    self.next = pi;
                }
            }
        }

        self.state = RoomState::Gaming;

        let msg = Msg::Start(self.next as u32);
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

        let p = self.players.get(pid as usize).ok_or(
            Status::new(
                Code::NotFound,
                format!("Player {} not found!", pid)
            )
        )?;

        Ok(GameInfo {
            cards: p.game.get_cards(),
            holds: Some(HeldCards {
                my: p.game.get_holds(),
                eachone: self.players.iter().map(
                    |p| p.game.get_holds_num()
                ).collect(),
            }),
            desk: Some(self.desk.get_desk_info(&self.thisround)),
        })
    }

    pub fn play_card(&mut self, pid: u32, play: &Play) -> RPCResult<u32> {
        if self.state != RoomState::Gaming {
            return Err(Status::new(
                Code::PermissionDenied,
                "Not gaming!"
            ));
        } else if self.next != pid as usize {
            return Err(Status::new(
                Code::PermissionDenied,
                format!("Not your turn! Waiting for player {}!", self.next)
            ));
        }

        let p = self.players.get_mut(pid as usize).ok_or(
            Status::new(
                Code::NotFound,
                format!("Player {} not exist!", pid)
            )
        )?;

        self.player_alive = true;

        p.game.is_valid_play(&self.desk, play, self.play_cnt == 0)?;

        p.game.play_card(play)?;

        self.next += 1;
        self.next %= 4;

        self.play_cnt += 1;

        self.desk.update(play, pid);

        if pid == 0 {
            self.thisround.clear();
        }
        if let Play::Discard(c) = play {
            self.thisround.push(Card::from_info(c));
        }

        Ok(self.play_cnt)
    }

    pub fn end_game(&mut self) -> RPCResult<GameResult> {
        if self.state != RoomState::Gaming {
            Err(Status::new(
                Code::PermissionDenied,
                "Room is not gaming!"
            ))
        } else if self.play_cnt != 52 {
            Err(Status::new(
                Code::PermissionDenied,
                "Game is not end!"
            ))
        } else if self.players.iter().any(|p| p.game.has_cards()) {
            Err(Status::new(
                Code::PermissionDenied,
                "Try to get game result but someone still owns cards!"
            ))
        } else {
            self.state = RoomState::EndGame;
            Ok(self.get_game_result())
        }
    }

    // this function doesn't check whether game is end !!!
    fn get_game_result(&self) -> GameResult {
        let mut hold = Vec::new();
        self.players.iter().for_each(
            |p| hold.push(p.game.get_hold_list())
        );

        GameResult{
            desk: Some(self.desk.clone().into()),
            hold,
        }
    }

    pub async fn exit_game(&mut self, pid: usize) -> RPCResult<()> {
        if pid < self.players.len() {
            match self.state {
                RoomState::NotFull =>
                    Err(Status::new(
                        Code::PermissionDenied,
                        "Room is not in a game!"
                    )),
                RoomState::WaitReady => Ok(()),
                _ => {
                    self.ready_cnt = 0;
                    self.players.iter_mut().for_each(
                        |p| p.game.unready()
                    );
                    self.state = RoomState::WaitReady;
                    self.send_gamemsg(Msg::ExitGame(pid as u32)).await;
                    Ok(())
                }
            }
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("Player {} not exist!", pid),
            ))
        }
    }

    pub fn exit_room(&mut self, pid: usize) -> RPCResult<usize> {
        debug!("{:?}", self.players.len());
        if pid < self.players.len() {
            self.ready_cnt = 0;
            self.players.remove(pid);
            self.players.iter_mut().for_each(
                |p| p.game.unready()
            );
            self.state = RoomState::NotFull;
            Ok(self.players.len())
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("Player {} not exist!", pid),
            ))
        }
    }

    pub fn kill_unready(&mut self) -> RPCResult<usize> {
        assert!(self.state == RoomState::WaitReady);

        info!("Killing unready: {:?}", self.players.iter().filter(
            |p| !p.game.is_ready()
        ).map(
            |p| p.name.clone()
        ).collect::<String>());

        self.players = self.players.iter().filter(
            |p| p.game.is_ready()
        ).cloned().collect();

        self.ready_cnt = 0;
        self.state = RoomState::NotFull;
        Ok(self.players.len())
    }
}
