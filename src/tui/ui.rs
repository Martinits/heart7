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
        AppState::GetRoom {input, msg, button, ..}
            => ask_name(frame, input, msg, button),
        AppState::JoinRoom {input, msg, ..}
            => join_room(frame, input, msg),
        AppState::WaitPlayer {name, msg, ..}
            => wait_player(frame, name, msg),
        AppState::WaitReady => {}
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
                        msg: &String, button: &u16,
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
            .style(Style::default().fg(INPUT_BORDER))
            .scroll((0, scroll as u16))
            .block(Block::default().borders(Borders::ALL).title("Nickname")),
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
    frame.render_widget(get_button("New Room", *button == 0), buttons[1]);
    frame.render_widget(get_button("Join Room", *button == 1), buttons[3]);
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

fn wait_player<B: Backend>(frame: &mut Frame<B>, name: &String, msg: &String) {
    
}
