use crate::*;
use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct Game {
    cards: HashSet<Card>,
    holds: HashSet<Card>,
    ready: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Card {
    suit: CardSuit,
    num: u32,
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
}
