use std::collections::HashSet;
use super::*;

#[derive(Debug, Default, Clone)]
pub struct Player {
    name: String,
    cards: Vec<Card>,
    holds: Vec<Card>,
    ready: bool,
}
#[derive(Debug, Default, Clone)]
pub enum PlayCardResult {
    #[default] Normal,
    Clear,
    TheSeven,
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
        if self.cards.contains(&c) {
            return Err(GameError::Internal("Add same card multiple times!".into()))
        }
        self.cards.push(c);
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

    pub fn get_cards_set(&self) -> HashSet<Card> {
        self.cards.clone().into_iter().collect()
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
        self.cards.contains(c)
    }

    pub fn has_card_left(&self) -> bool {
        self.cards.len() != 0
    }

    pub fn is_holding(&self, c: &Card) -> bool {
        self.holds.iter().find(|&cc| cc == c).is_some()
    }

    // this function doesn't check whether is valid !!!
    pub fn play_card(&mut self, play: Play) -> PlayCardResult {
        assert!(self.has_card_left());

        let (is_discard, c, _) = play.split();

        if self.cards.first().unwrap().is_dummy_card() {
            self.cards.pop();
        } else {
            let idx = self.cards.iter().position(|cc| cc == &c).unwrap();
            self.cards.remove(idx);
        }

        if !is_discard {
            self.holds.push(c.clone());
        }

        if self.holds.len() == 0 && self.cards.len() == 0 {
            if c.num == 7 {
                PlayCardResult::TheSeven
            } else {
                PlayCardResult::Clear
            }
        } else {
            PlayCardResult::Normal
        }

    }
}
