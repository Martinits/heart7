use super::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CardStyleOnDesk {
    Normal,
    ThisRound,
    ThisRoundMy,
}

impl Into<Option<&str>> for CardStyleOnDesk {
    fn into(self) -> Option<&'static str> {
        match self {
            CardStyleOnDesk::Normal => None,
            CardStyleOnDesk::ThisRound => Some(CARD_HIGHLIGHT),
            CardStyleOnDesk::ThisRoundMy => Some(CARD_HIGHLIGHT_MY),
        }
    }
}

pub fn ui_desk_hold_num(hold_nums: Vec<u32>) {
    // let r = [
    //     // right
    //     get_canvas_rect().cut_width([
    //         Percent(85),
    //         Percent(13),
    //     ])[1].cut_height([
    //         Percent(43),
    //         Percent(5),
    //     ])[1].clone(),
    //
    //     // top
    //     get_canvas_rect().cut_width([
    //         Percent(42),
    //         Percent(13),
    //     ])[1].cut_height([
    //         Percent(8),
    //         Percent(5),
    //     ])[1].clone(),
    //
    //     // left
    //     get_canvas_rect().cut_width([
    //         Percent(1),
    //         Percent(13),
    //     ])[1].cut_height([
    //         Percent(48),
    //         Percent(5),
    //     ])[1].clone(),
    // ];

    for (r, hn) in DESK_HOLD_NUM.into_iter().zip(hold_nums).skip(1) {
        // warn!("{:?}", r);
        // draw_rect(&r, BORDER_LIGHT);
        draw_text_oneline_center_color(
            &r,
            &format!("HOLD: {}", hn),
            HOLD_NUM,
        );
    }
}

pub fn ui_desk_my_holds(mut my_holds: Vec<Card>, clear: bool) {
    // border
    // let border = get_canvas_rect().cut_width([
    //     Percent(74),
    //     Percent(25),
    // ])[1].cut_height([
    //     Percent(65),
    //     Percent(33),
    // ])[1].clone();
    // warn!("{:?}", border);

    draw_rounded_rect(&DESK_MY_HOLD_BORDER, HOLD_BORDER);

    // let r = border.cut_height([
    //     Percent(2),
    //     Percent(19),
    //     Percent(79),
    // ])[1].clone();
    // warn!("{:?}", r);

    // title
    let points = if clear {
            0
    } else {
        my_holds.iter().fold(0, |acc, c| acc + c.num)
    };
    let hn = if clear {
        0
    } else {
        my_holds.len()
    };
    let hold_title = format!("HOLD: {}  POINTS: {}", hn, points);
    set_font_small();
    draw_text_oneline_center_color(&DESK_MY_HOLD_TITLE, &hold_title, HOLD_BORDER);
    set_font_normal();

    // clear
    if true {
        draw_text_oneline_center_color(&DESK_MY_HOLD_BOTTOM, "CLEAR!", CARD_CLEAR_BOREDER);
        return;
    }

    // holds
    // let slices = border.center_cut_width(
    //     Fixed(CARD_WIDTH * 7.0 + DESK_MY_HOLD_GAP_WIDTH * 6.0)
    // ).cut_height([
    //     Fixed(30.0),
    //     Fixed(CARD_HEIGHT),
    //     Fixed(DESK_MY_HOLD_GAP_HEIGHT),
    //     Fixed(CARD_HEIGHT),
    // ]);
    // let mut line1 = slices[1].clone();
    // let mut line2 = slices[3].clone();
    // line1.w = CARD_WIDTH;
    // line1.h = CARD_HEIGHT;
    // line2.w = CARD_WIDTH;
    // line2.h = CARD_HEIGHT;
    // warn!("{:?}", line1);
    // warn!("{:?}", line2);

    let mut line1 = DESK_MY_HOLD_LINE1_START.clone();
    let mut line2 = DESK_MY_HOLD_LINE2_START;
    let (h1, h2) = if hn > 7 {
        let h2 = my_holds.split_off(7);
        (my_holds, h2)
    } else {
        (my_holds, vec![])
    };

    for c in h1 {
        ui_card(&line1, Some(c), MYCARD_BORDER_DIM);
        line1.x += line1.w + DESK_MY_HOLD_GAP_WIDTH;
    }
    for c in h2 {
        ui_card(&line2, Some(c), MYCARD_BORDER_DIM);
        line2.x += line2.w + DESK_MY_HOLD_GAP_WIDTH;
    }
}
