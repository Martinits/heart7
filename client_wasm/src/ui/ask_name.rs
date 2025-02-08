use super::*;
use super::home_page::*;

pub fn ui_ask_name(input: Input, msg: String, is_input: bool) {
    ui_prompt_window(
        input.into(),
        "Nickname".into(),
        msg,
        is_input,
        vec![("New Room".into(), true), ("Join Room".into(), true)],
    );
}
