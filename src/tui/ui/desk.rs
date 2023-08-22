use crate::tui::color::*;
use crate::*;
use ratatui::{
    backend::Backend,
    layout::*,
    Frame,
    style::Color,
};
use crate::game::Card;
use crate::client::desk::*;
use super::common::*;
use super::card::*;

fn get_card_highlight(sign: u32) -> Option<Color> {
    match sign {
        0 => None,
        1 => Some(CARD_HIGHLIGHT),
        2 => Some(CARD_HIGHLIGHT_MY),
        _ => panic!("Invalid sign!"),
    }
}

fn render_chain<B: Backend>(frame: &mut Frame<B>, cs: CardSuit,
    chain_small: &Vec<(Card, u32)>, chain_big: &Vec<(Card, u32)>, a: Rect
) {
    if chain_small.len() == 0 && chain_big.len() == 0 {
        //empty chain
        let a = rect_cut_center(a, -8, 100);
        render_card(frame, &Card{suit: cs, num: 1}, a, CardAppearance::Empty, false, None);
    } else if chain_big.len() == 0 && chain_small.len() == 1 && chain_small[0].0.num == 7 {
        // only a seven
        let a = rect_cut_center(a, -8, 100);
        render_card(frame, &chain_small[0].0, a, CardAppearance::All, false,
            get_card_highlight(chain_small[0].1)
        );
    } else if chain_big.len() == 0 {
        // only small card(s)
        // render 7
        let mut a = rect_cut_center(a, -8, 100);
        render_card(frame, &Card{suit: cs, num: 7}, a.clone(), CardAppearance::Horizontal, false, None);
        // render smaller
        a.y += 2;
        for _ in 0..(7 - chain_small.iter().last().unwrap().0.num - 1) {
            render_card(frame, &NULL_CARD, a.clone(), CardAppearance::Horizontal, false, None);
            a.y += 1;
        }
        let mut csmall = chain_small.clone();
        csmall.reverse();
        for i in 0..csmall.len() - 1 {
            assert!(csmall[i].1 != 0);
            render_card(frame, &csmall[i].0, a, CardAppearance::Horizontal, false,
                get_card_highlight(csmall[i].1)
            );
            a.y += 2;
        }
        // last one
        render_card(frame, &chain_small[0].0, a, CardAppearance::All, false,
            get_card_highlight(chain_small[0].1)
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
        for (ec, hi) in chain_big.iter() {
            render_card(frame, ec, a.clone(), CardAppearance::Horizontal, false,
                get_card_highlight(*hi)
            );
            a.y += 2;
        }
        //big folded
        for _ in 0..(big_last_num - 7 - 1) {
            render_card(frame, &NULL_CARD, a.clone(), CardAppearance::Horizontal, false, None);
            a.y += 1;
        }
        //small folded
        for _ in 0..(7 - chain_small.iter().last().unwrap().0.num) {
            render_card(frame, &NULL_CARD, a.clone(), CardAppearance::Horizontal, false, None);
            a.y += 1;
        }
        //small highlighted
        let mut csmall = chain_small.clone();
        csmall.reverse();
        for i in 0..csmall.len() - 1 {
            assert!(csmall[i].1 != 0);
            render_card(frame, &csmall[i].0, a, CardAppearance::Horizontal, false,
                get_card_highlight(csmall[i].1)
            );
            a.y += 2;
        }
        // last one
        render_card(frame, &chain_small[0].0, a, CardAppearance::All, false,
            get_card_highlight(chain_small[0].1)
        );
    }
}

pub fn render_desk<B: Backend>(frame: &mut Frame<B>, desk: &Desk) {
    // debug!("rendering Desk: {:?}", desk);
    let desk_rect = rect_cut_center(frame.size(), -24, -70);
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

    render_chain(frame, CardSuit::Spade,   desk.spade.0.as_ref(), desk.spade.1.as_ref(), rects[1]);
    render_chain(frame, CardSuit::Heart,   desk.heart.0.as_ref(), desk.heart.1.as_ref(), rects[3]);
    render_chain(frame, CardSuit::Club,    desk.club.0.as_ref(), desk.club.1.as_ref(), rects[5]);
    render_chain(frame, CardSuit::Diamond, desk.diamond.0.as_ref(), desk.diamond.1.as_ref(), rects[7]);
}
