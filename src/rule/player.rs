use std::collections::HashSet;
use super::*;

#[derive(Debug, Default, Clone)]
pub struct Player {
    name: String,
    cards: HashSet<Card>,
    holds: Vec<Card>,
    ready: bool,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            ..Default::default()
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_ready(&mut self) {
        self.ready = true;
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn reset(&mut self) {
        self.ready = false;
        self.clear();
    }

    pub fn add_card(&mut self, c: Card) -> GameResult<()> {
        if !self.cards.insert(c) {
            return Err(GameError::Internal("Add same card multiple times!".into()))
        }
        Ok(())
    }

    pub fn init_cards(&mut self, cards: Vec<Card>) {
        self.cards = cards.into_iter().collect();
        self.holds.clear();
    }

    pub fn init_dummy_cards(&mut self) {
        let dc: Vec<Card> = (0..13).map(
            |_| DUMMY_CARD.clone()
        ).collect();
        self.init_cards(dc);
    }

    pub fn clear(&mut self) {
        self.cards.clear();
        self.holds.clear();
    }

    pub fn get_cards(&self) -> Vec<Card> {
        self.cards.iter().map(
            |c| c.clone()
        ).collect()
    }

    pub fn get_cards_iter(&self) -> impl Iterator<Item = &Card> {
        self.cards.iter()
    }

    pub fn get_card_num(&self) -> usize {
        self.cards.len()
    }

    pub fn get_hold_num(&self) -> u32 {
        self.holds.len() as u32
    }

    pub fn get_holds(&self) -> Vec<Card> {
        self.holds.clone()
    }

    pub fn has_card(&self, c: &Card) -> bool {
        self.cards.iter().find(|&cc| cc == c).is_some()
    }

    pub fn has_card_left(&self) -> bool {
        self.cards.len() != 0
    }

    pub fn is_holding(&self, c: &Card) -> bool {
        self.holds.iter().find(|&cc| cc == c).is_some()
    }

    // this function doesn't check whether is valid !!!
    pub fn play_card(&mut self, play: Play) {
        match play {
            Play::Discard(c, _) => {
                self.cards.remove(&c);
            },
            Play::Hold(c, _) => {
                self.cards.remove(&c);
                self.holds.push(c);
            }
        }
    }
}
