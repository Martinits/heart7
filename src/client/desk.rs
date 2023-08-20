use crate::game::Card;
use crate::*;

#[derive(Debug, Default)]
pub struct Desk {
    // (cards<=7, cards>7), index == 0: min or max cardnum
    // bool means whether it is in this round
    spade:   (Vec<(Card, bool)>, Vec<(Card, bool)>),
    heart:   (Vec<(Card, bool)>, Vec<(Card, bool)>),
    club:    (Vec<(Card, bool)>, Vec<(Card, bool)>),
    diamond: (Vec<(Card, bool)>, Vec<(Card, bool)>),
    // bool: false => 0, true => 1 in tuple
    // usize: idx of Vec
    round:   Vec<(CardSuit, bool, usize)>
}

impl Desk {
    pub fn update(&mut self, c: Card, new_round: bool) {
        if new_round {
            for each in self.round.iter() {
                match each {
                    (CardSuit::Spade, false, idx) => {
                        if *idx == 0 {
                            self.spade.0[0].1 = false;
                        } else {
                            self.spade.0.remove(*idx);
                        }
                    }
                    (CardSuit::Spade,true, idx) => {
                        if *idx == 0 {
                            self.spade.1[0].1 = false;
                        } else {
                            self.spade.1.remove(*idx);
                        }
                    }
                    (CardSuit::Heart, false, idx) => {
                        if *idx == 0 {
                            self.heart.0[0].1 = false;
                        } else {
                            self.heart.0.remove(*idx);
                        }
                    }
                    (CardSuit::Heart, true, idx) => {
                        if *idx == 0 {
                            self.heart.1[0].1 = false;
                        } else {
                            self.heart.1.remove(*idx);
                        }
                    }
                    (CardSuit::Club, false, idx) => {
                        if *idx == 0 {
                            self.club.0[0].1 = false;
                        } else {
                            self.club.0.remove(*idx);
                        }
                    }
                    (CardSuit::Club, true, idx) => {
                        if *idx == 0 {
                            self.club.1[0].1 = false;
                        } else {
                            self.club.1.remove(*idx);
                        }
                    }
                    (CardSuit::Diamond, false, idx) => {
                        if *idx == 0 {
                            self.diamond.0[0].1 = false;
                        } else {
                            self.diamond.0.remove(*idx);
                        }
                    }
                    (CardSuit::Diamond, true, idx) => {
                        if *idx == 0 {
                            self.diamond.1[0].1 = false;
                        } else {
                            self.diamond.1.remove(*idx);
                        }
                    }
                }
            }
            self.round.clear();
        }

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

        target.insert(0, (c, true));
        if target.len() > 1 && !target[1].1 {
            assert!(target.len() == 2);
            target.pop();
        }
    }
}
