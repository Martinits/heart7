use crate::ui::color::*;
use crate::*;
use ratatui::{
    backend::Backend,
    layout::*,
    Frame,
    style::Color,
};
use crate::rule::Card;
use super::card::*;
use super::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CardStyleOnDesk {
    Normal,
    ThisRound,
    ThisRoundMy,
}

impl Into<Option<Color>> for CardStyleOnDesk {
    fn into(self) -> Option<Color> {
        match self {
            CardStyleOnDesk::Normal => None,
            CardStyleOnDesk::ThisRound => Some(CARD_HIGHLIGHT),
            CardStyleOnDesk::ThisRoundMy => Some(CARD_HIGHLIGHT_MY),
        }
    }
}

fn render_chain<B: Backend>(
    frame: &mut Frame<B>, cs: CardSuit,
    chain_small: Vec<(Card, CardStyleOnDesk)>,
    chain_big: Vec<(Card, CardStyleOnDesk)>, a: Rect
) {
    if chain_small.len() == 0 && chain_big.len() == 0 {
        //empty chain
        let a = rect_cut_center(a, -8, 100);
        render_card(frame, &Card{suit: cs, num: 1}, a, CardStyle::Empty, false, None);
    } else if chain_big.len() == 0 && chain_small.len() == 1 && chain_small[0].0.num == 7 {
        // only a seven
        let a = rect_cut_center(a, -8, 100);
        render_card(
            frame, &chain_small[0].0, a, CardStyle::All, false,
            chain_small[0].1.into()
        );
    } else if chain_big.len() == 0 {
        // only small card(s)
        let mut a = rect_cut_center(a, -8, 100);
        if chain_small.iter().last().unwrap().0.num != 7 {
            // render 7
            render_card(
                frame, &Card{suit: cs, num: 7}, a.clone(),
                CardStyle::Horizontal, false, None
            );
            // render smaller
            a.y += 2;
            for _ in 0..(7 - chain_small.iter().last().unwrap().0.num - 1) {
                render_card(frame, &NULL_CARD, a.clone(), CardStyle::Horizontal, false, None);
                a.y += 1;
            }
        }

        for i in (1..chain_small.len()).rev() {
            assert_ne!(chain_small[i].1, CardStyleOnDesk::Normal);
            render_card(
                frame, &chain_small[i].0, a, CardStyle::Horizontal, false,
                chain_small[i].1.into()
            );
            a.y += 2;
        }
        // last one
        render_card(
            frame, &chain_small[0].0, a, CardStyle::All, false,
            chain_small[0].1.into()
        );
    } else {
        // both small and big
        // calculate center 7 position
        let mut a = rect_cut_center(a, -8, 100);
        if chain_small[0].0.num + chain_big[0].0.num > 14 {
            a.y += 2;
        }
        // calculate top card position
        let big_last_num = chain_big.iter().last().unwrap().0.num;
        let big_length = chain_big.len() as u32 *2 + big_last_num - 7 - 1;
        a.y -= big_length as u16;
        //render from top
        //big highlighted
        for (ec, hi) in chain_big {
            render_card(
                frame, &ec, a.clone(), CardStyle::Horizontal, false,
                hi.into()
            );
            a.y += 2;
        }
        //big folded
        for _ in 0..(big_last_num - 7 - 1) {
            render_card(frame, &NULL_CARD, a.clone(), CardStyle::Horizontal, false, None);
            a.y += 1;
        }
        //small folded
        for _ in 0..(7 - chain_small.iter().last().unwrap().0.num) {
            render_card(frame, &NULL_CARD, a.clone(), CardStyle::Horizontal, false, None);
            a.y += 1;
        }
        //small highlighted
        for i in (1..chain_small.len()).rev() {
            assert_ne!(chain_small[i].1, CardStyleOnDesk::Normal);
            render_card(
                frame, &chain_small[i].0, a, CardStyle::Horizontal, false,
                chain_small[i].1.into()
            );
            a.y += 2;
        }
        // last one
        render_card(frame, &chain_small[0].0, a, CardStyle::All, false,
            chain_small[0].1.into()
        );
    }
}

pub fn render_desk<B: Backend>(
    frame: &mut Frame<B>,
    mut chains_small: Vec<Vec<(Card, CardStyleOnDesk)>>,
    mut chains_big: Vec<Vec<(Card, CardStyleOnDesk)>>,
) {
    // debug!("rendering Desk: {:?}", desk);
    let desk_rect = rect_cut_center(frame.size(), -24, -69);
    let rects = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(11),
                Constraint::Length(7),
                Constraint::Length(11),
                Constraint::Length(7),
                Constraint::Length(11),
                Constraint::Length(7),
                Constraint::Length(11),
                Constraint::Length(3),
            ].as_ref()
        ).split(desk_rect);

    assert_eq!(chains_small.len(), 4);
    assert_eq!(chains_big.len(), 4);
    render_chain(frame, CardSuit::Diamond, chains_small.remove(3), chains_big.remove(3), rects[7]);
    render_chain(frame, CardSuit::Club,    chains_small.remove(2), chains_big.remove(2), rects[5]);
    render_chain(frame, CardSuit::Heart,   chains_small.remove(1), chains_big.remove(1), rects[3]);
    render_chain(frame, CardSuit::Spade,   chains_small.remove(0), chains_big.remove(0), rects[1]);
}
