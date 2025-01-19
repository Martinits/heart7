use crate::*;
use super::desk::*;
use super::player::Player;
use rand::{thread_rng, seq::SliceRandom};
use thiserror::Error;
use super::*;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Permission Denied: {0}")]
    PermissionDenied(String),
    #[error("NotFound: {0}")]
    NotFound(String),
    #[error("AlreadyDone: {0}")]
    AlreadyDone(String),
    #[error("Internal: {0}")]
    Internal(String),
}

impl From<GameError> for tonic::Status {
    fn from(value: GameError) -> Self {
        match value {
            GameError::PermissionDenied(s) => Self::permission_denied(s),
            GameError::NotFound(s) => Self::not_found(s),
            GameError::AlreadyDone(s) => Self::already_exists(s),
            GameError::Internal(s) => Self::internal(s),
        }
    }
}

pub type GameResult<T> = Result<T, GameError>;

#[derive(Debug, Default, Clone)]
pub struct Game {
    desk: Desk,
    players: Vec<Player>,
    next: usize,
    ready_cnt: u32,
    thisround: Vec<(Card, usize)>,
    last: Option<Play>,
    play_cnt: u32,
}

static END_GAME_CNT: u32 = 52;

impl Game {
    pub fn add_player(&mut self, name: String) {
        self.players.push(Player::new(name));
    }

    pub fn new_game(&mut self) -> GameResult<()> {
        if self.players.len() != 4 {
            return Err(GameError::PermissionDenied("Player not enough!".into()))
        }

        if self.ready_cnt != 4 || self.players.iter().any(|p| !p.is_ready()) {
            return Err(GameError::PermissionDenied("Not everyone ready!".into()))
        }

        self.clear();

        let mut cards: Vec<u32> = (0..=51).collect();
        cards.shuffle(&mut thread_rng());

        for pi in 0..=3 {
            for c in &cards[pi*13 .. (pi+1)*13] {
                self.players[pi].add_card(Card::from(*c))?;
                if *c == 19 {
                    self.next = pi;
                }
            }
        }

        Ok(())
    }

    pub fn get_next(&self) -> usize {
        self.next
    }

    pub fn is_my_turn(&self) -> bool {
        self.get_next() == 0
    }

    pub fn set_next(&mut self, next: usize) {
        self.next = next;
    }

    pub fn has_just_begin(&self) -> bool {
        self.play_cnt == 0
    }

    pub fn get_last(&self) -> Option<(usize, Option<Card>)> {
        if self.has_just_begin() {
            return None
        }

        self.last.clone().map(
            |p| {
                let (is_discard, c, who) = p.split();
                (who, is_discard.then_some(c))
            }
        )
    }

    pub fn get_last_card(&self) -> Option<Play> {
        self.last.clone()
    }

    fn clear(&mut self) {
        self.desk.clear();
        self.thisround.clear();
        self.players.iter_mut().for_each(
            |p| p.reset()
        );
        self.ready_cnt = 0;
        self.play_cnt = 0;
        self.next = 0;
        self.last = None;
    }

    fn check_pid(&self, pid: usize) -> GameResult<()> {
        if pid >= self.players.len() {
            Err(GameError::NotFound(format!("Player {} not exist!", pid)))
        } else {
            Ok(())
        }
    }

    pub fn player_exit_game(&mut self, pid: usize) -> GameResult<()> {
        self.check_pid(pid)?;

        self.clear();

        Ok(())
    }

    pub fn player_exit(&mut self, pid: usize) -> GameResult<usize> {
        self.check_pid(pid)?;

        self.players.remove(pid);
        self.clear();

        Ok(self.players.len())
    }

    pub fn get_ready_list(&self) -> Vec<usize> {
        self.players.iter().enumerate().filter_map(
            |(i, p)| p.is_ready().then_some(i)
        ).collect()
    }

    pub fn kill_unready(&mut self) -> GameResult<usize> {
        self.players = self.players.iter().filter(
            |p| p.is_ready()
        ).cloned().collect();

        self.clear();

        Ok(self.players.len())
    }

    pub fn get_player_names(&self) -> Vec<String> {
        self.players.iter().map(
            |p| p.get_name()
        ).collect()
    }


    pub fn get_player_name(&self, pid: usize) -> String {
        self.players.get(pid).expect("Invalid pid!").get_name()
    }

    pub fn get_my_name(&self) -> String {
        self.get_player_name(0)
    }

    pub fn get_player_num(&self) -> usize {
        self.players.len()
    }

    pub fn player_ready(&mut self, pid: usize) -> GameResult<u32> {
        self.check_pid(pid)?;

        let p = self.players.get_mut(pid).unwrap();
        if p.is_ready() {
            return Err(GameError::AlreadyDone("You have been ready!".into()))
        }

        p.get_ready();

        self.ready_cnt += 1;

        Ok(4 - self.ready_cnt)
    }

    pub fn get_someone_cards(&self, pid: usize) -> GameResult<Vec<Card>> {
        self.check_pid(pid)?;

        let mut cards = self.players.get(pid).unwrap().get_cards();
        cards.sort();
        Ok(cards)
    }

    pub fn get_my_cards(&self) -> Vec<Card> {
        self.get_someone_cards(0).unwrap()
    }

    pub fn do_i_have_cards(&self) -> bool {
        self.get_my_card_num() != 0
    }

