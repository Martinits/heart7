pub mod desk;
pub mod game;
pub mod player;

pub use game::{Game, GameResult, GameError};
pub use player::Player;

use crate::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Card {
    pub suit: CardSuit,
    pub num: u32,
}

pub static DUMMY_CARD: Card = Card {
    suit: CardSuit::Spade,
    num: 0,
};

impl Card {
    pub fn is_dummy_card(&self) -> bool {
        *self == DUMMY_CARD
    }
}

impl Into<CardInfo> for Card {
    fn into(self) -> CardInfo {
        CardInfo {
            suit: self.suit as i32,
            num: self.num,
        }
    }
}

impl Into<CardInfo> for &Card {
    fn into(self) -> CardInfo {
        self.clone().into()
    }
}

impl Into<CardResult> for &(Card, usize) {
    fn into(self) -> CardResult {
        CardResult{
            card: Some(self.0.clone().into()),
            whose: self.1 as u32,
        }
    }
}

impl From<CardInfo> for Card {
    fn from(value: CardInfo) -> Self {
        Card {
            suit: match value.suit {
                0 => CardSuit::Spade,
                1 => CardSuit::Heart,
                2 => CardSuit::Club,
                _ => CardSuit::Diamond,
            },
            num: value.num,
        }
    }
}

impl From<&CardInfo> for Card {
    fn from(value: &CardInfo) -> Self {
        value.clone().into()
    }
}

impl From<u32> for Card {
    fn from(value: u32) -> Self {
        Card {
            suit: match value/13 {
                0 => CardSuit::Spade,
                1 => CardSuit::Heart,
                2 => CardSuit::Club,
                _ => CardSuit::Diamond,
            },
            num: value%13 + 1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Play {
    Discard(Card, usize),
    Hold(Card, usize),
}

impl From<PlayInfo> for Play {
    fn from(value: PlayInfo) -> Self {
        let pc = value.playone.unwrap();
        let c = pc.card.as_ref().unwrap().into();
        if pc.is_discard {
            Play::Discard(c, value.player as usize)
        } else {
            Play::Hold(c, value.player as usize)
        }
    }
}

impl Play {
    pub fn split(self) -> (bool, Card, usize) {
        match self {
            Self::Discard(c, pid) => (true, c, pid),
            Self::Hold(c, pid) => (false, c, pid),
        }
    }

    pub fn get_pid(&self) -> usize {
        match self {
            Self::Discard(_, pid) | Self::Hold(_, pid) => *pid,
        }
    }
}
