use async_channel::Sender;
use crate::*;
use crate::ui::*;

pub fn handle_click(
    x: f64,
    y: f64,
    tx: Sender<ClientEvent>,
    csbrief: ClientStateMachineBrief,
) -> JsResult<()> {
    match csbrief {
        ClientStateMachineBrief::GetServer => handle_click_get_server(x, y, tx)?,
        ClientStateMachineBrief::AskName => handle_click_ask_name(x, y, tx)?,
        ClientStateMachineBrief::NewRoom => handle_click_new_room(x, y, tx)?,
        ClientStateMachineBrief::JoinRoom => handle_click_join_room(x, y, tx)?,
        ClientStateMachineBrief::WaitPlayer => handle_click_wait_player(x, y, tx)?,
        ClientStateMachineBrief::WaitReady => handle_click_wait_ready(x, y, tx)?,
        ClientStateMachineBrief::Gaming => handle_click_gaming(x, y, tx)?,
        ClientStateMachineBrief::GameResult => handle_click_game_result(x, y, tx)?,
    }

    Ok(())
}

fn handle_click_get_server(x: f64, y: f64, tx: Sender<ClientEvent>,) -> JsResult<()> {
    Ok(())
}

fn handle_click_ask_name(x: f64, y: f64, tx: Sender<ClientEvent>,) -> JsResult<()> {
    Ok(())
}

fn handle_click_new_room(x: f64, y: f64, tx: Sender<ClientEvent>,) -> JsResult<()> {
    Ok(())
}

fn handle_click_join_room(x: f64, y: f64, tx: Sender<ClientEvent>,) -> JsResult<()> {
    Ok(())
}

fn handle_click_wait_player(x: f64, y: f64, tx: Sender<ClientEvent>,) -> JsResult<()> {
    Ok(())
}

fn handle_click_wait_ready(x: f64, y: f64, tx: Sender<ClientEvent>,) -> JsResult<()> {
    Ok(())
}

fn handle_click_gaming(x: f64, y: f64, tx: Sender<ClientEvent>,) -> JsResult<()> {
    Ok(())
}

fn handle_click_game_result(x: f64, y: f64, tx: Sender<ClientEvent>,) -> JsResult<()> {
    Ok(())
}
