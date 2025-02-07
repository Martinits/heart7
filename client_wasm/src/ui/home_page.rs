use super::*;

pub fn ui_prompt_window(
    input: String,
    input_title: String,
    msg: String,
    input_selected: bool,
    buttons: Vec<(String, bool)>, // msg, selected
) {
    let win_rect = get_canvas_rect().center_cut(Percent(80), Percent(70));
    draw_rounded_rect(&win_rect, BORDER_NORMAL);

    let slices = win_rect.cut_height([
        Percent(20),
        Percent(15),
        Percent(20),
        Percent(10),
        Percent(10),
    ]);

    draw_paragraph(&slices[1], &msg);

    let input_color = if input_selected {
        INPUT_BORDER
    } else {
        INPUT_BORDER_BLOCK
    };
    draw_input(&slices[2].center_cut_width(Percent(60)), input, input_title, input_color);

    if buttons.len() == 1 {
        draw_button(&slices[4].center_cut_width(Percent(20)), &buttons[0].0, buttons[0].1);
    } else if buttons.len() == 2 {
        let slices = &slices[4].cut_width([
            Percent(25),
            Percent(20),
            Percent(15),
            Percent(20),
            Percent(25),
        ]);
        draw_button(&slices[1], &buttons[0].0, buttons[0].1);
        draw_button(&slices[3], &buttons[1].0, buttons[1].1);
    }
}

fn draw_input(rect: &Rect, input: String, input_title: String, input_color: &str) {
    let (_, input_text_h) = get_text_metric(&input);
    let input_h = input_text_h * 3.2;

    let input_rect = rect.center_cut_height(Fixed(input_h));
    draw_rect(&input_rect, input_color);

    let input_text_rect = input_rect.cut_border(Percent(2.0), Fixed(input_text_h * 1.0));
    draw_text_oneline(&input_text_rect, &input);

    let (title_w, title_h) = get_text_metric(&input_title);
    let h = input_h + title_h / 2f64;
    if rect.h < h {
        warn!("Try to draw_input of height {} inside rect of height {}", rect.h, h);
    }

    let title_rect = Rect {
        x: input_rect.x + input_rect.width_slice(Percent(3)),
        y: input_rect.y - title_h * 0.5,
        w: title_w,
        h: title_h,
    };
    clear_rect(&title_rect);

    draw_text_oneline(&title_rect, &input_title);
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
