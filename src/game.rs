use crate::{*, desk::*};
use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct Game {
    cards: HashSet<Card>,
    holds: HashSet<Card>,
    ready: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Card {
    pub suit: CardSuit,
    pub num: u32,
}

impl Into<CardInfo> for Card {
    fn into(self) -> CardInfo {
        CardInfo {
            suit: self.suit as i32,
            num: self.num,
        }
    }
}

impl Into<CardResult> for (Card, u32) {
    fn into(self) -> CardResult {
        CardResult{
            card: Some(self.0.into()),
            whose: self.1
        }
    }
}

impl Card {
    pub fn from_info(cinfo: &CardInfo) -> Card {
        Card {
            suit: match cinfo.suit {
                0 => CardSuit::Spade,
                1 => CardSuit::Heart,
                2 => CardSuit::Club,
                _ => CardSuit::Diamond,
            },
            num: cinfo.num,
        }
    }

    pub fn into_result(self, pid: u32) -> CardResult {
        CardResult{
            card: Some(self.into()),
            whose: pid,
        }
    }
}


impl Game {
    pub fn ready(&mut self) -> RPCResult<()> {
        if self.ready {
            Err(Status::new(
                Code::AlreadyExists,
                "You have been ready!"
            ))
        } else {
            self.ready = true;
            Ok(())
        }
    }
    pub fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn unready(&mut self) {
        self.ready = false
    }

    pub fn has_cards(&self) -> bool {
        self.cards.len() != 0
    }

    pub fn add_card(&mut self, c: &u32) {
        let card = Card {
            suit: match c/13 {
                0 => CardSuit::Spade,
                1 => CardSuit::Heart,
                2 => CardSuit::Club,
                _ => CardSuit::Diamond,
            },
            num: c%13 + 1,
        };

        if !self.cards.insert(card) {
            error!("Cannot add card")
        }
    }

    pub fn new_game(&mut self) {
        if !self.ready {
            error!("Player not ready!");
        }

        self.cards.clear();
        self.holds.clear();
    }

    pub fn get_cards(&self) -> Vec<CardInfo> {
        self.cards.clone().into_iter().map(
            |c| c.into()
        ).collect()
    }

    pub fn get_holds_num(&self) -> u32 {
        self.holds.len() as u32
    }

    pub fn get_holds(&self) -> Vec<CardInfo> {
        self.holds.clone().into_iter().map(
            |c| c.into()
        ).collect()
    }

    pub fn get_hold_list(&self) -> HoldList {
        HoldList{
            holds: self.holds.iter().map(
                |c| c.clone().into()
            ).collect()
        }
    }

    fn has_card(&self, c: &Card) -> bool {
        if let Some(_) = self.cards.iter().find(|&cc| cc == c) {
            true
        } else {
            false
        }
    }

    pub fn is_valid_play(&self, desk: &Desk, play: &Play, is_first: bool) -> RPCResult<()>{
        match play {
            Play::Discard(ci) => {
                let c = Card::from_info(ci);
                if self.has_card(&c) {
                    desk.is_valid_discard(&c, is_first)?;
                    Ok(())
                } else {
                    Err(Status::new(
                        Code::PermissionDenied,
                        "You don't own this card!"
                    ))
                }
            },
            Play::Hold(ci) => {
                let c = Card::from_info(ci);
                if self.has_card(&c) {
                    let desk_cand = desk.discard_candidates(is_first);
                    if desk_cand.intersection(&self.cards).any(|_| true) {
                        Err(Status::new(
                            Code::PermissionDenied,
                            "You can't hold, since you have cards to play!"
                        ))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(Status::new(
                        Code::PermissionDenied,
                        "You don't own this card!"
                    ))
                }
            }
        }
    }

    // this function doesn't check whether is valid !!!
    pub fn play_card(&mut self, play: &Play) -> RPCResult<()> {
        match play {
            Play::Discard(ci) => {
                let c = Card::from_info(ci);
                if !self.cards.remove(&c) {
                    error!("Remove a card that player doesn't own!");
                }
                Ok(())
            },
            Play::Hold(ci) => {
                let c = Card::from_info(ci);
                if !self.cards.remove(&c) {
                    error!("Remove a card that player doesn't own!");
                }
                if !self.holds.insert(c) {
                    error!("Already held this card!");
                }
                Ok(())
            }
        }
    }
}
