use crate::*;
use super::*;
use std::collections::{VecDeque, HashSet};

pub type ChainType = VecDeque<(Card, usize)>;

#[derive(Debug, Clone)]
pub struct Desk {
    spade:   ChainType,
    heart:   ChainType,
    club:    ChainType,
    diamond: ChainType,
}

impl Default for Desk {
    fn default() -> Self {
        Desk {
            spade:   VecDeque::with_capacity(13),
            heart:   VecDeque::with_capacity(13),
            club:    VecDeque::with_capacity(13),
            diamond: VecDeque::with_capacity(13),
        }
    }
}

macro_rules! get_candidates_for_one_suit {
    ($set:ident, $suit:expr, $cardsuit:expr) => (
        let suit = $cardsuit;
        if ($suit).is_empty() {
            $set.insert(Card{
                suit,
                num: 7,
            });
        } else {
            let front = ($suit).front().unwrap().0.num;
            if front != 1 {
                $set.insert(Card{
                    suit,
                    num: front - 1,
                });
            }
            let back = ($suit).back().unwrap().0.num;
            if back != 13 {
                $set.insert(Card{
                    suit,
                    num: back + 1,
                });
            }
        }
    )
}

impl Desk {
    pub fn clear(&mut self) {
        self.spade.clear();
        self.heart.clear();
        self.club.clear();
        self.diamond.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.spade.is_empty()
        && self.heart.is_empty()
        && self.club.is_empty()
        && self.diamond.is_empty()
    }

    pub fn get_chain(&self, cs: CardSuit) -> &ChainType {
        match cs {
            CardSuit::Spade   => &self.spade,
            CardSuit::Heart   => &self.heart,
            CardSuit::Club    => &self.club,
            CardSuit::Diamond => &self.diamond,
        }
    }

    // this function doesn't check whether is valid !!!
    pub fn play_card(&mut self, play: Play) {
        if let Play::Discard(c, pid) = play {
            let chain = match c.suit {
                CardSuit::Spade => &mut self.spade,
                CardSuit::Heart => &mut self.heart,
                CardSuit::Club => &mut self.club,
                CardSuit::Diamond => &mut self.diamond,
            };
            if c.num < 7 {
                chain.push_front((c, pid));
            } else {
                chain.push_back((c, pid));
            }
        }
    }

    fn discard_candidates(&self) -> HashSet<Card> {
        let mut ret = HashSet::new();
        if self.is_empty() {
            ret.insert(Card{ suit: CardSuit::Heart, num: 7});
        } else {
            get_candidates_for_one_suit!(ret, self.spade,   CardSuit::Spade);
            get_candidates_for_one_suit!(ret, self.heart,   CardSuit::Heart);
            get_candidates_for_one_suit!(ret, self.club,    CardSuit::Club);
            get_candidates_for_one_suit!(ret, self.diamond, CardSuit::Diamond);
        }
        ret
    }

    pub fn is_discard_candidates(&self, c: &Card) -> bool {
        let cand = self.discard_candidates();
        cand.iter().find(|&cc| cc == c).is_some()
    }

    pub fn someone_has_discard_candidates<'a>(
        &'a self, mut iter: impl Iterator<Item = &'a Card>
    ) -> bool {
        let cand = self.discard_candidates();
        iter.any(
            |c| cand.contains(c)
        )
    }

    pub fn get_desk_result(&self) -> DeskResult {
        DeskResult {
            spade: self.spade.iter().map(|c| c.into()).collect(),
            heart: self.heart.iter().map(|c| c.into()).collect(),
            club: self.club.iter().map(|c| c.into()).collect(),
            diamond: self.diamond.iter().map(|c| c.into()).collect(),
        }
    }

    pub fn export(&self) -> Vec<Vec<Card>> {
        [&self.spade, &self.heart, &self.club, &self.diamond].into_iter().map(
            |v| v.iter().map(|(c, _)| c.clone() ).collect()
        ).collect()
    }
}
