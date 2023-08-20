use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use tokio::sync::watch::{self, Sender, Receiver};
use crate::{*, game::*, desk::*};
use rand::{thread_rng, seq::SliceRandom};

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
    gamemsg_tx: Option<MsgTX>,
    gamemsg_rx: Option<MsgRX>,
    next: usize,
    desk: Desk,
    thisround: Vec<Card>,
    play_cnt: u32,
}

#[derive(Debug, Default, PartialEq)]
enum RoomState {
    #[default] NotFull,
    WaitReady,
    Gaming,
    EndGame,
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

        let mut r = Room {
            state: RoomState::NotFull,
            players: Vec::new(),
            id: id.clone(),
            gamemsg_tx: None,
            gamemsg_rx: None,
            ready_cnt: 0,
            next: 0,
            ..Default::default()

        };

        let mut rooms = self.rooms.write().await;

        if let Some(_) = rooms.get(&id) {
            return Err(Status::new(
                Code::AlreadyExists,
                format!("Room {} already exists!", id),
            ));
        }

        let initmsg = GameMsg {
            msg: Some(Msg::InitMsg(true))
        };

        let (tx, rx) = watch::channel(Ok(initmsg));

        r.gamemsg_tx = Some(tx);
        r.gamemsg_rx = Some(rx);

        let ar = Arc::new(RwLock::new(r));
        rooms.insert(id.clone(), ar.clone());
        Ok(ar)
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

    fn get_ready_list(&self) -> ReadyList {
        let mut l: Vec<u32> = Vec::new();
        for (i, p) in self.players.iter().enumerate() {
            if p.game.is_ready() {
                l.push(i as u32);
            }
        }

        ReadyList { l }
    }

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

    pub fn send_gamemsg(&self, msg: GameMsg) {
        if let Some(tx) = self.gamemsg_tx.as_ref() {
            if let Err(e) = tx.send(Ok(msg)) {
                error!("Cannot send gamemsg: {:?}", e);
            }
        } else {
            error!("No channel created for this room");
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
            Ok(4 - self.ready_cnt)
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("Player {} not exist!", pid),
            ))
        }
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

    pub fn start_game(&mut self) {
        if self.state != RoomState::WaitReady {
            error!("Room {} is not full or game has begun!", &self.id);
        } else if self.ready_cnt != 4 {
            error!("Room {} not ready!", &self.id);
        }
 
        let _ = self.players.iter_mut().map(
            |p| p.game.new_game()
        );

        self.desk = Default::default();

        self.thisround.clear();

        self.play_cnt = 0;

        let mut cards: Vec<u32> = (0..51).collect();
        cards.shuffle(&mut thread_rng());

        for pi in 0..3 {
            for c in &cards[pi*13 .. (pi+1)*13-1] {
                self.players[pi].game.add_card(c);
                if *c == 19 {
                    self.next = pi;
                }
            }
        }

        self.state = RoomState::Gaming;

        let msg = GameMsg {
            msg: Some(Msg::Start(self.next as u32))
        };
        info!("Sending GameMsg: {:?}", msg);
        self.send_gamemsg(msg);
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

        p.game.is_valid_play(&self.desk, play)?;

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

    pub fn exit_game(&mut self, pid: usize) -> RPCResult<()> {
        if pid < self.players.len() {
            match self.state {
                RoomState::NotFull =>
                    Err(Status::new(
                        Code::PermissionDenied,
                        "Room is not in a game!"
                    )),
                RoomState::EndGame => {
                    self.ready_cnt = 0;
                    let _ = self.players.iter_mut().map(
                        |p| p.game.unready()
                    );
                    self.state = RoomState::WaitReady;
                    Ok(())
                },
                _ => {
                    self.ready_cnt = 0;
                    let _ = self.players.iter_mut().map(
                        |p| p.game.unready()
                    );
                    self.state = RoomState::WaitReady;
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
        if pid < self.players.len() {
            self.ready_cnt = 0;
            self.players.remove(pid);
            let _ = self.players.iter_mut().map(
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
}
