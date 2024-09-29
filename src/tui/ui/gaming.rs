use crate::tui::color::*;
use ratatui::{
    backend::Backend,
    layout::*,
    style::*,
    widgets::*,
    Frame
};
use crate::game::Card;
use crate::client::desk::*;
use super::*;
use super::card::*;
use super::players::*;
use super::desk::*;

fn render_my_cards<B: Backend>(frame: &mut Frame<B>, cards: &Vec<Card>,
    choose: usize, hints: Vec<bool>
) {
    let mut a = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(9),
                Constraint::Length(1),
            ].as_ref()
        )
        .split(frame.size())[1];
    a = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Length(14),
                Constraint::Percentage(5),
                Constraint::Length(47),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(a)[3];
    a = rect_cut_center(a, 100, -(cards.len() as i16 *3 + 8));
    a.y += 1;
    a.width = 11;
    a.height = 8;

    for (i, c) in cards.iter().enumerate() {
        if i+1 == choose {
            a.y -= 1;
        }
        render_card(frame, c, a.clone(),
            if i == cards.len()- 1 {
                CardStyle::All
            } else {
                CardStyle::Vertical
            },
            !hints[i],
            if hints[i] { Some(MYCARD_BORDER) } else { Some(MYCARD_BORDER_DIM) }
        );
        if i+1 == choose {
            a.y += 1;
        }
        a.x += 3;
    }
}

fn render_next<B: Backend>(frame: &mut Frame<B>, next: usize) {
    let a = match next {
        // myself
        0 => {
            let a = Layout::default()
                .direction(Direction::Vertical)
                .vertical_margin(1)
                .constraints(
                    [
                        Constraint::Min(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(7),
                        Constraint::Length(2),
                    ].as_ref()
                )
                .split(frame.size())[1];
            Layout::default()
                .direction(Direction::Horizontal)
                .horizontal_margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Length(14),
                        Constraint::Percentage(3),
                        Constraint::Percentage(9),
                        Constraint::Min(1),
                    ].as_ref()
                )
                .split(a)[3]
        }
        // right
        1 => {
            let mut a = Layout::default()
                .direction(Direction::Vertical)
                .vertical_margin(1)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Length(11),
                        Constraint::Min(1)
                    ].as_ref()
                )
                .split(frame.size())[1];
            a = rect_cut_center(a, -1, 100);
            Layout::default()
                .direction(Direction::Horizontal)
                .horizontal_margin(1)
                .constraints(
                    [
                        Constraint::Min(1),
                        Constraint::Length(12),
                        Constraint::Percentage(5),
                        Constraint::Length(14),
                    ].as_ref()
                )
                .split(a)[1]
                }
        // top
        2 => {
            let mut a = Layout::default()
                .direction(Direction::Vertical)
                .vertical_margin(1)
                .constraints(
                    [
                        Constraint::Length(11),
                        Constraint::Min(1),
                    ].as_ref()
                )
                .split(frame.size())[0];
            a = rect_cut_center(a, -1, 100);
            Layout::default()
                .direction(Direction::Horizontal)
                .horizontal_margin(1)
                .constraints(
                    [
                        Constraint::Percentage(40),
                        Constraint::Percentage(10),
                        Constraint::Percentage(3),
                        Constraint::Length(12),
                        Constraint::Min(1),
                    ].as_ref()
                )
                .split(a)[3]
        }
        // left
        3 => {
            let mut a = Layout::default()
                .direction(Direction::Vertical)
                .vertical_margin(1)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Length(11),
                        Constraint::Min(1)
                    ].as_ref()
                )
                .split(frame.size())[1];
            a = rect_cut_center(a, -1, 100);
            Layout::default()
                .direction(Direction::Horizontal)
                .horizontal_margin(1)
                .constraints(
                    [
                        Constraint::Length(14),
                        Constraint::Percentage(5),
                        Constraint::Length(12),
                        Constraint::Min(1),
                    ].as_ref()
                )
                .split(a)[2]
        }
        _ => panic!("next is not in 0..=3 !"),
    };

    frame.render_widget(
        Paragraph::new(
            if next == 0 {
                "Your Turn!"
            } else {
                "Waiting..."
            }
        ).alignment(Alignment::Center)
        .style(Style::default().fg(NEXT_TURN).add_modifier(Modifier::BOLD)),
        a
    );
}

