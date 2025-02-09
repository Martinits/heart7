use super::*;


fn ui_gaming_my_cards(my_cards: Vec<Card>, choose: usize, hints: Vec<bool>) {
    if my_cards.len() == 0 {
        return;
    }

    // let w = (my_cards.len() - 1) as f64 * MY_CARD_OVERLAP_WIDTH + MY_CARD_WIDTH;
    // let h = MY_CARD_HEIGHT + MY_CARD_UP_HEIGHT;
    // let mut r = get_canvas_rect().cut_width([
    //     Percent(33.0),
    //     Fixed(w),
    // ])[1].cut_height([
    //     Percent(75.0),
    //     Fixed(h),
    // ])[1].clone();
    // warn!("{:?}", r);

    let mut r = MY_CARD_LEFT_START.clone();
    r.w = MY_CARD_WIDTH;
    r.h = MY_CARD_HEIGHT;

    for (i, (c, h)) in my_cards.into_iter().zip(hints).into_iter().enumerate() {
        if choose == i {
            r.y -= MY_CARD_UP_HEIGHT;
        }

        ui_card(
            &r,
            &c,
            if h {
                MYCARD_BORDER
            } else {
                MYCARD_BORDER_DIM
            }
        );

        if choose == i {
            r.y += MY_CARD_UP_HEIGHT;
        }
        r.x += MY_CARD_OVERLAP_WIDTH;
    }
}

fn ui_gameing_msg(msg: String) {

}

fn ui_gaming_next(next: usize) {

}

fn ui_gaming_last(last: Option<Card>, who: usize) {

}

fn ui_gaming_hold_num(hold_nums: Vec<u32>) {

}

fn ui_gaming_my_holds(my_holds: Vec<Card>, clear: bool) {

}

fn ui_gaming_button() {
    // let r = get_canvas_rect().cut_width([
    //     Percent(19),
    //     Percent(10),
    // ])[1].cut_height([
    //     Percent(75),
    //     Percent(9),
    //     Percent(3),
    //     Percent(9),
    // ]);
    // let b0 = r[1].clone();
    // let b1 = r[3].clone();
    // warn!("{:?}", b0);
    // warn!("{:?}", b1);

    draw_button(&GAMING_BUTTON_PLAY, "Play", true);
    draw_button(&GAMING_BUTTON_HOLD, "Hold", true);
}

pub fn ui_gaming(
    names: Vec<String>, hold_nums: Vec<u32>, next: usize,
    roomid: String, choose: usize, last: Option<(usize, Option<Card>)>,
    my_cards: Vec<Card>, my_holds: Vec<Card>, hints: Vec<bool>,
    chains_small: Vec<Vec<(Card, CardStyleOnDesk)>>,
    chains_big: Vec<Vec<(Card, CardStyleOnDesk)>>,
    button: u32, has_done: bool, msg: Option<String>
) {
    ui_room_id(roomid);

    ui_players(names);

    ui_gaming_hold_num(hold_nums);

    let is_no_discard = !hints.iter().any(|b| *b);
    ui_gaming_my_cards(my_cards, choose, hints);

    ui_gaming_button();

    if let Some(m) = msg {
        ui_gameing_msg(m.clone());
    } else if next == 0 && is_no_discard {
        ui_gameing_msg("No Card to Play!".into());
    }

    if !has_done {
        ui_gaming_next(next);
    }

    if let Some((who, last)) = last {
        assert_eq!((next+3)%4, who);
        ui_gaming_last(last, who);
    }

    ui_gaming_my_holds(my_holds, false);
}
