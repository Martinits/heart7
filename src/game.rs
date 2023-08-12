use crate::*;
use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct Game {
    cards: HashSet<Card>,
    holds: HashSet<Card>,
    ready: bool,
}

#[derive(Debug, Default, Clone)]
struct Card {
    suit: CardSuit,
    num: i32,
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
}
