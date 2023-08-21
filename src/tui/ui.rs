use super::app::AppState;
use super::color::*;
use crate::*;
use ratatui::{
    backend::Backend,
    layout::*,
    style::*,
    widgets::*,
    text::*,
    Frame
};
use tui_input::Input;
use crate::game::Card;
use crate::client::desk::*;

pub fn render<B: Backend>(frame: &mut Frame<B>, appstate: &AppState) {
    // outer border
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(BORDER_DARK)),
        frame.size(),
    );

    match appstate {
        AppState::GetServer {connecting, input, msg}
            => home_page(frame, input, msg, connecting),
        AppState::GetRoom {input, msg, button, is_input, ..}
            => ask_name(frame, input, msg, button, is_input),
        AppState::JoinRoom {input, msg, ..}
            => join_room(frame, input, msg),
        AppState::WaitPlayer {players, msg, roomid, ..}
            => wait_player(frame, players, msg, roomid),
        AppState::WaitReady {players, msg, roomid, ..}
            => wait_ready(frame, players, msg, roomid),
        AppState::Gaming {
            players, next, choose, last, cards, holds,
            has_last, desk, roomid, button, ..
        } => gaming(frame, players, *next, roomid, *choose, last.as_ref(), cards,
                holds, *has_last, desk, roomid, *button),
        AppState::GameResult => {}
    }
}

// cut out the center of `org` with v and h
// v and h: negative for fixed value, positive for percentage
// num: how many aeras to cut, negative for vertical
fn rect_cut_center(mut org: Rect, v: i16, h: i16) -> Rect {
    if v < 0 {
        let v = (-v) as u16;
        let up = (org.height - v) / 2;
        let down = org.height - up - v;
        org = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(up),
                    Constraint::Length(v),
                    Constraint::Length(down),
                ]
                .as_ref(),
            )
            .split(org)[1];
    } else if v < 100 {
        let v = v as u16;
        let up = (100 - v) / 2;
        let down = 100 - up - v;
        org = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(up),
                    Constraint::Percentage(v),
                    Constraint::Percentage(down),
                ]
                .as_ref(),
            )
            .split(org)[1];
    }

    if h < 0 {
        let h = (-h) as u16;
        let left = (org.width - h) / 2;
        let right = org.width - left - h;
        org = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(left),
                    Constraint::Length(h),
                    Constraint::Length(right),
                ]
                .as_ref(),
            )
            .split(org)[1];
    } else if h < 100 {
        let h = h as u16;
        let left = (100 - h) / 2;
        let right = 100 - left - h;
        org = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(left),
                    Constraint::Percentage(h),
                    Constraint::Percentage(right),
                ]
                .as_ref(),
            )
            .split(org)[1];
    }
    org
}

fn prompt_rect(frame_size: Rect) -> Rect {
    rect_cut_center(frame_size, 40, 50)
}

fn render_prompt<B: Backend>(frame: &mut Frame<B>) -> Rect {
    let prompt = prompt_rect(frame.size());

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(BORDER_NORMAL)),
        prompt.clone(),
    );

    prompt
}

fn get_button(cmd: &str, selected: bool) -> Paragraph {
    Paragraph::new(cmd)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .style(
            match selected {
                true => Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(BUTTON),
                false => Style::default()
                    .add_modifier(Modifier::DIM)
                    .fg(BUTTON_DIM),
            }
        )
}

fn home_page<B: Backend>(frame: &mut Frame<B>, input: &Input, msg: &String, connecting: &bool) {
    let prompt = render_prompt(frame);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Min(3),
            ]
            .as_ref(),
        )
        .split(prompt);

    frame.render_widget(
        Paragraph::new(msg.as_str())
        .alignment(Alignment::Center)
        .style(Style::default().fg(TEXT_NORMAL).bold()),
        chunks[0],
    );

    let input_rect = rect_cut_center(chunks[1], -3, 60);
    let input_width = input_rect.width.max(3) - 3;
    let scroll = input.visual_scroll(input_width as usize);
    frame.render_widget(
        Paragraph::new(input.value())
            .style(Style::default().fg(
                if *connecting {
                    INPUT_BORDER_BLOCK
                } else {
                    INPUT_BORDER
                }
            ))
            .scroll((0, scroll as u16))
            .block(Block::default().borders(Borders::ALL).title("IP:PORT")),
        input_rect,
    );
    if !connecting {
        frame.set_cursor(
            // Put cursor past the end of the input text
            input_rect.x
                + ((input.visual_cursor()).max(scroll) - scroll) as u16
                + 1,
            // Move one line down, from the border to the input line
            input_rect.y + 1,
        );
    }

    let button_rect = rect_cut_center(chunks[2], -3, 20);
    frame.render_widget(get_button("GO!", !connecting), button_rect);
}

