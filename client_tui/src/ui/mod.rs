pub mod home_page;
pub mod ask_name;
pub mod join_room;
pub mod card;
pub mod desk;
pub mod gaming;
pub mod wait;
pub mod players;
pub mod game_result;
pub mod blocked;
pub mod exit_menu;
pub mod new_room;
pub mod color;

pub use home_page::*;
pub use ask_name::*;
pub use join_room::*;
pub use desk::*;
pub use gaming::*;
pub use wait::*;
pub use game_result::*;
pub use blocked::*;
pub use exit_menu::*;
pub use new_room::*;
use crate::*;
pub use color::*;
use ratatui::{
    backend::Backend,
    layout::*,
    style::*,
    widgets::*,
    Frame
};

pub fn render<B: Backend>(frame: &mut Frame<B>, cs: ClientState) {
    // outer border
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(BORDER_DARK)),
        frame.size(),
    );

    if cs.exitmenu.0 {
        render_exit_menu(frame, get_button_num(&cs), cs.exitmenu.1);
    } else {
        match cs.fsm {
            ClientStateMachine::GetServer {connecting, input, msg}
                => ui_home_page(frame, input, msg, connecting),
            ClientStateMachine::AskName {input, msg, button, is_input, ..}
                => ui_ask_name(frame, input, msg, button, is_input),
            ClientStateMachine::NewRoom { input, msg, ..}
                => ui_new_room(frame, input, msg),
            ClientStateMachine::JoinRoom {input, msg, ..}
                => ui_join_room(frame, input, msg),
            ClientStateMachine::WaitPlayer {players, msg, roomid, ..}
                => ui_wait_player(frame, players, msg, roomid),
            ClientStateMachine::WaitReady {players, msg, roomid, ..}
                => ui_wait_ready(frame, players, msg, roomid),
            ClientStateMachine::Gaming {
                choose, mut game, roomid, button, msg, ..
            } => {
                let names = game.get_player_names();
                let hold_nums = game.get_hold_nums();
                let next = game.get_next();
                let last = game.get_last();
                let my_cards = game.get_my_cards();
                let my_holds = game.get_my_holds();
                let hints = game.get_my_hint();
                let has_done = game.has_done();
                let thisround = game.get_thisround();
                let thisround_my = game.get_thisround_my();
                let mut chains_small = vec![];
                let mut chains_big = vec![];
                game.export_desk().into_iter().for_each(
                    |l| {
                        let mut small = vec![];
                        let mut big = vec![];
                        for c in l {
                            if c.num <= 7 {
                                small.push(c);
                            } else {
                                big.push(c);
                            }
                        }
                        big.reverse();
                        for (v, chain) in [(small, &mut chains_small), (big, &mut chains_big)] {
                            chain.push(if v.len() == 0 {
                                Vec::new()
                            } else if !thisround.contains(&v[0]) {
                                vec![(v[0].clone(), CardStyleOnDesk::Normal)]
                            } else {
                                let mut viter = v.into_iter();
                                let mut ret = vec![];
                                while let Some(c) = viter.next() {
                                    if !thisround.contains(&c) {
                                        break;
                                    }
                                    ret.push(
                                        (c.clone(),
                                         if thisround_my.is_some()
                                            && thisround_my.as_ref().unwrap().clone() == c {
                                            CardStyleOnDesk::ThisRoundMy
                                         } else {
                                            CardStyleOnDesk::ThisRound
                                        })
                                    );
                                }
                                ret
                            });
                        }

                    }
                );

                ui_gaming(frame, names, hold_nums, next, roomid, choose, last,
                    my_cards, my_holds, hints, chains_small,
                    chains_big, button, has_done, msg
                );
            }
            ClientStateMachine::GameResult {ds, players, roomid, winner, winner_state, ..}
                => ui_game_result(frame, ds, players, roomid, winner, winner_state),
        }
    }
}

// cut out the center of `org` with v and h
// v and h: negative for fixed value, positive for percentage
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
