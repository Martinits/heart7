use async_channel::Sender;
use crate::*;
use crate::ui::*;

pub fn handle_click(
    x: f64,
    y: f64,
    tx: Sender<ClientEvent>,
    csbrief: ClientStateBrief,
) -> JsResult<()> {
    // warn!("Clicked! {}, {}", x, y);
    if handle_click_esc_button(x, y, tx.clone())? {
        // pass
    } else if csbrief.exitmenu.0 {
        handle_click_exit_menu(get_button_num_from_brief(&csbrief), csbrief.exitmenu.1, x, y, tx)?;
    } else {
        match csbrief.fsm {
            ClientStateMachineBrief::GetServer => handle_click_get_server(x, y, tx)?,
            ClientStateMachineBrief::AskName{button, is_input}
                => handle_click_ask_name(x, y, tx, button, is_input)?,
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
    button_num: u32, which: u32,
    x: f64, y: f64, tx: Sender<ClientEvent>,
) -> JsResult<()> {
    let mut buttons = match button_num {
        2 => EM_BUTTON_2.iter(),
        3 => EM_BUTTON_3.iter(),
        4 => EM_BUTTON_4.iter(),
        _ => panic!("Invalid buttom nums!"),
    };
    if let Some(clicked) = buttons.position(|b| b.is_clicked_in(x, y)) {
        let dis = clicked as i32 - which as i32;
        let e = if dis > 0 {
            ClientEvent::DownArrow
        } else {
            ClientEvent::UpArrow
        };
        let mut payload = vec![e; dis.abs() as usize];
        payload.push(ClientEvent::Enter);
        spawn_tx_send_multiple(tx, payload);
    }
    Ok(())
}

// return true if handled as esc button
fn handle_click_esc_button(x: f64, y: f64, tx: Sender<ClientEvent>) -> JsResult<bool> {
    let ret = if ESC_BUTTON.is_clicked_in(x, y) {
        spawn_tx_send(tx, ClientEvent::Esc);
        true
    } else {
        false
    };
    Ok(ret)
}

fn handle_click_prompt_single_button(x: f64, y: f64, tx: Sender<ClientEvent>) -> JsResult<()> {
    if PROMPT_INPUT.is_clicked_in(x, y) {
        get_hidden_input().focus()?;
    } else {
        get_hidden_input().blur()?;
        if PROMPT_BUTTON_1.is_clicked_in(x, y) {
            spawn_tx_send(tx, ClientEvent::Enter);
        }
    }
    Ok(())
}

fn handle_click_get_server(x: f64, y: f64, tx: Sender<ClientEvent>) -> JsResult<()> {
    handle_click_prompt_single_button(x, y, tx)
}

fn handle_click_ask_name(x: f64, y: f64, tx: Sender<ClientEvent>, button: u16, is_input: bool) -> JsResult<()> {
    if PROMPT_INPUT.is_clicked_in(x, y) {
        get_hidden_input().focus()?;
    } else {
        get_hidden_input().blur()?;
        if let Some(clicked) = PROMPT_BUTTON_2.iter().position(|b| b.is_clicked_in(x, y)) {
            let mut payload = vec![];
            if is_input {
                payload.push(ClientEvent::DownArrow);
            }
            if clicked != button as usize {
                payload.push(ClientEvent::LeftArrow);
            }
            payload.push(ClientEvent::Enter);
            spawn_tx_send_multiple(tx, payload);
        }
    }
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
