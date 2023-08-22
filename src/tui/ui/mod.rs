pub mod home_page;
pub mod ask_name;
pub mod join_room;
pub mod common;
pub mod card;
pub mod desk;
pub mod gaming;
pub mod wait;
pub mod players;

pub use home_page::*;
pub use ask_name::*;
pub use join_room::*;
pub use common::*;
pub use card::*;
pub use desk::*;
pub use gaming::*;
pub use wait::*;
pub use players::*;

use super::app::AppState;
use super::color::*;
use ratatui::{
    backend::Backend,
    style::*,
    widgets::*,
    Frame
};

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
            has_last, desk, roomid, button, play_cnt, msg, ..
        } => gaming(frame, players, *next, roomid, *choose, last.as_ref(), cards,
                holds, *has_last, desk, *button, *play_cnt, msg.as_ref()),
        AppState::GameResult => {}
    }
}