fn ask_name<B: Backend>(frame: &mut Frame<B>, input: &Input,
                        msg: &String, button: &u16, is_input: &bool
) {
    let prompt = render_prompt(frame);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Min(3),
            ]
            .as_ref(),
        )
        .split(prompt);

    frame.render_widget(
        Paragraph::new(msg.as_str())
        .alignment(Alignment::Center)
        .style(Style::default().fg(TEXT_NORMAL).bold()),
        chunks[0],
    );

    let input_rect = rect_cut_center(chunks[1], -3, 60);
    let input_width = input_rect.width.max(3) - 3;
    let scroll = input.visual_scroll(input_width as usize);
    frame.render_widget(
        Paragraph::new(input.value())
            .style(Style::default().fg(
                if *is_input {
                    INPUT_BORDER
                } else {
                    INPUT_BORDER_BLOCK
                }
            ))
            .scroll((0, scroll as u16))
            .block(Block::default().borders(Borders::ALL).title("Nickname")),
        input_rect,
    );
    if *is_input {
        frame.set_cursor(
            // Put cursor past the end of the input text
            input_rect.x
                + ((input.visual_cursor()).max(scroll) - scroll) as u16
                + 1,
            // Move one line down, from the border to the input line
            input_rect.y + 1,
        );
    }

    let button_line = rect_cut_center(chunks[2], -3, 100);
    let buttons = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(20),
                Constraint::Percentage(10),
                Constraint::Percentage(20),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(button_line);
    frame.render_widget(get_button("New Room", !*is_input && *button == 0), buttons[1]);
    frame.render_widget(get_button("Join Room", !*is_input && *button == 1), buttons[3]);
}

fn join_room<B: Backend>(frame: &mut Frame<B>, input: &Input, msg: &String) {
    let prompt = render_prompt(frame);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Min(3),
            ]
            .as_ref(),
        )
        .split(prompt);

    frame.render_widget(
        Paragraph::new(msg.as_str())
        .alignment(Alignment::Center)
        .style(Style::default().fg(TEXT_NORMAL).bold()),
        chunks[0],
    );

    let input_rect = rect_cut_center(chunks[1], -3, 60);
    let input_width = input_rect.width.max(3) - 3;
    let scroll = input.visual_scroll(input_width as usize);
    frame.render_widget(
        Paragraph::new(input.value())
            .style(Style::default().fg(INPUT_BORDER))
            .scroll((0, scroll as u16))
            .block(Block::default().borders(Borders::ALL).title("Room ID")),
        input_rect,
    );
    frame.set_cursor(
        // Put cursor past the end of the input text
        input_rect.x
            + ((input.visual_cursor()).max(scroll) - scroll) as u16
            + 1,
        // Move one line down, from the border to the input line
        input_rect.y + 1,
    );

    let button_rect = rect_cut_center(chunks[2], -3, 20);
    frame.render_widget(get_button("Join Room!", true), button_rect);
}

fn render_game_info<B: Backend>(frame: &mut Frame<B>, roomid: String) {
    frame.render_widget(
        Paragraph::new(format!("ROOM-ID: {}", roomid))
            .alignment(Alignment::Left)
            .style(Style::default().fg(NORMAL_DIM).add_modifier(Modifier::DIM)),
        Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Min(1),
                ].as_ref()
            )
            .split(frame.size())[0]
    )
}

fn render_one_player<B:Backend>(frame: &mut Frame<B>, name: String, a: Rect){
    // card sign
    let (width, height) = (a.width - 2, a.height - 2);
    let blocks = [
        Rect::new(a.x + 2, a.y,     width, height),
        Rect::new(a.x + 1, a.y + 1, width, height),
        Rect::new(a.x    , a.y + 2, width, height),
    ];
    for b in blocks.into_iter() {
        frame.render_widget(Clear, b);
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(CARD_SIGN)),
            b
        );
    }

    // name
    let (name, lines) = if name.len() > 9 {
        (format!("{}\n...", &name[0..10]), 2)
    } else {
        (name, 1)
    };
    frame.render_widget(
        Paragraph::new(
            Text::from(
                if name == "" {
                    Span::styled(
                        "???",
                        Style::default().bold().fg(NAME_DIM).add_modifier(Modifier::DIM)
                    )
                } else {
                    Span::styled(
                        name,
                        Style::default().bold().fg(NAME)
                    )
                }
            )
        )
        .alignment(Alignment::Center),
        rect_cut_center(blocks[2].inner(&Margin{vertical: 1, horizontal: 1}), -lines, 100)
    )
}

