use super::*;

pub fn ui_join_room(input: Input, msg: String) {
    ui_prompt_window(
        input.into(),
        "Room ID".into(),
        msg,
        true,
        [("Join Room!".into(), true)].to_vec(),
    );
}
