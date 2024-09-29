use crate::tui::color::*;
use ratatui::{
    backend::Backend,
    layout::*,
    style::*,
    text::*,
    widgets::*,
    Frame
};
use crate::game::Card;
use super::*;
use super::card::*;
use super::gaming::*;
use super::players::*;

fn render_desk_result<B: Backend>(
    frame: &mut Frame<B>, ds: &Vec<Vec<(Card, usize)>>
) {
    let desk_rect = rect_cut_center(frame.size(), -32, -69);
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
    let rects = vec![
        rect_cut_center(rects[1], -(ds[0].len() as i16*2+6), 100),
        rect_cut_center(rects[3], -(ds[1].len() as i16*2+6), 100),
        rect_cut_center(rects[5], -(ds[2].len() as i16*2+6), 100),
        rect_cut_center(rects[7], -(ds[3].len() as i16*2+6), 100),
    ];


    let colors = vec![DESK_RESULT_0, DESK_RESULT_1, DESK_RESULT_2, DESK_RESULT_3];
    ds.iter().zip(rects).for_each(
        |(chain, ref mut a)| {
            a.height = 8;
            a.width = 11;
            for (i, (c, who)) in chain.iter().enumerate() {
                render_card(frame, c, a.clone(),
                    if i == chain.len() - 1 {
                        CardStyle::All
                    } else {
                        CardStyle::Horizontal
                    },
                    false, Some(colors[*who]));
                a.y += 2;
            }
        }
    );
}

