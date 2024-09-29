pub mod home_page;
pub mod ask_name;
pub mod join_room;
pub mod common;
pub mod card;
pub mod desk;
pub mod gaming;
pub mod wait;
pub mod players;
pub mod game_result;
pub mod resize;
pub mod exit_menu;
pub mod new_room;

pub use home_page::*;
pub use ask_name::*;
pub use join_room::*;
pub use card::*;
pub use desk::*;
pub use gaming::*;
pub use wait::*;
pub use players::*;
pub use game_result::*;
pub use resize::*;
pub use exit_menu::*;
pub use new_room::*;

use crate::client::ClientState;
use super::color::*;
use ratatui::{
    backend::Backend,
    layout::*,
    style::*,
    widgets::*,
    Frame
};

pub fn render<B: Backend>(frame: &mut Frame<B>, cs: &ClientState, exit: (bool, u32)) {
    // outer border
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(BORDER_DARK)),
        frame.size(),
    );

    if exit.0 {
        let button_num = match cs {
            ClientState::GetServer {..} | ClientState::AskName {..}
            | ClientState::JoinRoom {..} | ClientState::NewRoom { .. } => 2,
            ClientState::WaitPlayer {..} | ClientState::WaitReady {..} => 3,
            ClientState::Gaming {..} | ClientState::GameResult {..} => 4,
        };
        render_exit_menu(frame, button_num, exit.1);
    } else {
        match cs {
            ClientState::GetServer {connecting, input, msg}
                => ui_home_page(frame, input, msg, connecting),
            ClientState::AskName {input, msg, button, is_input, ..}
                => ui_ask_name(frame, input, msg, button, is_input),
            ClientState::NewRoom { input, msg, ..}
                => new_room(frame, input, msg),
            ClientState::JoinRoom {input, msg, ..}
                => ui_join_room(frame, input, msg),
            ClientState::WaitPlayer {players, msg, roomid, ..}
                => ui_wait_player(frame, players, msg, roomid),
            ClientState::WaitReady {players, msg, roomid, ..}
                => ui_wait_ready(frame, players, msg, roomid),
            ClientState::Gaming {
                players, next, choose, last, cards, holds,
                has_last, desk, roomid, button, play_cnt, msg, ..
            } => ui_gaming(frame, players, *next, roomid, *choose, last.as_ref(), cards,
                    holds, *has_last, desk, *button, *play_cnt, msg.as_ref()),
            ClientState::GameResult {ds, players, roomid, ..}
                => ui_game_result(frame, ds, players, roomid),
        }
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

fn render_prompt_window<B: Backend>(frame: &mut Frame<B>) -> Rect {
    let prompt = rect_cut_center(frame.size(), 40, 50);

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