fn render_ready<B: Backend>(frame: &mut Frame<B>, a: Rect) {
    frame.render_widget(
        Paragraph::new("READY!")
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(READY)
                    .add_modifier(Modifier::BOLD)
            ),
        a
    )
}

fn render_hold<B: Backend>(frame: &mut Frame<B>, a: Rect, num: u32) {
    frame.render_widget(
        Paragraph::new(format!("HOLD: {}", num))
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(HOLD_NUM)
                    .add_modifier(Modifier::BOLD)
            ),
        a
    )
}

fn render_players<B: Backend>(frame: &mut Frame<B>, names: &Vec<String>,
    ready: Vec<bool>, holds: Option<Vec<u32>>
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
                Constraint::Percentage(17),
                Constraint::Length(14),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(a)[1];
    render_one_player(frame, names[0].clone(), a);

    // right one
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
    a = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(14),
            ].as_ref()
        )
        .split(a)[1];
    render_one_player(frame, names[1].clone(), a);

    // top one
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
    a = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Length(14),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(a)[1];
    render_one_player(frame, names[2].clone(), a);

    // left one
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
    a = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Length(14),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(a)[0];
    render_one_player(frame, names[3].clone(), a);

    // ready
    // myself
    if ready[0] {
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
        a = rect_cut_center(a, -3, 20);
        render_ready(frame, a);
    }

    // right one
    if ready[1] {
        let mut a = rect_cut_center(frame.size(), -3, 100);
        a = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Min(1),
                    Constraint::Percentage(10),
                    Constraint::Length(20),
                ].as_ref()
            )
            .split(a)[1];
        render_ready(frame, a);
    }

    // top one
    if ready[2] {
        let mut a = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .constraints(
                [
                    Constraint::Length(13),
                    Constraint::Min(1),
                ].as_ref()
            )
            .split(frame.size())[0];
        a = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(10),
                    Constraint::Min(1),
                ].as_ref()
            )
            .split(a)[1];
        a = rect_cut_center(a, -3, 100);
        render_ready(frame, a);
    }

    // left one
    if ready[3] {
        let mut a = rect_cut_center(frame.size(), -3, 100);
        a = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Length(20),
                    Constraint::Percentage(10),
                    Constraint::Min(1),
                ].as_ref()
            )
            .split(a)[1];
        render_ready(frame, a);
    }

    // hold num
    if let Some(holds) = holds {
        // right
        let mut a = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Length(13),
                    Constraint::Length(3),
                    Constraint::Min(1)
                ].as_ref()
            )
            .split(frame.size())[2];
        a = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Min(1),
                    Constraint::Length(16),
                ].as_ref()
            )
            .split(a)[1];
        render_hold(frame, a, holds[1]);

        // top
        let mut a = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .constraints(
                [
                    Constraint::Length(13),
                    Constraint::Min(1),
                ].as_ref()
            )
            .split(frame.size())[0];
        a = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(10),
                    Constraint::Min(1),
                ].as_ref()
            )
            .split(a)[1];
        a = rect_cut_center(a, -3, 100);
        render_hold(frame, a, holds[2]);

        // left
        let mut a = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Length(13),
                    Constraint::Length(3),
                    Constraint::Min(1)
                ].as_ref()
            )
            .split(frame.size())[2];
        a = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Length(12),
                    Constraint::Min(1),
                ].as_ref()
            )
            .split(a)[0];
        render_hold(frame, a, holds[3]);
    }
}

fn render_center_msg<B: Backend>(frame: &mut Frame<B>, msg: String) {
    frame.render_widget(
        Paragraph::new(msg)
            .style(Style::default().fg(CENTER_MSG).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center),
        rect_cut_center(frame.size(), -1, 50)
    )
}

fn render_ready_button<B: Backend>(frame: &mut Frame<B>, active: bool) {
    let mut button = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(11),
            ].as_ref()
        )
        .split(frame.size())[1];
    button = rect_cut_center(button, -3, 20);

    frame.render_widget(get_button("Get Ready!", active), button);
}

fn wait_player<B: Backend>(
    frame: &mut Frame<B>, players: &Vec<(String, usize, bool)>,
    msg: &String, roomid: &String)
{
    render_players(frame,
        players.iter().map(|p| p.0.clone()).collect::<Vec<String>>().as_ref(),
        vec![false; 4], None
    );

    render_center_msg(frame, msg.clone());

    render_game_info(frame, roomid.clone());

    render_ready_button(frame, false);
}