fn render_hold_result<B: Backend>(
    frame: &mut Frame<B>, players: &Vec<(String, usize, Vec<Card>)>
) {
    // myself
    let mut a = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(11),
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
                Constraint::Min(1),
            ].as_ref()
        )
        .split(a)[1];
    render_one_player(frame, players[0].0.clone(), a, Some(DESK_RESULT_0));
    render_my_holds(frame, &players[0].2, players[0].2.len() == 0);

    //right
    let a = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Percentage(18),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(16),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(frame.size());
    let name_a = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(17),
                Constraint::Min(1),
                Constraint::Length(17),
                Constraint::Length(1),
            ].as_ref()
        )
        .split(a[1]);
    let name = Text::from(
        Span::styled(players[1].0.clone(),
        Style::default().bold().fg(NAME))
    );
    frame.render_widget(
        Paragraph::new(name)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(DESK_RESULT_1))
            ),
        name_a[3]
    );
    let sum_a = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Length(30),
                Constraint::Min(1),
                Constraint::Length(30),
                Constraint::Length(2),
            ].as_ref()
        )
        .split(a[3]);
    let sum_str = Text::from(
        Span::styled(
            format!("HOLD: {}    POINTS: {}", players[1].2.len(), hold_sum(&players[1].2)),
            Style::default().fg(HOLD_BORDER)
        )
    );
    frame.render_widget(
        Paragraph::new(sum_str)
            .alignment(Alignment::Right),
        sum_a[3]
    );
    let holds_rect = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(29),
                Constraint::Min(1),
                Constraint::Length(29),
                Constraint::Length(1),
            ].as_ref()
        )
        .split(a[5]);
    let mut holds = holds_rect[3];
    if players[1].2.len() == 0 {
        holds = rect_cut_center(holds, -8, 100);
        holds.x += 3 * (7 - 1);
        holds.width = 11;
        holds.height = 8;
        render_card(frame, &NULL_CARD, holds.clone(),
            CardStyle::Clear,
            false, Some(CARD_CLEAR_BOREDER)
        );
    } else if players[1].2.len() <= 7 {
        holds = rect_cut_center(holds, -8, 100);
        holds.x += 3 * (7 - players[1].2.len() as u16);
        holds.width = 11;
        holds.height = 8;
        for (i, c) in players[1].2.iter().enumerate() {
            render_card(frame, c, holds.clone(),
                if i == players[1].2.len() - 1 {
                    CardStyle::All
                } else {
                    CardStyle::Vertical
                },
                false, Some(MYCARD_BORDER)
            );
            holds.x += 3;
        }
    } else {
        let org_x = holds.x;
        holds.width = 11;
        holds.height = 8;
        for i in 0..=6 {
            render_card(frame, &players[1].2[i], holds.clone(),
                if i == 6 {
                    CardStyle::All
                } else {
                    CardStyle::Vertical
                },
                false, Some(MYCARD_BORDER)
            );
            holds.x += 3;
        }
        holds.y += 8;
        holds.x = org_x;
        holds.x += 3 * (14 - players[1].2.len() as u16);
        for i in 7..players[1].2.len() {
            render_card(frame, &players[1].2[i], holds.clone(),
                if i == players[1].2.len() - 1 {
                    CardStyle::All
                } else {
                    CardStyle::Vertical
                },
                false, Some(MYCARD_BORDER)
            );
            holds.x += 3;
        }
    }

    // left
    let name = Text::from(
        Span::styled(players[3].0.clone(),
        Style::default().bold().fg(NAME))
    );
    frame.render_widget(
        Paragraph::new(name)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(DESK_RESULT_3))
            ),
        name_a[1]
    );
    let sum_str = Text::from(
        Span::styled(
            format!("HOLD: {}    POINTS: {}", players[3].2.len(), hold_sum(&players[3].2)),
            Style::default().fg(HOLD_BORDER)
        )
    );
    frame.render_widget(
        Paragraph::new(sum_str)
            .alignment(Alignment::Left),
        sum_a[1]
    );
    let mut holds = holds_rect[1];
    if players[3].2.len() == 0 {
        holds = rect_cut_center(holds, -8, 100);
        holds.width = 11;
        holds.height = 8;
        render_card(frame, &NULL_CARD, holds.clone(),
            CardStyle::Clear,
            false, Some(CARD_CLEAR_BOREDER)
        );
    } else if players[3].2.len() <= 7 {
        holds = rect_cut_center(holds, -8, 100);
        holds.width = 11;
        holds.height = 8;
        for (i, c) in players[3].2.iter().enumerate() {
            render_card(frame, c, holds.clone(),
                if i == players[3].2.len() - 1 {
                    CardStyle::All
                } else {
                    CardStyle::Vertical
                },
                false, Some(MYCARD_BORDER)
            );
            holds.x += 3;
        }
    } else {
        let org_x = holds.x;
        holds.width = 11;
        holds.height = 8;
        for i in 0..=6 {
            render_card(frame, &players[3].2[i], holds.clone(),
                if i == 6 {
                    CardStyle::All
                } else {
                    CardStyle::Vertical
                },
                false, Some(MYCARD_BORDER)
            );
            holds.x += 3;
        }
        holds.y += 8;
        holds.x = org_x;
        for i in 7..players[3].2.len() {
            render_card(frame, &players[3].2[i], holds.clone(),
                if i == players[3].2.len() - 1 {
                    CardStyle::All
                } else {
                    CardStyle::Vertical
                },
                false, Some(MYCARD_BORDER)
            );
            holds.x += 3;
        }
    }

    // top
    let mut a = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(5),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(frame.size())[1];
    a = rect_cut_center(a, 100, -82);
    a.x -= frame.size().width / 25;
    let rects = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(30),
                Constraint::Length(5),
                Constraint::Length(47),
            ].as_ref()
        )
        .split(a);
    let mut holds_rect = rects[2];
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(1),
            ].as_ref()
        )
        .split(rects[0]);
    let name_a = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(17),
            ].as_ref()
        )
        .split(rects[0])[1];
    let name = Text::from(
        Span::styled(players[2].0.clone(),
        Style::default().bold().fg(NAME))
    );
    frame.render_widget(
        Paragraph::new(name)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(DESK_RESULT_2))
            ),
        name_a
    );
    let sum_str = Text::from(
        Span::styled(
            format!("HOLD: {}    POINTS: {}", players[2].2.len(), hold_sum(&players[2].2)),
            Style::default().fg(HOLD_BORDER)
        )
    );
    frame.render_widget(
        Paragraph::new(sum_str)
            .alignment(Alignment::Right),
        rects[2]
    );
    holds_rect.width = 11;
    holds_rect.height = 5;

    if players[2].2.len() == 0 {
        render_card(frame, &NULL_CARD, holds_rect.clone(),
            CardStyle::ClearHalf,
            false, Some(CARD_CLEAR_BOREDER)
        );
    } else {
        for (i, c) in players[2].2.iter().enumerate() {
            render_card(frame, c, holds_rect.clone(),
                if i == players[2].2.len() - 1 {
                    CardStyle::Half
                } else {
                    CardStyle::Vertical
                },
                false,
                Some(MYCARD_BORDER)
            );
            holds_rect.x += 3;
        }
    }
}

