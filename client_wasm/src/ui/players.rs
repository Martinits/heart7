use super::*;

pub fn ui_one_player(mut r: Rect, name: &str) {
    draw_rounded_rect(&r, CARD_SIGN);

    let shift = r.width_slice(Percent(10));
    r.shift(-shift, shift);
    clear_rect(&r);
    draw_rounded_rect(&r, CARD_SIGN);

    r.shift(-shift, shift);
    clear_rect(&r);
    set_font_small();
    let name = if name.len() == 0 {
        "???"
    } else if get_text_metric(name).0 >= r.w {
        &format!("{}..", name.split_at(5).0)
    } else {
        name
    };
    draw_rounded_rect_with_title(&r, name, CARD_SIGN);
    set_font_normal();
}

pub fn ui_players(names: Vec<String>) {
    // myself
    // let r = get_canvas_rect().cut_height([
    //     Percent(75),
    //     Percent(15),
    // ])[1].cut_width([
    //     Percent(5),
    //     Percent(10),
    // ])[1].clone();
    // warn!("{:?}", r);
    ui_one_player(PLAYER_MYSELF.clone(), &names[0]);

    // right
    // let r = get_canvas_rect().cut_height([
    //     Percent(25),
    //     Percent(12),
    // ])[1].cut_width([
    //     Percent(87),
    //     Percent(10),
    //     Percent(3),
    // ])[1].clone();
    // warn!("{:?}", r);
    ui_one_player(PLAYER_RIGHT.clone(), &names[1]);

    // top
    // let r = get_canvas_rect().cut_height([
    //     Percent(3),
    //     Percent(12),
    // ])[1].cut_width([
    //     Percent(30),
    //     Percent(10),
    // ])[1].clone();
    // warn!("{:?}", r);
    ui_one_player(PLAYER_TOP.clone(), &names[2]);

    // left
    // let r = get_canvas_rect().cut_height([
    //     Percent(30),
    //     Percent(12),
    // ])[1].cut_width([
    //     Percent(3),
    //     Percent(10),
    // ])[1].clone();
    // warn!("{:?}", r);
    ui_one_player(PLAYER_LEFT.clone(), &names[3]);
}