fn render_last<B: Backend>(frame: &mut Frame<B>, last: Option<&Card>, who: usize) {
    let a = match who {
        // myself
        0 => {return}
        // right
        1 => {
            let mut a = Layout::default()
                .direction(Direction::Vertical)
                .vertical_margin(1)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Length(11),
                        Constraint::Min(1)
                    ].as_ref()
                )
                .split(frame.size())[1];
            a = rect_cut_center(a, -8, 100);
            Layout::default()
                .direction(Direction::Horizontal)
                .horizontal_margin(1)
                .constraints(
                    [
                        Constraint::Min(1),
                        Constraint::Length(11),
                        Constraint::Percentage(5),
                        Constraint::Length(14),
                    ].as_ref()
                )
                .split(a)[1]
                }
        // top
        2 => {
            let mut a = Layout::default()
                .direction(Direction::Vertical)
                .vertical_margin(1)
                .constraints(
                    [
                        Constraint::Length(11),
                        Constraint::Min(1),
                    ].as_ref()
                )
                .split(frame.size())[0];
            a = rect_cut_center(a, -8, 100);
            Layout::default()
                .direction(Direction::Horizontal)
                .horizontal_margin(1)
                .constraints(
                    [
                        Constraint::Percentage(40),
                        Constraint::Percentage(10),
                        Constraint::Percentage(3),
                        Constraint::Length(11),
                        Constraint::Min(1),
                    ].as_ref()
                )
                .split(a)[3]
        }
        // left
        3 => {
            let mut a = Layout::default()
                .direction(Direction::Vertical)
                .vertical_margin(1)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Length(11),
                        Constraint::Min(1)
                    ].as_ref()
                )
                .split(frame.size())[1];
            a = rect_cut_center(a, -8, 100);
            Layout::default()
                .direction(Direction::Horizontal)
                .horizontal_margin(1)
                .constraints(
                    [
                        Constraint::Length(14),
                        Constraint::Percentage(5),
                        Constraint::Length(11),
                        Constraint::Min(1),
                    ].as_ref()
                )
                .split(a)[2]
        }
        _ => panic!("next is not in 0..=3 !"),
    };

    if let Some(c) = last {
        // discard
        render_card(frame, c, a, CardStyle::All, false, Some(NEXT_TURN));
    } else {
        // hold
        render_card(frame, &NULL_CARD, a, CardStyle::Hold, false, Some(NEXT_TURN));
    }
}

pub fn render_my_holds<B: Backend>(frame: &mut Frame<B>, holds: &Vec<Card>, clear: bool) {
    let mut a = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Length(13),
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(13),
            ].as_ref()
        )
        .split(frame.size())[4];
    a = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(45),
            ].as_ref()
        )
        .split(a)[1];

    let mut hold_points = 0;
    holds.iter().for_each(|c| hold_points += c.num);
    frame.render_widget(
        Paragraph::new(format!("HOLD: {}      POINTS: {}", holds.len(), hold_points))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
            )
            .style(Style::default().fg(HOLD_BORDER)),
        a
    );

    // cards
    a = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(8),
                Constraint::Length(1),
            ].as_ref()
        )
        .split(a)[2];

    if clear {
        a = rect_cut_center(a, -8, -11);
        render_card(frame, &NULL_CARD, a,
            CardStyle::Clear, false, Some(CARD_CLEAR_BOREDER));
        return;
    }

    let mut needed_width = holds.len() as u16 *3 + 8;
    let mut might_overflow = false;
    if needed_width > a.width {
        might_overflow = true;
        needed_width -= 8;
    }
    a = rect_cut_center(a, 100, -(needed_width as i16));
    a.width = 11;
    a.height = 8;

    for (i, c) in holds.iter().enumerate() {
        if might_overflow {
            a.width = 4;
        }
        render_card(frame, c, a.clone(),
            if !might_overflow && i == holds.len() - 1 {
                CardStyle::All
            } else {
                CardStyle::Vertical
            },
            false,
            Some(MYCARD_BORDER)
        );
        a.x += 3;
        a.width = 11;
    }
}

fn render_game_button<B: Backend>(frame: &mut Frame<B>, button: u32) {
    let mut a = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(7),
                Constraint::Length(2),
            ].as_ref()
        )
        .split(frame.size())[1];
    a = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Length(14),
                Constraint::Percentage(3),
                Constraint::Percentage(9),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(a)[3];
    let buttons = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(3),
            ].as_ref()
        )
        .split(a);

    frame.render_widget(get_button("Play", button == 0), buttons[0]);
    frame.render_widget(get_button("Hold", button == 1), buttons[2]);
}

fn render_msg<B: Backend>(frame: &mut Frame<B>, msg: String) {
    let mut a = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(9),
                Constraint::Length(1),
            ].as_ref()
        )
        .split(frame.size())[1];
    a = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Length(14),
                Constraint::Percentage(5),
                Constraint::Length(47),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(a)[3];

    frame.render_widget(
        Paragraph::new(msg)
            .alignment(Alignment::Center)
            .style(Style::default().fg(GAME_MSG).add_modifier(Modifier::BOLD)),
        a
    );
}

pub fn ui_gaming<B: Backend>(
    frame: &mut Frame<B>, players: &Vec<(String, usize, u32)>, next: usize, roomid: &String,
    choose: usize, last: Option<&Card>, cards: &Vec<Card>, holds: &Vec<Card>,
    has_last: bool, desk: &Desk, button: u32, play_cnt: u32, msg: Option<&String>
) {
    render_players(frame,
        players.iter().map(|p| p.0.clone()).collect::<Vec<String>>().as_ref(),
        vec![false; 4], Some(players.iter().map(|p| p.2).collect())
    );

    render_game_info(frame, roomid.clone());

    render_desk(frame, desk);

    let hints = desk.get_play_hint(cards);
    let is_no_discard = !hints.iter().any(|b| *b);
    render_my_cards(frame, cards, choose, hints);

    if let Some(m) = msg {
        render_msg(frame, m.clone());
    } else if next == 0 && is_no_discard {
        render_msg(frame, "No Card to Play!".into());
    }

    if play_cnt < 54 {
        render_next(frame, next);
    }

    if has_last {
        render_last(frame, last, (next+3)%4);
    }

    render_my_holds(frame, holds, false);

    render_game_button(frame,
        if next == 0 {
            button
        } else {
            10 // anything not 0 or 1
        }
    );
}
