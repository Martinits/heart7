use super::*;

fn ui_prompt_window(
    input: Input,
    input_title: String,
    msg: String,
    input_selected: bool,
    buttons: Vec<(String, bool)>, // msg, selected
) {
    // let win_rect = get_canvas_rect().center_cut(Percent(80), Percent(70));
    draw_rounded_rect(&PROMPT_WINDOW, BORDER_NORMAL);

    // let slices = win_rect.cut_height([
    //     Percent(20),
    //     Percent(15),
    //     Percent(20),
    //     Percent(10),
    //     Percent(10),
    // ]);

    draw_paragraph(&PROMPT_MSG, &msg);

    let input_color = if input_selected {
        INPUT_BORDER
    } else {
        INPUT_BORDER_BLOCK
    };
    // draw_input(&slices[2].center_cut_width(Percent(60)), input, input_title, input_color);
    draw_input(input, input_title, input_color);

    if buttons.len() == 1 {
        // let r = &slices[4].center_cut_width(Percent(23));
        // warn!("Rect {:?}", r);
        // draw_button(r, &buttons[0].0, buttons[0].1);
        draw_button(&PROMPT_BUTTON_1, &buttons[0].0, buttons[0].1);
    } else if buttons.len() == 2 {
        // let slices = &slices[4].cut_width([
        //     Percent(25),
        //     Percent(20),
        //     Percent(10),
        //     Percent(20),
        //     Percent(25),
        // ]);
        // warn!("Rect {:?}", &slices[1]);
        // warn!("Rect {:?}", &slices[3]);

        draw_button(&PROMPT_BUTTON_2[0], &buttons[0].0, buttons[0].1);
        draw_button(&PROMPT_BUTTON_2[1], &buttons[1].0, buttons[1].1);
    } else {
        panic!("Invalid button number!");
    }
}

fn draw_input_text(rect: &Rect, t: &str) {
    draw_text_oneline_with_descent(rect, t, get_ascii_max_descent());
}

fn draw_cursor(rect: Rect) {
    get_canvas_ctx().fill_rect(rect.x, rect.y, rect.w, rect.h);
}

// fn draw_input(rect: &Rect, input: String, input_title: String, input_color: &str) {
fn draw_input(input: Input, input_title: String, input_color: &str) {
    // PROMPT_INPUT
    // let (_, input_text_h) = get_text_metric(input);
    // let input_h = input_text_h * 2.8;
    // let input_rect = rect.center_cut_height(Fixed(input_h));
    // warn!("input_rect {:?}", input_rect);

    draw_rect(&PROMPT_INPUT, input_color);

    // border height = input_text_h * 0.8
    // let input_text_rect = PROMPT_INPUT.cut_border(Percent(2.0), Percent(0.9/2.8 * 100f64));
    // warn!("{:?}", input_text_rect);
    draw_input_text(&PROMPT_INPUT_TEXT, input.value());

    let (title_w, title_h) = get_text_metric(&input_title);
    // let h = PROMPT_INPUT.h + title_h / 2f64;
    // if rect.h < h {
    //     warn!("Try to draw_input of height {} inside rect of height {}", rect.h, h);
    // }

    let title_rect = Rect {
        x: PROMPT_INPUT.x + PROMPT_INPUT.width_slice(Percent(3)),
        y: PROMPT_INPUT.y - title_h * 0.5,
        w: title_w,
        h: title_h,
    };
    clear_rect(&title_rect);

    draw_text_oneline(&title_rect, &input_title);

    // cursor
    if hidden_input_is_focused() {
        let cursor = input.cursor();
        let cursor_rect = Rect {
            x: PROMPT_INPUT_TEXT.x + get_text_metric(input.value().split_at(cursor).0).0,
            y: PROMPT_INPUT_TEXT.y,
            w: 1.0,
            h: PROMPT_INPUT_TEXT.h,
        };
        draw_cursor(cursor_rect);
    }
}

pub fn ui_home_page(input: Input, msg: String, connecting: bool) {
    ui_prompt_window(
        input.into(),
        "IP:PORT".into(),
        msg,
        !connecting,
        [("GO!".into(), !connecting)].to_vec(),
    );
}

pub fn ui_ask_name(input: Input, msg: String, is_input: bool) {
    ui_prompt_window(
        input.into(),
        "Nickname".into(),
        msg,
        is_input,
        vec![("New Room".into(), true), ("Join Room".into(), true)],
    );
}

pub fn ui_new_room(input: Input, msg: String) {
    ui_prompt_window(
        input.into(),
        "Room ID".into(),
        msg,
        true,
        [("Create Room!".into(), true)].to_vec(),
    );
}

pub fn ui_join_room(input: Input, msg: String) {
    ui_prompt_window(
        input.into(),
        "Room ID".into(),
        msg,
        true,
        [("Join Room!".into(), true)].to_vec(),
    );
}
