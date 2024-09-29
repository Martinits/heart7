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
pub use common::*;
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

