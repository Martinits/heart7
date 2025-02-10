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
            Some(c.clone()),
            if h {
                MYCARD_BORDER
            } else {
                MYCARD_BORDER_DIM
            }
        );

        draw_image(
            &r.center_cut(Fixed(CARD_ICON_WIDTH), Fixed(CARD_ICON_HEIGHT)),
            &get_card_image(&c).0,
        );

        if choose == i {
            r.y += MY_CARD_UP_HEIGHT;
        }
        r.x += MY_CARD_OVERLAP_WIDTH;
    }
}

fn ui_gameing_msg(msg: String) {
    // let r = get_canvas_rect().cut_width([
    //     Percent(32),
    //     Percent(43),
    // ])[1].cut_height([
    //     Percent(65),
    //     Percent(5),
    // ])[1].clone();
    // warn!("{:?}", r);

    set_font_small();
    draw_text_oneline_center_color(&GAMING_MSG, &msg, GAME_MSG);
    set_font_normal();
}

fn ui_gaming_next(next: usize) {
    // let r = match next {
    //     // myself
    //     0 => {
    //         get_canvas_rect().cut_width([
    //             Percent(18),
    //             Percent(13),
    //         ])[1].cut_height([
    //             Percent(65),
    //             Percent(5),
    //         ])[1].clone()
    //     }
    //     // right
    //     1 => {
    //         get_canvas_rect().cut_width([
    //             Percent(85),
    //             Percent(13),
    //         ])[1].cut_height([
    //             Percent(50),
    //             Percent(5),
    //         ])[1].clone()
    //     }
    //     // top
    //     2 => {
    //         get_canvas_rect().cut_width([
    //             Percent(57),
    //             Percent(13),
    //         ])[1].cut_height([
    //             Percent(8),
    //             Percent(5),
    //         ])[1].clone()
    //     }
    //     // left
    //     3 => {
    //         get_canvas_rect().cut_width([
    //             Percent(1),
    //             Percent(13),
    //         ])[1].cut_height([
    //             Percent(55),
    //             Percent(5),
    //         ])[1].clone()
    //     }
    //     _ => unreachable!(),
    // };
    // warn!("{:?}", r);
    // draw_rect(&r, BORDER_LIGHT);

    draw_text_oneline_center_color(
        &GAMING_NEXT[next],
        if next == 0 {
            "Your Turn!"
        } else {
            "Waiting..."
        },
        NEXT_TURN,
    );
}

fn ui_gaming_last(last: Option<Card>, who: usize) {
    if who == 0 {
        return;
    }

    // let r = match who {
    //     // right
    //     1 => {
    //         get_canvas_rect().cut_width([
    //             Percent(80),
    //             Fixed(22),
    //         ])[1].cut_height([
    //             Percent(35),
    //             Fixed(40),
    //         ])[1].clone()
    //     }
    //     // top
    //     2 => {
    //         get_canvas_rect().cut_width([
    //             Percent(60),
    //             Fixed(22),
    //         ])[1].cut_height([
    //             Percent(5),
    //             Fixed(40),
    //         ])[1].clone()
    //     }
    //     // left
    //     3 => {
    //         get_canvas_rect().cut_width([
    //             Percent(17),
    //             Fixed(22),
    //         ])[1].cut_height([
    //             Percent(37),
    //             Fixed(40),
    //         ])[1].clone()
    //     }
    //     _ => unreachable!(),
    // };
    // warn!("{:?}", r);

    ui_card(&GAMING_LAST[who], last, NEXT_TURN);
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

    ui_desk_hold_num(hold_nums);

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

    ui_desk_my_holds(my_holds, false);
}
