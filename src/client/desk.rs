use crate::game::Card;
use crate::*;

#[derive(Debug, Default)]
pub struct Desk {
    // (cards<=7, cards>7), index == 0: min or max cardnum
    // bool means whether it is in this round
    pub spade:   (Vec<(Card, bool)>, Vec<(Card, bool)>),
    pub heart:   (Vec<(Card, bool)>, Vec<(Card, bool)>),
    pub club:    (Vec<(Card, bool)>, Vec<(Card, bool)>),
    pub diamond: (Vec<(Card, bool)>, Vec<(Card, bool)>),
}

impl Desk {
    pub fn add(&mut self, c: Card) {
        let target: &mut Vec<(Card, bool)> = match (c.suit, c.num <= 7) {
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

        if target.len() > 0 && !target[0].1 {
            assert!(target.len() == 1);
            target[0] = (c, true);
        } else {
            target.insert(0, (c, true));
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
                each.iter_mut().for_each( |(_, r)| *r = false );
                each.truncate(1);
            }
        }
    }
}
