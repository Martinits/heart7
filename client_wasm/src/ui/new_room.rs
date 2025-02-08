use super::*;

pub fn ui_new_room(input: Input, msg: String) {
    ui_prompt_window(
        input.into(),
        "Room ID".into(),
        msg,
        true,
        [("Create Room!".into(), true)].to_vec(),
    );
}
