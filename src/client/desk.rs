use crate::game::Card;
use crate::*;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Desk {
    // (cards<=7, cards>7), index == 0: min or max cardnum
    // u32: 1: this round, 2: this round by myself
    pub spade:   (Vec<(Card, u32)>, Vec<(Card, u32)>),
    pub heart:   (Vec<(Card, u32)>, Vec<(Card, u32)>),
    pub club:    (Vec<(Card, u32)>, Vec<(Card, u32)>),
    pub diamond: (Vec<(Card, u32)>, Vec<(Card, u32)>),
}

impl Desk {
    pub fn add(&mut self, c: Card, is_myself: bool) {
        let target: &mut Vec<(Card, u32)> = match (c.suit, c.num <= 7) {
            (CardSuit::Spade,   true) => self.spade.0.as_mut(),
            (CardSuit::Spade,   false) => self.spade.1.as_mut(),
            (CardSuit::Heart,   true) => self.heart.0.as_mut(),
            (CardSuit::Heart,   false) => self.heart.1.as_mut(),
            (CardSuit::Club,    true) => self.club.0.as_mut(),
            (CardSuit::Club,    false) => self.club.1.as_mut(),
            (CardSuit::Diamond, true) => self.diamond.0.as_mut(),
            (CardSuit::Diamond, false) => self.diamond.1.as_mut(),
        };

        assert!(target.len() == 0 || (target[0].0.num.abs_diff(7) < c.num.abs_diff(7)));

        if target.len() > 0 && target[0].1 == 0 {
            assert!(target.len() == 1);
            target[0] = (c, if is_myself { 2 } else { 1 } );
        } else {
            target.insert(0, (c, if is_myself { 2 } else { 1 } ));
        }
    }

    pub fn new_round(&mut self) {
        for each in [
            &mut self.spade.0, &mut self.spade.1,
            &mut self.heart.0, &mut self.heart.1,
            &mut self.club.0, &mut self.club.1,
            &mut self.diamond.0, &mut self.diamond.1,
        ] {
            if each.len() > 0 {
                each.iter_mut().for_each( |(_, r)| *r = 0 );
                each.truncate(1);
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.spade.0.len() == 0
        && self.spade.1.len() == 0
        && self.heart.0.len() == 0
        && self.heart.1.len() == 0
        && self.club.0.len() == 0
        && self.club.1.len() == 0
        && self.diamond.0.len() == 0
        && self.diamond.1.len() == 0
    }

    pub fn get_play_hint(&self, cards: &Vec<Card>) -> Vec<bool> {
        let mut possible = HashSet::new();
        if self.is_empty() {
            possible.insert(Card { suit: CardSuit::Heart, num: 7 });
        } else {
            for (small, big, cs) in [
                (&self.spade.0,   &self.spade.1,   CardSuit::Spade),
                (&self.heart.0,   &self.heart.1,   CardSuit::Heart),
                (&self.club.0,    &self.club.1,    CardSuit::Club),
                (&self.diamond.0, &self.diamond.1, CardSuit::Diamond),
            ] {
                if small.len() == 0 && big.len() == 0 {
                    possible.insert(Card { suit: cs, num: 7 });
                } else {
                    if big.len() == 0 {
                        possible.insert(Card { suit: cs, num: 8 });
                    } else if big[0].0.num != 13 {
                        possible.insert(Card { suit: cs, num: big[0].0.num+1 });
                    }
                    if small[0].0.num != 1 {
                        possible.insert(Card { suit: cs, num: small[0].0.num-1 });
                    }
                }
            }
        }

        let mut hint = Vec::new();
        cards.iter().for_each( |c| {
            hint.push(possible.get(c).is_some());
        });
        hint
    }
}
