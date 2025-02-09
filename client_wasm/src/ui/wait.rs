use super::*;

fn ui_center_msg(msg: Vec<String>) {
    // let r = get_canvas_rect().center_cut(Percent(50), Percent(15));
    // draw_rect(&r, BORDER_NORMAL);
    // warn!("{:?}", r);
    draw_paragraph_vec(&WAIT_CENTER_MSG, msg);
}

fn ui_ready_button(active: bool) {
    // let r = get_canvas_rect().center_cut_width(Percent(18));
    // let slices = r.cut_height([
    //     Percent(80),
    //     Percent(8),
    //     Percent(12),
    // ]);
    // warn!("{:?}", &slices[1]);

    draw_button(&WAIT_READY_BUTTON, "Get Ready!", active);
}

pub fn ui_wait_player(
    players: Vec<(String, usize, bool)>,
    msg: Vec<String>, roomid: String,
){
    ui_room_id(roomid);

    ui_center_msg(msg);

    ui_players(players.iter().map(|p| p.0.clone()).collect());

    ui_ready_button(false);
}

fn ui_players_ready(ready: Vec<bool>) {
    // // myself
    // let myself_r = &WAIT_READY_BUTTON.center_cut_width(Percent(60));
    // // right
    // let right_r = &get_canvas_rect().cut_width([
    //     Percent(85),
    //     Percent(10),
    // ])[1].center_cut_height(Percent(5));
    // // top
    // let top_r = &get_canvas_rect().center_cut_width(Percent(10)).cut_height([
    //     Percent(10),
    //     Percent(5),
    // ])[1];
    // // left
    // let left_r = &get_canvas_rect().cut_width([
    //     Percent(2),
    //     Percent(10),
    // ])[1].cut_height([
    //     Percent(52),
    //     Percent(5),
    // ])[1];
    // let r = vec![myself_r, right_r, top_r, left_r];
    // warn!("{:?}", r);

    let r = &WAIT_PLAYER_READY;
    ready.into_iter().zip(r.into_iter()).for_each(
        |(ready, r)| if ready {
            draw_text_oneline_center_color(r, "READY!", READY);
        }
    )
}

pub fn ui_wait_ready(
    players: Vec<(String, usize, bool)>,
    msg: Vec<String>, roomid: String,
) {
    ui_room_id(roomid);

    ui_center_msg(msg);

    ui_players(players.iter().map(|p| p.0.clone()).collect());

    ui_players_ready(players.iter().map(|p| p.2.clone()).collect());

    if !players[0].2 {
        ui_ready_button(true);
    }
}
