use crate::*;
use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct Game {
    cards: HashSet<Card>,
    holds: HashSet<Card>,
    ready: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
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

        if self.cards.insert(card) {
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
}
