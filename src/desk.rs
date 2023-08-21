use crate::{*, game::*};
use std::collections::{VecDeque, HashSet};

#[derive(Debug, Clone)]
pub struct Desk {
    spade:   VecDeque<(Card, u32)>,
    heart:   VecDeque<(Card, u32)>,
    club:    VecDeque<(Card, u32)>,
    diamond: VecDeque<(Card, u32)>,
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

impl Into<DeskResult> for Desk {
    fn into(self) -> DeskResult {
        DeskResult{
            spade: self.spade.into_iter().map(|c| c.into()).collect(),
            heart: self.heart.into_iter().map(|c| c.into()).collect(),
            club: self.club.into_iter().map(|c| c.into()).collect(),
            diamond: self.diamond.into_iter().map(|c| c.into()).collect(),
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
    pub fn get_desk_info(&self, thisround: &Vec<Card>) -> DeskInfo {
        DeskInfo {
            spade:   Desk::get_chain_info(thisround, &self.spade),
            heart:   Desk::get_chain_info(thisround, &self.heart),
            club:    Desk::get_chain_info(thisround, &self.club),
            diamond: Desk::get_chain_info(thisround, &self.diamond),
        }
    }

    pub fn get_chain_info(thisround: &Vec<Card>, chain: &VecDeque<(Card, u32)>)
        -> Option<ChainInfo> {
        if chain.len() == 0{
            return None;
        }

        let front: Card = chain.front().unwrap().0.clone();
        let back: Card = chain.back().unwrap().0.clone();
        let mut front_is_thisround = false;
        let mut back_is_thisround = false;

        if let Some(_) = thisround.iter().find(|&cc| *cc == front) {
            front_is_thisround = true;
        }

        if let Some(_) = thisround.iter().find(|&cc| *cc == back) {
            back_is_thisround = true;
        }

        Some(ChainInfo {
            front: Some(front.into()),
            back: Some(back.into()),
            front_is_thisround,
            back_is_thisround,
        })
    }

    // this function doesn't check whether is valid !!!
    pub fn update(&mut self, play: &Play, pid: u32) {
        if let Play::Discard(ci) = play {
            let c = Card::from_info(ci);
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

    pub fn is_valid_discard(&self, c: &Card, is_first: bool) -> bool {
        if let Some(_) = self.discard_candidates(is_first).iter().find(|&cc| cc == c) {
            true
        } else {
            false
        }
    }

    pub fn discard_candidates(&self, is_first: bool) -> HashSet<Card> {
        let mut ret = HashSet::new();
        if is_first {
            ret.insert(Card{ suit: CardSuit::Heart, num: 7});
        } else {
            get_candidates_for_one_suit!(ret, self.spade,   CardSuit::Spade);
            get_candidates_for_one_suit!(ret, self.heart,   CardSuit::Heart);
            get_candidates_for_one_suit!(ret, self.club,    CardSuit::Club);
            get_candidates_for_one_suit!(ret, self.diamond, CardSuit::Diamond);
        }
        ret
    }
}
