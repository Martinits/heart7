mod color;
mod utils;
mod prompt_win;
mod wait;
mod gaming;
mod game_result;
mod card;
mod desk;
mod exit_menu;
mod layout;
mod players;
mod common;

use crate::*;
pub(crate) use color::*;
pub(crate) use utils::*;
pub(crate) use Slice1D::*;
pub(crate) use desk::*;
pub(crate) use card::*;
pub(crate) use prompt_win::*;
pub(crate) use wait::*;
pub(crate) use gaming::*;
pub(crate) use game_result::*;
pub(crate) use exit_menu::*;
pub(crate) use layout::*;
pub(crate) use players::*;
pub(crate) use common::*;

fn draw_normal(cs: ClientState) -> JsResult<()> {
    ui_esc_button();

    if cs.exitmenu.0 {
        ui_exit_menu(get_button_num(&cs));
    } else {
        match cs.fsm {
            ClientStateMachine::GetServer {connecting, input, msg} => {
                hidden_input_reset(input.value());
                ui_home_page(input, msg, connecting);
            }
            ClientStateMachine::AskName {input, msg, is_input, ..} => {
                hidden_input_reset(input.value());
                ui_ask_name(input, msg, is_input);
            }
            ClientStateMachine::NewRoom { input, msg, ..} => {
                hidden_input_reset(input.value());
                ui_new_room(input, msg);
            }
            ClientStateMachine::JoinRoom {input, msg, ..} => {
                hidden_input_reset(input.value());
                ui_join_room(input, msg);
            }
            ClientStateMachine::WaitPlayer {players, msg, roomid, ..}
                => ui_wait_player(players, msg, roomid),
            ClientStateMachine::WaitReady {players, msg, roomid, ..}
                => ui_wait_ready(players, msg, roomid),
            ClientStateMachine::Gaming {
                choose, mut game, roomid, msg, ..
            } => {
                let names = game.get_player_names();
                let hold_nums = game.get_hold_nums();
                let next = game.get_next();
                let last = game.get_last();
                let my_cards = game.get_my_cards();
                let my_holds = game.get_my_holds();
                let hints = game.get_my_hint();
                let has_done = game.has_done();
                let desk = game.export_desk();

                ui_gaming(names, hold_nums, next, roomid, choose, last,
                    my_cards, my_holds, hints, desk, has_done, msg
                );
            }
            ClientStateMachine::GameResult {ds, players, roomid, winner, winner_state, ..} => {
                let names = players.iter().map(|(n, _)| n.clone()).collect();
                let holds = players.into_iter().map(|(_, h)| h).collect();
                let desk = ds.into_iter().map(|c| {
                    c.into_iter().rev().map(|(c, _)| c).collect()
                }).collect();
                ui_game_result(desk, names, holds, roomid, winner, winner_state);
            }
        }
    }

    Ok(())
}

pub fn should_block() -> bool {
    // TODO: should_block
    false
}

fn draw_blocked(_cs: ClientState) -> JsResult<()> {
    Ok(())
}

pub fn draw(cs: ClientState) -> JsResult<()> {
    clear_canvas();

    draw_outer_border();

    if should_block() {
        draw_blocked(cs)?;
    } else {
        draw_normal(cs)?;
    }

    Ok(())
}

pub fn ui_init() -> JsResult<()> {
    // hiddent input init
    // let hidden_input = get_hidden_input();
    // hidden_input.set_value(&init_input_value);
    get_hidden_input().blur()?;

    // font init
    set_font_normal();

    Ok(())
}

pub fn hidden_input_reset(new_value: &str) {
    get_hidden_input().set_value(&new_value);
}

pub fn hidden_input_focus() {
    get_hidden_input().focus().unwrap_throw();
}

pub fn hidden_input_blur() {
    get_hidden_input().blur().unwrap_throw();
}

pub fn hidden_input_is_focused() -> bool {
    if let Some(e) = gloo::utils::document().active_element() {
        e.id() == "hidden-input"
    } else {
        false
    }
}
