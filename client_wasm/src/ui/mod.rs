mod color;
mod utils;
mod home_page;
mod ask_name;
mod new_room;
mod join_room;
mod wait;
mod gaming;
mod game_result;
mod card;
mod desk;
mod exit_menu;

use crate::*;
pub(crate) use color::*;
pub(crate) use utils::*;
pub(crate) use Slice1D::*;
pub(crate) use desk::*;
pub(crate) use card::*;
pub(crate) use home_page::*;
pub(crate) use ask_name::*;
pub(crate) use new_room::*;
pub(crate) use join_room::*;
pub(crate) use wait::*;
pub(crate) use gaming::*;
pub(crate) use game_result::*;
pub(crate) use exit_menu::*;

fn draw_normal(cs: ClientState) -> JsResult<()> {
    draw_esc_button()?;

    if cs.exitmenu.0 {
        let button_num = match cs.fsm {
            ClientStateMachine::GetServer {..} | ClientStateMachine::AskName {..}
            | ClientStateMachine::JoinRoom {..} | ClientStateMachine::NewRoom { .. } => 2,
            ClientStateMachine::WaitPlayer {..} | ClientStateMachine::WaitReady {..} => 3,
            ClientStateMachine::Gaming {..} | ClientStateMachine::GameResult {..} => 4,
        };
        ui_exit_menu(button_num, cs.exitmenu.1);
    } else {
        match cs.fsm {
            ClientStateMachine::GetServer {connecting, input, msg}
                => ui_home_page(input, msg, connecting),
            ClientStateMachine::AskName {input, msg, button, is_input, ..}
                => ui_ask_name(input, msg, button, is_input),
            ClientStateMachine::NewRoom { input, msg, ..}
                => ui_new_room(input, msg),
            ClientStateMachine::JoinRoom {input, msg, ..}
                => ui_join_room(input, msg),
            ClientStateMachine::WaitPlayer {players, msg, roomid, ..}
                => ui_wait_player(players, msg, roomid),
            ClientStateMachine::WaitReady {players, msg, roomid, ..}
                => ui_wait_ready(players, msg, roomid),
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

                ui_gaming(names, hold_nums, next, roomid, choose, last,
                    my_cards, my_holds, hints, chains_small,
                    chains_big, button, has_done, msg
                );
            }
            ClientStateMachine::GameResult {ds, players, roomid, ..}
                => ui_game_result(ds, players, roomid),
        }
    }

    Ok(())
}

pub fn should_block() -> bool {
    // TODO: should_block
    false
}

fn draw_blocked(cs: ClientState) -> JsResult<()> {
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
