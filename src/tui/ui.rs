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
        AppState::Gaming => {}
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
            Style::default()
                .add_modifier(
                    match selected {
                        true => Modifier::BOLD,
                        false => Modifier::DIM,
                    }
                )
                .fg(BUTTON)
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

fn render_players<B: Backend>(frame: &mut Frame<B>, names: &Vec<String>, ready: Vec<bool>) {
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
    let mut a = rect_cut_center(frame.size(), -11, 100);
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
    a = rect_cut_center(a, 100, -14);
    render_one_player(frame, names[2].clone(), a);

    // left one
    let mut a = rect_cut_center(frame.size(), -11, 100);
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
}

fn render_center_msg<B: Backend>(frame: &mut Frame<B>, msg: String) {
    frame.render_widget(
        Paragraph::new(msg)
            .style(Style::default().fg(CENTER_MSG).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center),
        rect_cut_center(frame.size(), -1, 50)
    )
}

fn wait_player<B: Backend>(
    frame: &mut Frame<B>, players: &Vec<(String, usize, bool)>,
    msg: &String, roomid: &String)
{
    render_players(frame,
        players.iter().map(|p| p.0.clone()).collect::<Vec<String>>().as_ref(),
        vec![false; 4]
    );

    render_center_msg(frame, msg.clone());

    render_game_info(frame, roomid.clone());
}

fn wait_ready<B: Backend>(
    frame: &mut Frame<B>, players: &Vec<(String, usize, bool)>,
    msg: &String, roomid: &String)
{
    render_players(frame,
        players.iter().map(|p| p.0.clone()).collect::<Vec<String>>().as_ref(),
        players.iter().map(|p| p.2).collect::<Vec<bool>>()
    );

    render_center_msg(frame, msg.clone());

    render_game_info(frame, roomid.clone());
}