    pub fn get_my_card_num(&self) -> usize {
        self.players[0].get_card_num()
    }

    pub fn get_someone_holds(&self, pid: usize) -> GameResult<Vec<Card>> {
        self.check_pid(pid)?;

        Ok(self.players.get(pid).unwrap().get_holds())
    }

    pub fn get_my_holds(&self) -> Vec<Card> {
        self.get_someone_holds(0).unwrap()
    }

    pub fn get_hold_nums(&self) -> Vec<u32> {
        self.players.iter().map(
            |p| p.get_hold_num()
        ).collect()
    }

    pub fn has_done(&self) -> bool {
        self.play_cnt >= 54
    }

    pub fn get_thisround(&self) -> Vec<Card> {
        self.thisround.clone().into_iter().map(
            |(c, _)| c
        ).collect()
    }

    pub fn get_thisround_my(&self) -> Option<Card> {
        self.thisround.iter().find_map(
            |(c, who)| (*who == 0).then_some(c.clone())
        )
    }

    pub fn get_my_hint(&mut self) -> Vec<bool> {
        self.get_my_cards().iter().map(
            |c| self.desk.is_discard_candidates(c)
        ).collect()
    }

    pub fn check_play(&mut self, play: &Play) -> GameResult<()> {
        let (is_discard, c, pid) = play.clone().split();

        self.check_pid(pid)?;

        if self.next != pid as usize {
            return Err(GameError::PermissionDenied(
                format!("Not your turn! Waiting for player {}!", self.next)
            ));
        }

        let p = self.players.get(pid).unwrap();
        if !p.has_card(&c) {
            return Err(GameError::PermissionDenied(
                "You don't own this card!".into()
            ))
        }

        let is_cand = self.desk.is_discard_candidates(&c);

        if is_discard && !is_cand {
            return Err(GameError::PermissionDenied(
                "You can't play this card!".into()
            ))
        }

        if !is_discard && p.is_holding(&c) {
            return Err(GameError::PermissionDenied(
                "You are already holding this card!".into()
            ))
        }

        if !is_discard && self.desk.someone_has_discard_candidates(p.get_cards_set()) {
            return Err(GameError::PermissionDenied(
                "You can't hold, since you have cards to play!".into()
            ))
        }

        Ok(())
    }

    pub fn play_card_no_check(&mut self, play: Play) -> GameResult<()> {
        self.desk.play_card(play.clone());

        let pid = play.get_pid();
        self.players.get_mut(pid).unwrap().play_card(play.clone());

        if self.play_cnt % 4 == 0 {
            self.thisround.clear();
        }
        self.play_cnt += 1;

        self.next += 1;
        self.next %= 4;

        if let Play::Discard(c, _) = &play {
            self.thisround.push((c.clone(), pid));
        }
        self.last = Some(play);

        Ok(())
    }

    pub fn play_card(&mut self, play: Play) -> GameResult<bool> {
        self.check_play(&play)?;
        self.play_card_no_check(play)?;
        Ok(self.play_cnt == END_GAME_CNT)
    }

    pub fn get_desk_info(&self) -> DeskInfo {
        DeskInfo {
            spade: self.get_chain_info(self.desk.get_chain(CardSuit::Spade)),
            heart: self.get_chain_info(self.desk.get_chain(CardSuit::Heart)),
            club: self.get_chain_info(self.desk.get_chain(CardSuit::Club)),
            diamond: self.get_chain_info(self.desk.get_chain(CardSuit::Diamond)),
        }
    }

    fn get_chain_info(&self, chain: &ChainType) -> Option<ChainInfo> {
        if chain.len() == 0{
            return None;
        }

        let front: Card = chain.front().unwrap().0.clone();
        let back: Card = chain.back().unwrap().0.clone();
        let mut front_is_thisround = false;
        let mut back_is_thisround = false;

        if let Some(_) = self.thisround.iter().find(|(cc, _)| *cc == front) {
            front_is_thisround = true;
        }

        if let Some(_) = self.thisround.iter().find(|(cc, _)| *cc == back) {
            back_is_thisround = true;
        }

        Some(ChainInfo {
            front: Some(front.into()),
            back: Some(back.into()),
            front_is_thisround,
            back_is_thisround,
        })
    }

    fn get_hold_list(&self) -> Vec<HoldList> {
        (0..4).map(
            |pid| HoldList {
                holds: self.get_someone_holds(pid).unwrap().into_iter().map(
                    |c| c.into()
                ).collect()
            }
        ).collect()
    }

    pub fn end_game(&self) -> GameResult<GameEnding> {
        if self.play_cnt != END_GAME_CNT {
            return Err(GameError::PermissionDenied("Game has not ended!".into()))
        }

        if self.players.iter().any(|p| p.has_card_left()) {
            return Err(GameError::PermissionDenied(
                "Someone still owns cards!".into()
            ))
        }

        Ok(GameEnding {
            desk: Some(self.desk.get_desk_result()),
            hold: self.get_hold_list(),
        })
    }

    pub fn init_my_cards(&mut self, cards: Vec<Card>) {
        assert_eq!(self.players.len(), 4);

        self.players[0].init_cards(cards);

        (1..=3).for_each(
            |pi| self.players[pi].init_dummy_cards()
        );
    }

    pub fn export_desk(&self) -> Vec<Vec<Card>> {
        self.desk.export()
    }
}