fn render_result_msg<B: Backend>(frame: &mut Frame<B>, msg: String, msg_color: Color){
    let mut a = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Ratio(1, 11),
            ].as_ref()
        )
        .split(frame.size())[1];
    a = rect_cut_center(a, 100, -40);

    let msg = Text::from([
        Line::styled(msg, Style::default().fg(msg_color).add_modifier(Modifier::BOLD)),
        Line::default(),
        Line::styled("\n\nPress ENTER to continue", Style::default().fg(RESULT_MSG_GREY)),
    ].to_vec());

    frame.render_widget(
        Paragraph::new(msg)
            .alignment(Alignment::Center),
        a
    );
}

fn render_result_button<B: Backend>(frame: &mut Frame<B>){
    let mut a = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Ratio(1, 11),
            ].as_ref()
        )
        .split(frame.size())[1];
    a = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(a)[0];
    a = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Length(14),
                Constraint::Percentage(6),
                Constraint::Percentage(10),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(a)[3];
    frame.render_widget(get_button("Continue", true), a);
}

fn hold_sum(holds: &Vec<Card>) -> u32 {
    holds.iter().map(
        |c| c.num
    ).sum()
}

fn name_shorten(name: &String) -> String {
    if name.len() > 8 {
        let mut ret = String::from(&name[0..6]);
        ret.push_str("..");
        ret
    } else {
        name.clone()
    }
}

pub fn ui_game_result<B: Backend>(
    frame: &mut Frame<B>, ds: &Vec<Vec<(Card, usize)>>,
    players: &Vec<(String, usize, Vec<Card>)>, roomid: &String
) {
    render_game_info(frame, roomid.clone());

    render_desk_result(frame, ds);

    render_hold_result(frame, players);

    let mut hold_sums: Vec<(usize, u32)> = players.iter().map(
        |p| hold_sum(&p.2)
    ).enumerate().collect();
    hold_sums.sort_by_key(|p| p.1);
    let mut num = 0;
    let mut me_win = 4;
    while num < 4 && hold_sums[num].1 == hold_sums[0].1 {
        if hold_sums[num].0 == 0 {
            me_win = num;
        }
        num += 1;
    }
    if me_win < 4 {
        hold_sums.swap(me_win, 0);
    }

    let (msg, color) = if me_win < 4 {
        (match num {
            1 => format!("󰱱󰱱󰱱 You win!"),
            2 => format!("󰱱󰱱󰱱 You and player {} win!", name_shorten(&players[hold_sums[1].0].0)),
            3 => format!("󰱱󰱱󰱱 You and player {}, {} win!",
                            name_shorten(&players[hold_sums[1].0].0),
                            name_shorten(&players[hold_sums[2].0].0),
                        ),
            4 => format!("Tie!"),
            _ => panic!("Invalid num!"),
        },
        RESULT_MSG_WIN)
    } else {
        (match num {
            1 => format!("󰱶󰱶󰱶 Player {} wins...", name_shorten(&players[hold_sums[0].0].0)),
            2 => format!("󰱶󰱶󰱶 Player {}, {} win...",
                            name_shorten(&players[hold_sums[0].0].0),
                            name_shorten(&players[hold_sums[1].0].0)
                        ),
            3 => format!("󰱶󰱶󰱶 The other three players win..."),
            _ => panic!("Invalid num!"),
        },
        RESULT_MSG_LOSE)
    };

    render_result_msg(frame, msg, color);
    render_result_button(frame);
}