fn wait_ready<B: Backend>(
    frame: &mut Frame<B>, players: &Vec<(String, usize, bool)>,
    msg: &String, roomid: &String)
{
    render_players(frame,
        players.iter().map(|p| p.0.clone()).collect::<Vec<String>>().as_ref(),
        players.iter().map(|p| p.2).collect::<Vec<bool>>(), None
    );

    render_center_msg(frame, msg.clone());

    render_game_info(frame, roomid.clone());

    if !players[0].2 {
        render_ready_button(frame, true);
    }
}

enum CardAppearance {
    All,
    Vertical,
    Horizontal,
    Empty,
}

fn get_card_text(card: &Card) -> (String, String) {
    (
        match card.suit {
            CardSuit::Spade => "󰣑", //󱢲
            CardSuit::Heart => "󰣐", //󱢠
            CardSuit::Club => "󰣎", //󱢟
            CardSuit::Diamond => "󰣏", //󱀝
        }.into(),
        match card.num {
            1 => "󰫮".into(),
            2 => "󰬻".into(),
            3 => "󰬼".into(),
            4 => "󰬽".into(),
            5 => "󰬾".into(),
            6 => "󰬿".into(),
            7 => "󰭀".into(),
            8 => "󰭁".into(),
            9 => "󰭂".into(),
            10 => "󰿩".into(),
            11 => "󰫷".into(),
            12 => "󰫾".into(),
            13 => "󰫸".into(),
            _ => panic!("Invalid card num!")
        }
    )
}

fn render_card<B: Backend>(
    frame: &mut Frame<B>, card: &Card, a: Rect, ca: CardAppearance,
    highlight: Option<Color>
) {
    let block_style = if let Some(c) = highlight {
        Style::default().fg(c).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(CARD_BORDER)
    };

    let card_suit_style = match card.suit {
        CardSuit::Spade => Style::default().fg(SPADE),
        CardSuit::Heart => Style::default().fg(HEART),
        CardSuit::Club => Style::default().fg(CLUB),
        CardSuit::Diamond => Style::default().fg(DIAMOND),
    };

    let (text_suit, text_num) = get_card_text(card);
    let text = match ca {
        CardAppearance::All => {
            Text::from(
                [
                    Line::styled(text_num.clone(), card_suit_style),
                    Line::styled(
                        format!("{}   {}", text_suit.clone(), text_suit.clone()),
                        card_suit_style
                    ),
                    Line::styled("", card_suit_style),
                    Line::styled("", card_suit_style),
                    Line::styled(text_suit.clone(), card_suit_style).alignment(Alignment::Center),
                ].to_vec()
            )
        }
        CardAppearance::Vertical => {
            Text::from(
                [
                    Line::styled(text_num.clone(), card_suit_style),
                    Line::styled(text_suit.clone(), card_suit_style),
                ].to_vec()
            )
        }
        CardAppearance::Horizontal => {
            Text::from(
                Line::styled(format!("{} {}", text_num.clone(), text_suit.clone()), card_suit_style),
            )
        }
        CardAppearance::Empty => {
            Text::from(
                [
                    Line::styled("", card_suit_style),
                    Line::styled(text_suit.clone(), card_suit_style).alignment(Alignment::Center),
                    Line::styled("", card_suit_style),
                    Line::styled("", card_suit_style),
                    Line::styled(text_suit.clone(), card_suit_style).alignment(Alignment::Center),
                ].to_vec()
            )
        }
    };

    // clear first
    frame.render_widget(Clear, a);

    frame.render_widget(
        Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(block_style)
            ),
        a
    )
}

const NULL_CARD: Card = Card { suit: CardSuit::Spade, num: 1};

