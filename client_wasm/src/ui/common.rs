use super::*;

pub fn draw_outer_border() {
    draw_rounded_rect(&get_canvas_rect(), BORDER_DARK);
}

pub fn draw_button(rect: &Rect, msg: &str, active: bool) {
    // warn!("draw button {} at {:?}", msg, rect);
    let color = if active {
        BUTTON
    } else {
        BUTTON_DIM
    };
    draw_rounded_rect_with_title(&rect, msg, color);
}

pub fn ui_esc_button() {
    draw_button(&ESC_BUTTON, "ESC", true);
}

pub fn ui_room_id(mut id: String) {
    if id.len() > 8 {
        let _ = id.split_off(5);
        id = format!("{}...", id);
    };
    set_font_small();
    draw_text_oneline(&ROOM_ID, &format!("room: {}", id));
    set_font_normal();
}
