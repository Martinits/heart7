use async_channel::Sender;
use crate::*;
use crate::ui::*;

pub fn handle_click(
    x: f64,
    y: f64,
    tx: Sender<ClientEvent>,
    csbrief: ClientStateBrief,
) -> JsResult<()> {
    // log!("Clicked! {}, {}", x, y);
    if csbrief.exitmenu.0 {
        handle_click_exit_menu(csbrief.fsm, x, y, tx)?;
    } else if !handle_click_esc_button(x, y, tx.clone())? {
        match csbrief.fsm {
            ClientStateMachineBrief::GetServer => handle_click_get_server(x, y, tx)?,
            ClientStateMachineBrief::AskName => handle_click_ask_name(x, y, tx)?,
            ClientStateMachineBrief::NewRoom => handle_click_new_room(x, y, tx)?,
            ClientStateMachineBrief::JoinRoom => handle_click_join_room(x, y, tx)?,
            ClientStateMachineBrief::WaitPlayer => handle_click_wait_player(x, y, tx)?,
            ClientStateMachineBrief::WaitReady => handle_click_wait_ready(x, y, tx)?,
            ClientStateMachineBrief::Gaming => handle_click_gaming(x, y, tx)?,
            ClientStateMachineBrief::GameResult => handle_click_game_result(x, y, tx)?,
        }
    }

    Ok(())
}

fn handle_click_exit_menu(
    fsm: ClientStateMachineBrief,
    x: f64, y: f64, tx: Sender<ClientEvent>,
) -> JsResult<()> {
    Ok(())
}

// return true if handled as esc button
fn handle_click_esc_button(x: f64, y: f64, tx: Sender<ClientEvent>) -> JsResult<bool> {
    Ok(false)
}

fn handle_click_get_server(x: f64, y: f64, tx: Sender<ClientEvent>) -> JsResult<()> {
    if PROMPT_INPUT.is_clicked_in(x, y) {
        get_hidden_input().focus()?;
    } else {
        get_hidden_input().blur()?;
        if HOME_PAGE_BUTTON_GO.is_clicked_in(x, y) {
            spawn_tx_send(tx, ClientEvent::Enter);
        }
    }
    Ok(())
}

fn handle_click_ask_name(x: f64, y: f64, tx: Sender<ClientEvent>) -> JsResult<()> {
    Ok(())
}

fn handle_click_new_room(x: f64, y: f64, tx: Sender<ClientEvent>) -> JsResult<()> {
    Ok(())
}

fn handle_click_join_room(x: f64, y: f64, tx: Sender<ClientEvent>) -> JsResult<()> {
    Ok(())
}

fn handle_click_wait_player(x: f64, y: f64, tx: Sender<ClientEvent>) -> JsResult<()> {
    Ok(())
}

fn handle_click_wait_ready(x: f64, y: f64, tx: Sender<ClientEvent>) -> JsResult<()> {
    Ok(())
}

fn handle_click_gaming(x: f64, y: f64, tx: Sender<ClientEvent>) -> JsResult<()> {
    Ok(())
}

fn handle_click_game_result(x: f64, y: f64, tx: Sender<ClientEvent>) -> JsResult<()> {
    Ok(())
}