fn render_chain<B: Backend>(frame: &mut Frame<B>, cs: CardSuit,
    chain_small: &Vec<(Card, bool)>, chain_big: &Vec<(Card, bool)>, a: Rect
) {
    if chain_small.len() == 0 && chain_big.len() == 0 {
        //empty chain
        let a = rect_cut_center(a, -8, 100);
        render_card(frame, &Card{suit: cs, num: 1}, a, CardAppearance::Empty, None);
    } else if chain_big.len() == 0 && chain_small.len() == 1 && chain_small[0].0.num == 7 {
        // only a seven
        let a = rect_cut_center(a, -8, 100);
        render_card(frame, &chain_small[0].0, a, CardAppearance::All,
            if chain_small[0].1 { Some(CARD_HIGHLIGHT) } else { None }
        );
    } else if chain_big.len() == 0 {
        // only small card(s)
        // render 7
        let mut a = rect_cut_center(a, -8, 100);
        render_card(frame, &Card{suit: cs, num: 7}, a.clone(), CardAppearance::Horizontal, None);
        // render smaller
        a.y += 2;
        for _ in 0..(7 - chain_small.iter().last().unwrap().0.num - 1) {
            render_card(frame, &NULL_CARD, a.clone(), CardAppearance::Horizontal, None);
            a.y += 1;
        }
        let mut csmall = chain_small.clone();
        csmall.reverse();
        for i in 0..csmall.len() - 1 {
            assert!(csmall[i].1);
            render_card(frame, &csmall[i].0, a, CardAppearance::Horizontal,
                if csmall[i].1 { Some(CARD_HIGHLIGHT) } else { None }
            );
            a.y += 2;
        }
        // last one
        render_card(frame, &chain_small[0].0, a, CardAppearance::All,
            if chain_small[0].1 { Some(CARD_HIGHLIGHT) } else { None }
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
            render_card(frame, ec, a.clone(), CardAppearance::Horizontal,
                if *hi { Some(CARD_HIGHLIGHT) } else { None }
            );
            a.y += 2;
        }
        //big folded
        for _ in 0..(big_last_num - 7 - 1) {
            render_card(frame, &NULL_CARD, a.clone(), CardAppearance::Horizontal, None);
            a.y += 1;
        }
        //small folded
        for _ in 0..(7 - chain_small.iter().last().unwrap().0.num) {
            render_card(frame, &NULL_CARD, a.clone(), CardAppearance::Horizontal, None);
            a.y += 1;
        }
        //small highlighted
        let mut csmall = chain_small.clone();
        csmall.reverse();
        for i in 0..csmall.len() - 1 {
            assert!(csmall[i].1);
            render_card(frame, &csmall[i].0, a, CardAppearance::Horizontal,
                if csmall[i].1 { Some(CARD_HIGHLIGHT) } else { None }
            );
            a.y += 2;
        }
        // last one
        render_card(frame, &chain_small[0].0, a, CardAppearance::All,
            if chain_small[0].1 { Some(CARD_HIGHLIGHT) } else { None }
        );
    }
}

fn render_desk<B: Backend>(frame: &mut Frame<B>, desk: &Desk) {
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

fn render_my_cards<B: Backend>(frame: &mut Frame<B>, cards: &Vec<Card>, choose: usize) {
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
                Constraint::Percentage(17),
                Constraint::Length(14),
                Constraint::Percentage(10),
                Constraint::Length(39),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(a)[3];
    a = rect_cut_center(a, 100, -(cards.len() as i16 *3));
    a.width = 11;
    a.height = 8;

    let mut cards = cards.clone();
    cards.sort();
    for (i, c) in cards.iter().enumerate() {
        if i+1 == choose {
            a.y -= 1;
        }
        render_card(frame, c, a.clone(),
            if i == cards.len()- 1 {
                CardAppearance::All
            } else {
                CardAppearance::Vertical
            },
            Some(MYCARD_BORDER)
        );
        if i+1 == choose {
            a.y += 1;
        }
        a.x += 3;
    }
}

fn render_next<B: Backend>(frame: &mut Frame<B>, next: usize) {

}

fn render_last<B: Backend>(frame: &mut Frame<B>, last: Option<&Card>) {

}

fn render_my_holds<B: Backend>(frame: &mut Frame<B>, holds: &Vec<Card>) {

}

fn render_game_button<B: Backend>(frame: &mut Frame<B>, button: u32) {

}

fn gaming<B: Backend>(
    frame: &mut Frame<B>, players: &Vec<(String, usize, u32)>, next: usize, roomid: &String,
    choose: usize, last: Option<&Card>, cards: &Vec<Card>, holds: &Vec<Card>,
    has_last: bool, desk: &Desk, roomif: &String, button: u32
) {
    render_players(frame,
        players.iter().map(|p| p.0.clone()).collect::<Vec<String>>().as_ref(),
        vec![false; 4], Some(players.iter().map(|p| p.2).collect())
    );

    render_game_info(frame, roomid.clone());

    render_desk(frame, desk);

    render_my_cards(frame, cards, choose);

    render_next(frame, next);
    if has_last {
        render_last(frame, last);
    }

    render_my_holds(frame, holds);

    render_game_button(frame, button);
}
